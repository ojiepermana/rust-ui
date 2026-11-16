//! A facade-only consumer with application-owned selection and focus handoff.
use std::sync::Arc;
use ui_kit::{
    gpui::{self, FocusHandle, WindowOptions},
    prelude::*,
};

fn id(value: &'static str) -> NavigationId {
    NavigationId::new(value).expect("static nonempty ID")
}
struct Workspace {
    navigation: Entity<NavigationState>,
    focus: FocusHandle,
    selected: NavigationId,
    sidebar_visible: bool,
    config: ShellConfig,
    _subscriptions: Vec<Subscription>,
}
impl Workspace {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus = cx.focus_handle();
        let navigation =
            cx.new(|cx| NavigationState::new(id("example-navigation"), focus.clone(), window, cx));
        let model = Arc::new(
            NavigationModel::new(vec![
                NavigationItem::new(id("inbox"), "Inbox")
                    .icon(IconName::Book.id())
                    .badge(NavigationBadge::count(120).with_description("unread messages"))
                    .into(),
                NavigationBranch::new(
                    id("work"),
                    "Work",
                    vec![
                        NavigationItem::new(id("projects"), "Projects")
                            .icon(IconName::Layout.id())
                            .into(),
                        NavigationItem::new(id("tasks"), "Tasks")
                            .icon(IconName::Check.id())
                            .into(),
                    ],
                )
                .into(),
                NavigationNode::spacer(id("bottom")),
                NavigationItem::new(id("settings"), "Settings")
                    .icon(IconName::Settings.id())
                    .into(),
            ])
            .expect("static model is valid"),
        );
        navigation.update(cx, |state, cx| {
            state
                .configure(
                    NavigationKind::Sidebar,
                    model,
                    NavigationPresentation::default()
                        .with_selected(Some(id("inbox")))
                        .with_expanded([id("work")]),
                    window,
                    cx,
                )
                .expect("valid initial configuration");
            state.set_icons(
                Arc::new(IconRegistry::built_in().expect("bundled SVG assets")),
                cx,
            );
        });
        let requests = cx.subscribe_in(&navigation, window, |this, _, event, window, cx| {
            let mut presentation = this.navigation.read(cx).presentation().clone();
            match event {
                NavigationEvent::Activate { id } => {
                    this.selected = id.clone();
                    presentation = presentation
                        .with_selected(Some(id.clone()))
                        .with_open_panel(None);
                }
                NavigationEvent::ExpandedChange { id, expanded } => {
                    let mut branches = presentation.expanded().clone();
                    if *expanded {
                        branches.insert(id.clone());
                    } else {
                        branches.remove(id);
                    }
                    presentation = presentation.with_expanded(branches);
                }
                NavigationEvent::CollapsedChange { collapsed } => {
                    presentation = presentation.collapsed(*collapsed)
                }
                NavigationEvent::PanelChange { id } => {
                    presentation = presentation.with_open_panel(id.clone())
                }
                NavigationEvent::FlyoutOpenChange { open } => {
                    presentation = presentation.flyout_open(*open)
                }
                _ => return,
            }
            this.navigation.update(cx, |state, cx| {
                state.set_presentation(presentation, window, cx)
            });
            cx.notify();
        });
        // RenderOnce reads a snapshot. Retain an observer as well as the request subscription.
        let observer = cx.observe(&navigation, |_, _, cx| cx.notify());
        let config = ShellConfig::default();
        let sidebar_visible = f32::from(window.viewport_size().width) >= config.collapse_below();
        let bounds = cx.observe_window_bounds(window, |this, window, cx| {
            let visible = f32::from(window.viewport_size().width) >= this.config.collapse_below();
            if this.sidebar_visible && !visible {
                // Handoff before AppShell removes the sidebar, including focused slot controls.
                this.navigation
                    .update(cx, |state, cx| state.restore_focus(window, cx));
            }
            this.sidebar_visible = visible;
            cx.notify();
        });
        let appearance = cx.observe_window_appearance(window, |_, window, cx| {
            if let Err(error) = ThemeService::sync_system(window, cx) {
                eprintln!("{error}");
            }
        });
        window.focus(&focus, cx);
        Self {
            navigation,
            focus,
            selected: id("inbox"),
            sidebar_visible,
            config,
            _subscriptions: vec![requests, observer, bounds, appearance],
        }
    }
}
impl Render for Workspace {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = content_container("example-content")
            .child(div().text_2xl().font_semibold().child(format!("Destination: {}", self.selected.as_str())))
            .child("This application owns the selected destination and every navigation request.")
            .child(h_flex().gap_2().flex_wrap().children(["inbox", "projects", "tasks", "settings"].into_iter().map(|destination| {
                Button::new(SharedString::from(format!("fallback-{destination}"))).outline().label(destination)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.selected = id(destination);
                        let presentation = this.navigation.read(cx).presentation().clone().with_selected(Some(this.selected.clone()));
                        this.navigation.update(cx, |state, cx| state.set_presentation(presentation, window, cx));
                        cx.notify();
                    }))
            })))
            .child("The content buttons remain available when the shell hides its sidebar. Resize below 800 logical pixels to exercise focus handoff.");
        div().size_full().track_focus(&self.focus).child(
            AppShell::new(content)
                .config(self.config.clone())
                .expect("default shell configuration")
                .header(header().child("Navigation consumer"))
                .sidebar(
                    Navigation::new(&self.navigation)
                        .w_full()
                        .header(
                            Button::new("example-slot-command")
                                .ghost()
                                .label("Workspace"),
                        )
                        .footer(div().p_2().child("Application owned slots")),
                ),
        )
    }
}
fn main() {
    gpui::application()
        .with_assets(gpui::assets::Assets)
        .run(|cx| {
            gpui::init(cx);
            if let Err(error) = ThemeService::init(cx) {
                eprintln!("{error}");
                cx.quit();
                return;
            }
            ui_kit::navigation::init(cx);
            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            if let Err(error) = gpui::open_window(WindowOptions::default(), cx, |window, cx| {
                cx.new(|cx| Workspace::new(window, cx))
            }) {
                eprintln!("{error}");
                cx.quit();
            }
            cx.activate(true);
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Bounds, TestAppContext, WindowBounds, px, size};
    use gpui_kit::test::TestWindowExt;

    // covers: AC-13, AC-15, AC-16. Test the actual facade consumer and AppShell.
    #[gpui_kit::test]
    fn resize_hands_off_focus_from_projected_slot_before_hiding_sidebar(cx: &mut TestAppContext) {
        let (handle, view) = cx.update(|cx| {
            gpui::init(cx);
            ThemeService::init(cx).unwrap();
            ui_kit::navigation::init(cx);
            gpui::open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        Default::default(),
                        size(px(1100.), px(700.)),
                    ))),
                    ..Default::default()
                },
                cx,
                |window, cx| cx.new(|cx| Workspace::new(window, cx)),
            )
            .unwrap()
        });
        cx.update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            for _ in 0..20 {
                if window.find("example-slot-command").focused() == Some(true) {
                    break;
                }
                window.press("tab", cx);
            }
            assert_eq!(window.find("example-slot-command").focused(), Some(true));
        })
        .unwrap();
        cx.simulate_window_resize(handle, size(px(700.), px(700.)));
        cx.update_window(handle, |_, window, cx| {
            window.render_frame(cx);
            assert!(window.try_find("example-slot-command").is_none());
            assert!(view.read(cx).focus.is_focused(window));
            window.click("fallback-projects", cx);
            assert_eq!(view.read(cx).selected.as_str(), "projects");
        })
        .unwrap();
    }
}
