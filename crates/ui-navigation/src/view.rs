use crate::{
    state::{PopupRequest, projection, roots},
    *,
};
use gpui_kit::{base::TestSupportExt, prelude::FluentBuilder, *};
use std::{collections::BTreeMap, sync::Arc};
use ui_components::{
    ActiveTheme, Disableable, Selectable, Side, Sizable, Size, StyledExt, WindowExt,
    badge::Badge,
    button::{Button, ButtonVariants},
    collapsible::Collapsible,
    popover::Popover,
    sidebar::SidebarToggleButton,
    tag::Tag,
};
use ui_icons::{IconRegistry, IconSize, IconState};
use ui_theme::ThemeService;

/// Stateless composition reading a retained [`NavigationState`]. Store a
/// `cx.observe` subscription in its owner to repaint programmatic state updates.
#[derive(IntoElement)]
pub struct Navigation {
    state: Entity<NavigationState>,
    style: StyleRefinement,
    header: Option<AnyElement>,
    footer: Option<AnyElement>,
    empty: Option<AnyElement>,
    label: SharedString,
    trigger: SharedString,
    side: Side,
}
impl Navigation {
    /// Compose the application-retained entity, without creating another state.
    pub fn new(state: &Entity<NavigationState>) -> Self {
        Self {
            state: state.clone(),
            style: StyleRefinement::default(),
            header: None,
            footer: None,
            empty: None,
            label: "Navigation".into(),
            trigger: "Menu".into(),
            side: Side::Left,
        }
    }
    /// Fixed header (prefix on Navbar, inside popup on Flyout).
    pub fn header(mut self, content: impl IntoElement) -> Self {
        self.header = Some(content.into_any_element());
        self
    }
    /// Fixed footer (suffix on Navbar, inside popup on Flyout).
    pub fn footer(mut self, content: impl IntoElement) -> Self {
        self.footer = Some(content.into_any_element());
        self
    }
    /// Empty model content; defaults to a neutral message.
    pub fn empty(mut self, content: impl IntoElement) -> Self {
        self.empty = Some(content.into_any_element());
        self
    }
    /// Native region name.
    pub fn accessibility_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    /// Flyout trigger name (default Menu).
    pub fn trigger_label(mut self, label: impl Into<SharedString>) -> Self {
        self.trigger = label.into();
        self
    }
    /// Dockbar side; default Left.
    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }
}
impl Styled for Navigation {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
// The themed Button exposes InteractiveElement but does not opt into its
// stateful marker. This local composition forwards the same interactivity so
// selected/expanded metadata reaches the original native Button node.
#[derive(IntoElement)]
struct NativeButton {
    button: Button,
    selected: bool,
    expanded: Option<bool>,
}
impl InteractiveElement for NativeButton {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.button.interactivity()
    }
}
impl StatefulInteractiveElement for NativeButton {}
impl Selectable for NativeButton {
    fn selected(mut self, selected: bool) -> Self {
        self.button = self.button.selected(selected);
        self.selected = selected;
        self
    }
    fn is_selected(&self) -> bool {
        self.selected
    }
    fn open(mut self, open: bool) -> Self {
        self.button = self.button.open(open);
        self
    }
    fn is_open(&self) -> bool {
        self.button.is_open()
    }
}
impl NativeButton {
    // Pointer activation of upstream Button intentionally preserves focus. A
    // popup must capture its own trigger as the restoration target before its
    // ancestor Popover handles the same mouse-down; this adds no request path.
    fn focus_on_pointer(self, focus: FocusHandle) -> Self {
        self.on_mouse_down(MouseButton::Left, move |_, window, cx| {
            if !window.has_active_dialog(cx) {
                focus.focus(window, cx);
            }
        })
    }
    fn on_click(mut self, listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.button = self.button.on_click(listener);
        self
    }
    fn w_full(mut self) -> Self {
        self.button = self.button.w_full();
        self
    }
}
impl RenderOnce for NativeButton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let selected = self.selected;
        let expanded = self.expanded;
        self.aria_selected(selected)
            .when_some(expanded, |b, open| b.aria_expanded(open))
            .button
    }
}
#[derive(Clone)]
struct Snapshot {
    entity: WeakEntity<NavigationState>,
    instance: NavigationId,
    kind: NavigationKind,
    model: Arc<NavigationModel>,
    p: NavigationPresentation,
    metrics: NavigationMetrics,
    icons: Option<Arc<IconRegistry>>,
    focus: BTreeMap<NavigationId, FocusHandle>,
    scopes: BTreeMap<SharedString, FocusHandle>,
    scroll: BTreeMap<SharedString, ScrollHandle>,
    revision: u64,
    popup_allowed: bool,
    side: Side,
    size: Size,
}
impl Snapshot {
    fn id(&self, node: Option<&NavigationId>, part: &str) -> SharedString {
        navigation_element_id(&self.instance, node, part)
    }
    fn request(
        &self,
        event: NavigationEvent,
    ) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
        let entity = self.entity.clone();
        move |_, _, cx| {
            let _ = entity.update(cx, |s, cx| s.emit_request(event.clone(), cx));
        }
    }
    fn button(
        &self,
        item: &NavigationItem,
        branch: bool,
        compact: bool,
        disabled: bool,
        cx: &App,
    ) -> NativeButton {
        let selected = !branch && self.p.selected.as_ref() == Some(&item.id);
        let is_panel_trigger = branch
            && (matches!(
                self.kind,
                NavigationKind::Dockbar(_) | NavigationKind::Navbar
            ) || self.kind == NavigationKind::Sidebar && self.p.collapsed)
            && roots(self.model.nodes()).iter().any(|n| n.id() == &item.id);
        let expanded = branch
            && if is_panel_trigger {
                self.popup_allowed
                    && projection(self.kind, &self.model, &self.p).panel.as_ref() == Some(&item.id)
            } else {
                self.p.expanded.contains(&item.id)
            };
        let ancestor = branch
            && self.p.selected.as_ref().is_some_and(|id| {
                self.model.is_effectively_visible(id) && self.model.ancestors(id).contains(&item.id)
            });
        let mut button =
            Button::new(self.id(Some(&item.id), if branch { "branch" } else { "item" }))
                .ghost()
                .with_size(self.size)
                .accessibility_label(item.accessible_label())
                .accessibility_id(self.id(Some(&item.id), if branch { "branch" } else { "item" }))
                .track_focus(&self.focus[&item.id])
                .selected(selected)
                .disabled(disabled || item.disabled)
                .when(ancestor && !disabled && !item.disabled, |b| {
                    b.bg(cx.theme().accent)
                        .text_color(cx.theme().accent_foreground)
                });
        let icon = item.icon.as_ref().and_then(|id| {
            self.icons
                .as_ref()?
                .render(
                    id,
                    IconSize::Medium,
                    if disabled || item.disabled {
                        IconState::Disabled
                    } else if selected {
                        IconState::Active
                    } else {
                        IconState::Default
                    },
                    cx,
                )
                .ok()
        });
        if compact {
            let glyph = icon.map_or_else(
                || {
                    div()
                        .w_4()
                        .h_4()
                        .text_xs()
                        .child(item.fallback_label())
                        .into_any_element()
                },
                |icon| icon.size(rems(1.)).into_any_element(),
            );
            let glyph = match item.badge.as_ref().filter(|b| b.display_text().is_some()) {
                Some(badge) if badge.count_value().is_some() => div()
                    .pr_2()
                    .pt_1()
                    .child(
                        Badge::new()
                            .count(badge.count_value().unwrap_or(0))
                            .max(99)
                            .color(cx.theme().primary)
                            .child(glyph),
                    )
                    .into_any_element(),
                Some(badge) => div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .child(glyph)
                    .child(
                        Tag::secondary()
                            .small()
                            .max_w(rems(self.metrics.rail_width() - 0.5))
                            .child(
                                div()
                                    .min_w_0()
                                    .text_ellipsis()
                                    .child(badge.display_text().unwrap_or_default()),
                            ),
                    )
                    .into_any_element(),
                None => glyph,
            };
            button = button
                .w_full()
                .h_auto()
                .min_h(rems(2.))
                .px_1()
                .py_1()
                .child(glyph)
                .tooltip(
                    item.tooltip
                        .clone()
                        .unwrap_or_else(|| item.accessible_label()),
                );
            return NativeButton {
                button,
                selected,
                expanded: branch.then_some(expanded),
            };
        }
        if let Some(icon) = icon {
            button = button.icon(icon.size(rems(1.)));
        }
        if !compact {
            button = button.child(
                div()
                    .min_w_0()
                    .flex_1()
                    .text_ellipsis()
                    .child(item.label.clone())
                    .when_some(item.subtitle.clone(), |d, subtitle| {
                        d.child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child(subtitle),
                        )
                    }),
            );
            if branch {
                button = button.dropdown_caret(true);
            }
        }
        if let Some(tip) = item
            .tooltip
            .clone()
            .or_else(|| compact.then(|| item.accessible_label()))
        {
            button = button.tooltip(tip);
        }
        if let Some(badge) = &item.badge
            && let Some(text) = badge.display_text()
        {
            button = if let Some(count) = badge.count_value() {
                button.child(
                    div().pr_3().pt_1().child(
                        Badge::new()
                            .count(count)
                            .max(99)
                            .color(cx.theme().primary)
                            .child(div().w_4().h_4()),
                    ),
                )
            } else {
                button.child(Tag::secondary().child(text))
            };
        }
        NativeButton {
            button,
            selected,
            expanded: branch.then_some(expanded),
        }
    }
    fn list(
        &self,
        nodes: &[NavigationNode],
        inherited_disabled: bool,
        depth: usize,
        cx: &App,
    ) -> AnyElement {
        let mut result = div()
            .flex()
            .flex_col()
            .when(depth == 0, |d| d.min_h_full())
            .gap(ThemeService::global(cx).tokens().spacing.xxs);
        let visible: Vec<_> = nodes
            .iter()
            .filter(|n| n.visible() && content_node(n))
            .collect();
        let mut separator = false;
        let mut had_content = false;
        for n in visible {
            if matches!(n, NavigationNode::Separator(_)) {
                separator = had_content;
                continue;
            }
            if separator {
                result = result.child(div().h_px().bg(cx.theme().border).my_1());
                separator = false;
            }
            had_content = true;
            let child = match n {
                NavigationNode::Item(item) => self
                    .button(item, false, false, inherited_disabled, cx)
                    .w_full()
                    .on_click(self.request(NavigationEvent::Activate {
                        id: item.id.clone(),
                    }))
                    .into_any_element(),
                NavigationNode::Branch(branch) => {
                    let expanded = self.p.expanded.contains(&branch.item.id);
                    let button = self
                        .button(&branch.item, true, false, inherited_disabled, cx)
                        .w_full()
                        .on_click(self.request(NavigationEvent::ExpandedChange {
                            id: branch.item.id.clone(),
                            expanded: !expanded,
                        }));
                    Collapsible::new()
                        .open(expanded)
                        .w_full()
                        .child(button)
                        .content(
                            div()
                                .pl(ThemeService::global(cx).tokens().spacing.sm)
                                .child(self.list(
                                    &branch.children,
                                    inherited_disabled || branch.item.disabled,
                                    depth + 1,
                                    cx,
                                )),
                        )
                        .into_any_element()
                }
                NavigationNode::Group(group) => div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .id(self.id(Some(&group.id), "heading"))
                            .role(Role::Heading)
                            .aria_label(group.label.clone())
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(group.label.clone()),
                    )
                    .child(self.list(&group.children, inherited_disabled, depth + 1, cx))
                    .into_any_element(),
                NavigationNode::Spacer(_) => div().flex_1().min_h_2().into_any_element(),
                NavigationNode::Separator(_) => continue,
            };
            result = result.child(child);
        }
        result.into_any_element()
    }
    fn scrolling_list(
        &self,
        nodes: &[NavigationNode],
        key: &str,
        empty: Option<AnyElement>,
        cx: &App,
    ) -> AnyElement {
        let content = if nodes.iter().any(has_content) {
            self.list(nodes, false, 0, cx)
        } else {
            empty.unwrap_or_else(|| {
                div()
                    .text_color(cx.theme().muted_foreground)
                    .child("No navigation items")
                    .into_any_element()
            })
        };
        div()
            .id(self.id(None, key))
            .min_h_0()
            .flex_1()
            .overflow_y_scroll()
            .track_scroll(&self.scroll[key])
            .child(content)
            .into_any_element()
    }
    fn popup_panel(&self, branch: &NavigationBranch, window: &Window, cx: &App) -> AnyElement {
        let key = format!("panel:{}", branch.item.id.as_str());
        let inset = ThemeService::global(cx).tokens().spacing.sm;
        div()
            .id(self.id(Some(&branch.item.id), "panel"))
            .test_support()
            .track_focus(&self.scopes[key.as_str()])
            .flex()
            .flex_col()
            .gap(inset)
            .w(rems(self.metrics.panel_width()))
            .max_w((window.viewport_size().width - inset * 2.).max(px(0.)))
            .max_h((window.viewport_size().height - inset * 2.).max(px(0.)))
            .min_h_0()
            .p(inset)
            .bg(cx.theme().popover)
            .border_1()
            .border_color(cx.theme().border)
            .rounded(ThemeService::global(cx).tokens().radius.md)
            .child(self.panel_header(branch, cx))
            .child(self.scrolling_list(&branch.children, &key, None, cx))
            .into_any_element()
    }
    fn panel_header(&self, branch: &NavigationBranch, cx: &App) -> AnyElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap_2()
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_ellipsis()
                    .child(branch.item.label.clone()),
            )
            .child(
                Button::new(self.id(Some(&branch.item.id), "close"))
                    .ghost()
                    .with_size(self.size)
                    .track_focus(&self.scopes["close"])
                    .label("Close")
                    .accessibility_label(format!("Close {}", branch.item.label))
                    .on_click(self.request(NavigationEvent::PanelChange { id: None })),
            )
            .text_color(cx.theme().foreground)
            .into_any_element()
    }
    fn panel_trigger(
        &self,
        branch: &NavigationBranch,
        compact: bool,
        window: &Window,
        cx: &App,
    ) -> AnyElement {
        let button = self.button(&branch.item, true, compact, false, cx);
        if self.kind == NavigationKind::Dockbar(DockbarMode::Inline) {
            return button
                .on_click(self.request(NavigationEvent::PanelChange {
                    id: if self.p.open_panel.as_ref() == Some(&branch.item.id) {
                        None
                    } else {
                        Some(branch.item.id.clone())
                    },
                }))
                .into_any_element();
        }
        // Disabled disclosures never attach an active Popover trigger wrapper.
        if branch.item.disabled {
            return button.into_any_element();
        }
        let button = button.focus_on_pointer(self.focus[&branch.item.id].clone());
        let entity = self.entity.clone();
        let origin = branch.item.id.clone();
        let revision = self.revision;
        let snapshot = self.clone();
        let branch = branch.clone();
        let _ = window;
        let open = projection(self.kind, &self.model, &self.p).panel.as_ref() == Some(&origin);
        Popover::new(self.id(Some(&origin), "popup"))
            .open(open && self.popup_allowed)
            .appearance(false)
            .anchor(if self.kind == NavigationKind::Navbar {
                Anchor::TopLeft
            } else if self.side == Side::Left {
                Anchor::LeftCenter
            } else {
                Anchor::RightCenter
            })
            .trigger(button)
            .track_focus(&snapshot.scopes[format!("panel:{}", origin.as_str()).as_str()])
            .on_open_change(move |open, window, cx| {
                NavigationState::queue_popup(
                    &entity,
                    PopupRequest {
                        revision,
                        origin: Some(origin.clone()),
                        open: *open,
                        flyout: false,
                    },
                    window,
                    cx,
                )
            })
            .content(move |_, window, cx| snapshot.popup_panel(&branch, window, cx))
            .into_any_element()
    }
    fn rail(&self, horizontal: bool, window: &Window, cx: &App) -> AnyElement {
        let compact = !horizontal;
        let mut rail = div()
            .flex()
            .flex_shrink_0()
            .gap_1()
            .when(!horizontal, |d| d.flex_col().min_h_full());
        let nodes = roots(self.model.nodes());
        let mut had_content = false;
        let mut separator = false;
        for node in nodes {
            if matches!(node, NavigationNode::Separator(_)) {
                separator = had_content;
                continue;
            }
            if separator {
                rail = rail.child(
                    div()
                        .when(horizontal, |d| d.w_px().min_h_4())
                        .when(!horizontal, |d| d.h_px().w_full())
                        .bg(cx.theme().border),
                );
                separator = false;
            }
            had_content = true;
            let child = match node {
                NavigationNode::Item(i) => self
                    .button(i, false, compact, false, cx)
                    .on_click(self.request(NavigationEvent::Activate { id: i.id.clone() }))
                    .into_any_element(),
                NavigationNode::Branch(b) => self.panel_trigger(b, compact, window, cx),
                NavigationNode::Separator(_) => div()
                    .when(horizontal, |d| d.w_px().min_h_4())
                    .when(!horizontal, |d| d.h_px().w_full())
                    .bg(cx.theme().border)
                    .into_any_element(),
                NavigationNode::Spacer(_) => div().flex_1().min_h_2().into_any_element(),
                NavigationNode::Group(_) => continue,
            };
            let focus = node.control().and_then(|i| self.focus.get(&i.id)).cloned();
            let scroll = self.scroll["root"].clone();
            rail = rail.child(
                div()
                    .relative()
                    .when(matches!(node, NavigationNode::Spacer(_)), |d| d.flex_1())
                    .when(!matches!(node, NavigationNode::Spacer(_)), |d| {
                        d.flex_shrink_0()
                    })
                    .child(child)
                    .when(horizontal && focus.is_some(), |d| {
                        d.child(
                            canvas(
                                move |bounds, window, _| {
                                    if focus.as_ref().is_some_and(|f| f.is_focused(window)) {
                                        let viewport = scroll.bounds();
                                        let mut offset = scroll.offset();
                                        let delta = if bounds.left() < viewport.left() {
                                            viewport.left() - bounds.left()
                                        } else if bounds.right() > viewport.right() {
                                            viewport.right() - bounds.right()
                                        } else {
                                            px(0.)
                                        };
                                        if delta != px(0.) {
                                            offset.x += delta;
                                            scroll.set_offset(offset);
                                            window.refresh();
                                        }
                                    }
                                },
                                |_, _, _, _| {},
                            )
                            .absolute()
                            .size_full(),
                        )
                    }),
            );
        }
        rail.into_any_element()
    }
}
fn has_content(node: &NavigationNode) -> bool {
    node.visible()
        && match node {
            NavigationNode::Item(_) | NavigationNode::Branch(_) => true,
            NavigationNode::Group(g) => g.children.iter().any(has_content),
            _ => false,
        }
}
fn content_node(node: &NavigationNode) -> bool {
    match node {
        NavigationNode::Group(g) => g.children.iter().any(has_content),
        _ => true,
    }
}
impl RenderOnce for Navigation {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.read(cx);
        let snapshot = Snapshot {
            entity: self.state.downgrade(),
            instance: state.instance.clone(),
            kind: state.kind,
            model: state.model.clone(),
            p: state.presentation.clone(),
            metrics: state.metrics,
            icons: state.icons.clone(),
            focus: state.focus.clone(),
            scopes: state.scopes.clone(),
            scroll: state.scroll.clone(),
            revision: state.revision,
            popup_allowed: !window.has_active_dialog(cx),
            side: self.side,
            size: if ThemeService::global(cx).density() == ui_theme::Density::Compact {
                Size::Small
            } else {
                Size::Medium
            },
        };
        let root_id = snapshot.id(None, "root");
        let padding = ThemeService::global(cx).tokens().spacing.xs;
        let mut root = div()
            .id(root_id)
            .test_support()
            .role(Role::Navigation)
            .aria_label(self.label)
            .track_focus(&snapshot.scopes["root"])
            .flex()
            .min_h_0()
            .min_w_0()
            .text_color(cx.theme().foreground)
            .bg(ThemeService::global(cx).tokens().colors.surface)
            .gap(padding)
            .p(padding);
        match snapshot.kind {
            NavigationKind::Flyout => {
                root = root.flex_shrink_0();
                let entity = snapshot.entity.clone();
                let revision = snapshot.revision;
                let open = snapshot.p.flyout_open && snapshot.popup_allowed;
                let trigger = Button::new(snapshot.id(None, "trigger"))
                    .ghost()
                    .with_size(snapshot.size)
                    .label(self.trigger.clone())
                    .accessibility_label(self.trigger)
                    .track_focus(&snapshot.scopes["trigger"]);
                let trigger = NativeButton {
                    button: trigger,
                    selected: false,
                    expanded: Some(open),
                }
                .focus_on_pointer(snapshot.scopes["trigger"].clone());
                let inset = ThemeService::global(cx).tokens().spacing.sm;
                let slots = std::rc::Rc::new(std::cell::RefCell::new((
                    self.header,
                    self.footer,
                    self.empty,
                )));
                let popup_snapshot = snapshot.clone();
                let content = move |window: &Window, cx: &App| {
                    let (header, footer, empty) = std::mem::take(&mut *slots.borrow_mut());
                    let snapshot = &popup_snapshot;
                    div()
                        .track_focus(&snapshot.scopes["flyout"])
                        .flex()
                        .flex_col()
                        .min_h_0()
                        .w(rems(snapshot.metrics.flyout_width()))
                        .max_w((window.viewport_size().width - inset * 2.).max(px(0.)))
                        .max_h((window.viewport_size().height - inset * 2.).max(px(0.)))
                        .gap(padding)
                        .p(inset)
                        .bg(cx.theme().popover)
                        .border_1()
                        .border_color(cx.theme().border)
                        .rounded(ThemeService::global(cx).tokens().radius.md)
                        .children(header)
                        .child(
                            Button::new(snapshot.id(None, "close"))
                                .ghost()
                                .label("Close menu")
                                .track_focus(&snapshot.scopes["close"])
                                .on_click(
                                    snapshot
                                        .request(NavigationEvent::FlyoutOpenChange { open: false }),
                                ),
                        )
                        .child(snapshot.scrolling_list(snapshot.model.nodes(), "root", empty, cx))
                        .children(footer)
                };
                root = root.child(
                    Popover::new(snapshot.id(None, "popup"))
                        .appearance(false)
                        .open(open)
                        .track_focus(&snapshot.scopes["flyout"])
                        .trigger(trigger)
                        .on_open_change(move |open, window, cx| {
                            NavigationState::queue_popup(
                                &entity,
                                PopupRequest {
                                    revision,
                                    origin: None,
                                    open: *open,
                                    flyout: true,
                                },
                                window,
                                cx,
                            )
                        })
                        .content(move |_, window, cx| content(window, cx)),
                );
            }
            NavigationKind::Navbar => {
                root = root
                    .w_full()
                    .flex_shrink_0()
                    .overflow_x_scroll()
                    .track_scroll(&snapshot.scroll["root"])
                    .items_center()
                    .children(self.header)
                    .child(if snapshot.model.nodes().iter().any(has_content) {
                        snapshot.rail(true, window, cx)
                    } else {
                        self.empty.unwrap_or_else(|| {
                            div().child("No navigation items").into_any_element()
                        })
                    })
                    .children(self.footer);
            }
            _ => {
                let compact = snapshot.kind != NavigationKind::Sidebar || snapshot.p.collapsed;
                let mut rail = div()
                    .flex()
                    .flex_col()
                    .min_h_0()
                    .gap(padding)
                    .w(rems(if compact {
                        snapshot.metrics.rail_width()
                    } else {
                        snapshot.metrics.sidebar_width()
                    }))
                    .children(self.header);
                if snapshot.kind == NavigationKind::Sidebar {
                    let entity = snapshot.entity.clone();
                    let collapsed = snapshot.p.collapsed;
                    rail = rail.child(
                        div()
                            .id(snapshot.id(None, "toggle"))
                            .track_focus(&snapshot.scopes["toggle"])
                            .child(
                                SidebarToggleButton::new()
                                    .collapsed(collapsed)
                                    .side(self.side)
                                    .accessibility_label(if collapsed {
                                        "Expand navigation"
                                    } else {
                                        "Collapse navigation"
                                    })
                                    .on_click(move |_, _, cx| {
                                        let _ = entity.update(cx, |s, cx| {
                                            s.emit_request(
                                                NavigationEvent::CollapsedChange {
                                                    collapsed: !collapsed,
                                                },
                                                cx,
                                            )
                                        });
                                    }),
                            ),
                    );
                }
                rail = rail
                    .child(
                        if compact && snapshot.model.nodes().iter().any(has_content) {
                            div()
                                .id(snapshot.id(None, "items"))
                                .min_h_0()
                                .flex_1()
                                .overflow_y_scroll()
                                .track_scroll(&snapshot.scroll["root"])
                                .child(snapshot.rail(false, window, cx))
                                .into_any_element()
                        } else {
                            snapshot.scrolling_list(snapshot.model.nodes(), "root", self.empty, cx)
                        },
                    )
                    .children(self.footer);
                let panel = if snapshot.kind == NavigationKind::Dockbar(DockbarMode::Inline) {
                    projection(snapshot.kind, &snapshot.model, &snapshot.p)
                        .panel
                        .and_then(|id| match snapshot.model.item(&id) {
                            Some(NavigationNode::Branch(b)) => Some((id, b.clone())),
                            _ => None,
                        })
                } else {
                    None
                };
                let panel = panel.map(|(_id, b)| {
                    let entity = snapshot.entity.clone();
                    div()
                        .key_context("NavigationPanel")
                        .on_action(move |_: &CloseNavigationPanel, _, cx| {
                            let _ = entity.update(cx, |s, cx| {
                                s.emit_request(NavigationEvent::PanelChange { id: None }, cx)
                            });
                        })
                        .child(snapshot.popup_panel(&b, window, cx))
                });
                if self.side == Side::Right {
                    root = root.children(panel).child(rail);
                } else {
                    root = root.child(rail).children(panel);
                }
            }
        }
        root.refine_style(&self.style)
    }
}
