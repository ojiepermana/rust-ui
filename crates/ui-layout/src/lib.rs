//! Reusable application compositions over GPUI and upstream panel primitives.
use gpui_kit::{AnyElement, App, ElementId, IntoElement, RenderOnce, Window, div, prelude::*, px};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ui_components::{
    ActiveTheme, h_flex,
    resizable::{h_resizable, resizable_panel},
    v_flex,
};

/// Invalid shell dimensions or sizing relationships.
#[derive(Debug, Error, PartialEq)]
#[error("invalid shell configuration: {0}")]
pub struct LayoutError(&'static str);

/// Validated, serializable application frame dimensions, in logical pixels.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShellConfig {
    sidebar_width: f32,
    inspector_width: f32,
    collapse_below: f32,
    min_content_width: f32,
}
impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            sidebar_width: 224.,
            inspector_width: 240.,
            collapse_below: 800.,
            min_content_width: 320.,
        }
    }
}
impl ShellConfig {
    /// Set the sidebar width and validate the result.
    pub fn with_sidebar_width(mut self, width: f32) -> Result<Self, LayoutError> {
        self.sidebar_width = width;
        self.validate()?;
        Ok(self)
    }
    /// Sidebar's preferred logical width.
    pub fn sidebar_width(&self) -> f32 {
        self.sidebar_width
    }
    /// Minimum central content width.
    pub fn min_content_width(&self) -> f32 {
        self.min_content_width
    }
    /// Collapse navigation when the window is below this width.
    pub fn collapse_below(&self) -> f32 {
        self.collapse_below
    }
    /// Reject nonfinite, unusable or contradictory dimensions, including deserialized values.
    pub fn validate(&self) -> Result<(), LayoutError> {
        if ![
            self.sidebar_width,
            self.inspector_width,
            self.collapse_below,
            self.min_content_width,
        ]
        .iter()
        .all(|v| v.is_finite())
        {
            return Err(LayoutError("dimensions must be finite"));
        }
        if !(160. ..=400.).contains(&self.sidebar_width)
            || !(160. ..=480.).contains(&self.inspector_width)
        {
            return Err(LayoutError("side panels exceed supported width range"));
        }
        if self.min_content_width < 200.
            || self.collapse_below < self.sidebar_width + self.min_content_width
        {
            return Err(LayoutError(
                "collapse threshold must fit navigation and content",
            ));
        }
        Ok(())
    }
    /// Validate a persisted configuration before it reaches layout.
    pub fn from_json(json: &str) -> Result<Self, String> {
        let value: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        value.validate().map_err(|e| e.to_string())?;
        Ok(value)
    }
}

/// Main frame with independent header, sidebar, toolbar, content, inspector and footer slots.
#[derive(IntoElement)]
pub struct AppShell {
    config: ShellConfig,
    header: Option<AnyElement>,
    sidebar: Option<AnyElement>,
    toolbar: Option<AnyElement>,
    content: AnyElement,
    inspector: Option<AnyElement>,
    footer: Option<AnyElement>,
    collapsed: bool,
}
impl AppShell {
    /// Construct a shell around application content using valid defaults.
    pub fn new(content: impl IntoElement) -> Self {
        Self {
            config: ShellConfig::default(),
            header: None,
            sidebar: None,
            toolbar: None,
            content: content.into_any_element(),
            inspector: None,
            footer: None,
            collapsed: false,
        }
    }
    /// Apply validated shell dimensions.
    pub fn config(mut self, config: ShellConfig) -> Result<Self, LayoutError> {
        config.validate()?;
        self.config = config;
        Ok(self)
    }
    /// Persistent top region.
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }
    /// Primary navigation region.
    pub fn sidebar(mut self, sidebar: impl IntoElement) -> Self {
        self.sidebar = Some(sidebar.into_any_element());
        self
    }
    /// Toolbar immediately above main content.
    pub fn toolbar(mut self, toolbar: impl IntoElement) -> Self {
        self.toolbar = Some(toolbar.into_any_element());
        self
    }
    /// Supplementary detail region. Hidden at narrow widths to protect content.
    pub fn inspector(mut self, inspector: impl IntoElement) -> Self {
        self.inspector = Some(inspector.into_any_element());
        self
    }
    /// Persistent status region.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
    /// Explicit navigation collapse state, owned by the consuming application.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}
impl RenderOnce for AppShell {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let width = f32::from(window.viewport_size().width);
        let show_sidebar = !self.collapsed && width >= self.config.collapse_below;
        let show_inspector = width >= self.config.collapse_below + self.config.inspector_width;
        let border = cx.theme().border;
        v_flex()
            .id("app-shell")
            .size_full()
            .overflow_hidden()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .children(self.header.map(|h| {
                div()
                    .flex_shrink_0()
                    .border_b_1()
                    .border_color(border)
                    .child(h)
            }))
            .child(
                h_flex()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .children(self.sidebar.filter(|_| show_sidebar).map(|s| {
                        div()
                            .w(px(self.config.sidebar_width))
                            .h_full()
                            .flex_shrink_0()
                            .border_r_1()
                            .border_color(border)
                            .child(s)
                    }))
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w_0()
                            .h_full()
                            .children(self.toolbar)
                            .child(self.content),
                    )
                    .children(self.inspector.filter(|_| show_inspector).map(|s| {
                        div()
                            .w(px(self.config.inspector_width))
                            .h_full()
                            .flex_shrink_0()
                            .border_l_1()
                            .border_color(border)
                            .child(s)
                    })),
            )
            .children(self.footer.map(|f| {
                div()
                    .flex_shrink_0()
                    .border_t_1()
                    .border_color(border)
                    .child(f)
            }))
    }
}
/// Scrollable content region. Its ID must be stable and unique in its native scope.
pub fn content_container(id: impl Into<ElementId>) -> gpui_kit::Stateful<gpui_kit::Div> {
    v_flex()
        .id(id)
        .flex_1()
        .min_h_0()
        .w_full()
        .overflow_y_scroll()
        .p_6()
        .gap_5()
}
/// Sidebar composition supporting groups, nested content and arbitrary native navigation controls.
pub fn sidebar_navigation() -> gpui_kit::Div {
    v_flex().p_3().gap_2().size_full()
}
/// Group related navigation controls without inventing selection behavior.
pub fn navigation_group(label: impl Into<gpui_kit::SharedString>) -> gpui_kit::Div {
    v_flex()
        .gap_1()
        .child(div().text_xs().py_2().px_2().child(label.into()))
}
/// Bounded header composition, for native controls and breadcrumbs.
pub fn header() -> gpui_kit::Div {
    h_flex()
        .h_12()
        .px_5()
        .gap_3()
        .items_center()
        .justify_between()
}
/// Responsive wrapping dashboard region, with caller owned cards and data.
pub fn dashboard() -> gpui_kit::Div {
    h_flex().w_full().flex_wrap().gap_4()
}

/// Two or three resizable regions. Upstream owns drag behavior and retained panel sizes.
#[derive(IntoElement)]
pub struct SplitWorkspace {
    id: ElementId,
    main: AnyElement,
    leading: Option<AnyElement>,
    trailing: Option<AnyElement>,
}
impl SplitWorkspace {
    /// Master detail composition, with the master pane on the left.
    pub fn master_detail(
        id: impl Into<ElementId>,
        master: impl IntoElement,
        detail: impl IntoElement,
    ) -> Self {
        Self {
            id: id.into(),
            main: detail.into_any_element(),
            leading: Some(master.into_any_element()),
            trailing: None,
        }
    }
    /// Main workspace plus a trailing inspector.
    pub fn inspector(
        id: impl Into<ElementId>,
        main: impl IntoElement,
        inspector: impl IntoElement,
    ) -> Self {
        Self {
            id: id.into(),
            main: main.into_any_element(),
            leading: None,
            trailing: Some(inspector.into_any_element()),
        }
    }
    /// Add an inspector to a master detail workspace to form three panels.
    pub fn with_inspector(mut self, inspector: impl IntoElement) -> Self {
        self.trailing = Some(inspector.into_any_element());
        self
    }
}
impl RenderOnce for SplitWorkspace {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let mut group = h_resizable(self.id);
        if let Some(leading) = self.leading {
            group = group.child(
                resizable_panel()
                    .size(px(200.))
                    .size_range(px(140.)..px(400.))
                    .child(leading),
            );
        }
        group = group.child(
            resizable_panel()
                .size_range(px(200.)..gpui_kit::Pixels::MAX)
                .child(self.main),
        );
        if let Some(trailing) = self.trailing {
            group = group.child(
                resizable_panel()
                    .size(px(220.))
                    .size_range(px(140.)..px(400.))
                    .child(trailing),
            );
        }
        group
    }
}
/// Compose a caller controlled native TabBar with its selected content.
#[derive(IntoElement)]
pub struct TabbedWorkspace {
    tabs: AnyElement,
    content: AnyElement,
}
impl TabbedWorkspace {
    /// Tabs and selected content remain owned by the application.
    pub fn new(tabs: impl IntoElement, content: impl IntoElement) -> Self {
        Self {
            tabs: tabs.into_any_element(),
            content: content.into_any_element(),
        }
    }
}
impl RenderOnce for TabbedWorkspace {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_h_0()
            .child(self.tabs)
            .child(div().flex_1().min_h_0().child(self.content))
    }
}
#[cfg(test)]
mod tests {
    use super::ShellConfig;
    #[test]
    fn shell_configuration_validates_before_use_or_restore() {
        assert!(ShellConfig::default().validate().is_ok());
        for width in [0., -1., f32::NAN, f32::INFINITY, 900.] {
            assert!(ShellConfig::default().with_sidebar_width(width).is_err());
        }
        let config = ShellConfig::default().with_sidebar_width(280.).unwrap();
        assert_eq!(
            ShellConfig::from_json(&serde_json::to_string(&config).unwrap())
                .unwrap()
                .sidebar_width(),
            280.
        );
        assert!(ShellConfig::from_json(r#"{"sidebar_width":1,"inspector_width":200,"collapse_below":800,"min_content_width":320}"#).is_err());
    }
}
