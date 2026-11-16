//! Application-owned navigation story, including every controlled intent.
use std::sync::Arc;
use ui_kit::{
    components::Side,
    gpui::{AnyElement, FocusHandle},
    prelude::*,
};

pub(crate) fn id(value: &'static str) -> NavigationId {
    NavigationId::new(value).expect("static nonempty demo ID")
}

pub(crate) fn model(flat: bool, updated: bool, empty: bool) -> Arc<NavigationModel> {
    if empty {
        return Arc::new(NavigationModel::new(vec![]).expect("empty model is valid"));
    }
    let inbox = NavigationItem::new(id("inbox"), "Inbox")
        .icon(IconName::Book.id())
        .badge(
            NavigationBadge::count(if updated { 7 } else { 120 })
                .with_description("unread messages"),
        );
    let projects = NavigationItem::new(id("projects"), "Projects")
        .icon(IconName::Layout.id())
        .badge(NavigationBadge::text("New").expect("static badge"));
    let tasks = NavigationItem::new(id("tasks"), "Assigned to me").icon(IconName::Check.id());
    let activity = NavigationItem::new(id("activity"), "Activity log").icon(IconName::Search.id());
    let disabled = NavigationItem::new(id("billing"), "Billing").disabled(true);
    let settings = NavigationItem::new(id("settings"), "Settings")
        .icon(IconName::Settings.id())
        .badge(NavigationBadge::count(0));
    let mut nodes = if flat {
        vec![
            inbox.into(),
            projects.into(),
            tasks.into(),
            activity.into(),
            disabled.into(),
        ]
    } else {
        vec![
            NavigationGroup::new(
                id("workspace"),
                "WORKSPACE",
                vec![
                    inbox.into(),
                    NavigationBranch::new(
                        id("work"),
                        "Work",
                        vec![
                            projects.into(),
                            NavigationBranch::new(id("personal"), "Personal", vec![tasks.into()])
                                .into(),
                            activity.into(),
                        ],
                    )
                    .icon(IconName::Layout.id())
                    .badge(NavigationBadge::count(12))
                    .into(),
                    NavigationBranch::new(
                        id("restricted"),
                        "Administration",
                        vec![disabled.into()],
                    )
                    .icon(IconName::Settings.id())
                    .disabled(true)
                    .into(),
                ],
            )
            .into(),
        ]
    };
    if updated {
        nodes.insert(
            0,
            NavigationItem::new(id("reports"), "Reports and workspace insights")
                .icon(IconId::new("demo", "missing").expect("valid missing icon ID"))
                .tooltip("No registered icon: the compact label is the fallback")
                .into(),
        );
    }
    nodes.push(NavigationNode::spacer(id("bottom-space")));
    nodes.push(NavigationNode::separator(id("settings-divider")));
    nodes.push(settings.into());
    Arc::new(NavigationModel::new(nodes).expect("static demo model is valid"))
}

pub(crate) struct NavigationDemo {
    pub(crate) navigation: Entity<NavigationState>,
    pub(crate) pattern: usize,
    pub(crate) selected: NavigationId,
    pub(crate) activations: usize,
    updated: bool,
    empty: bool,
    overlay: bool,
    right: bool,
    status: SharedString,
    _subscriptions: Vec<Subscription>,
}
impl NavigationDemo {
    pub(crate) fn new(fallback: FocusHandle, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let navigation =
            cx.new(|cx| NavigationState::new(id("showcase-navigation"), fallback, window, cx));
        navigation.update(cx, |state, cx| {
            state
                .configure(
                    NavigationKind::Sidebar,
                    model(false, false, false),
                    NavigationPresentation::default()
                        .with_selected(Some(id("inbox")))
                        .with_expanded([id("work"), id("personal")]),
                    window,
                    cx,
                )
                .expect("initial demo configuration");
            state.set_icons(
                Arc::new(IconRegistry::built_in().expect("bundled icons")),
                cx,
            );
        });
        let events = cx.subscribe_in(&navigation, window, |this, _, event, window, cx| {
            let mut presentation = this.navigation.read(cx).presentation().clone();
            match event {
                NavigationEvent::Activate { id } => {
                    this.selected = id.clone();
                    this.activations += 1;
                    presentation = presentation
                        .with_selected(Some(id.clone()))
                        .with_open_panel(None)
                        .flyout_open(false);
                    this.status =
                        format!("Application accepted destination: {}", id.as_str()).into();
                }
                NavigationEvent::ExpandedChange { id, expanded } => {
                    let mut values = presentation.expanded().clone();
                    if *expanded {
                        values.insert(id.clone());
                    } else {
                        values.remove(id);
                    }
                    presentation = presentation.with_expanded(values);
                }
                NavigationEvent::PanelChange { id } => {
                    presentation = presentation.with_open_panel(id.clone());
                }
                NavigationEvent::CollapsedChange { collapsed } => {
                    presentation = presentation.collapsed(*collapsed).with_open_panel(None);
                }
                NavigationEvent::FlyoutOpenChange { open } => {
                    presentation = presentation.flyout_open(*open);
                }
                _ => return,
            }
            this.navigation.update(cx, |state, cx| {
                state.set_presentation(presentation, window, cx)
            });
            cx.notify();
        });
        let observe = cx.observe(&navigation, |_, _, cx| cx.notify());
        Self {
            navigation,
            pattern: 0,
            selected: id("inbox"),
            activations: 0,
            updated: false,
            empty: false,
            overlay: false,
            right: false,
            status: "Select a destination to update the application content".into(),
            _subscriptions: vec![events, observe],
        }
    }
    pub(crate) fn restore_focus(&self, window: &mut Window, cx: &mut App) {
        self.navigation
            .update(cx, |state, cx| state.restore_focus(window, cx));
    }
    pub(crate) fn set_pattern(
        &mut self,
        pattern: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let kind = match pattern {
            1 => NavigationKind::Dockbar(if self.overlay {
                DockbarMode::Overlay
            } else {
                DockbarMode::Inline
            }),
            2 => NavigationKind::Minidock,
            3 => NavigationKind::Navbar,
            4 => NavigationKind::Flyout,
            _ => NavigationKind::Sidebar,
        };
        let presentation = self
            .navigation
            .read(cx)
            .presentation()
            .clone()
            .with_open_panel(None)
            .flyout_open(false);
        let result = self.navigation.update(cx, |state, cx| {
            state.configure(
                kind,
                model(pattern == 2, self.updated, self.empty),
                presentation,
                window,
                cx,
            )
        });
        if let Err(error) = result {
            self.status = error.to_string().into();
        } else {
            self.pattern = pattern;
        }
        cx.notify();
    }
    fn refresh_model(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let model = model(self.pattern == 2, self.updated, self.empty);
        if let Err(error) = self
            .navigation
            .update(cx, |state, cx| state.set_model(model, window, cx))
        {
            self.status = error.to_string().into();
        }
        cx.notify();
    }
    fn destination(&self, cx: &App) -> AnyElement {
        let (title, description) = match self.selected.as_str() {
            "projects" => (
                "Projects",
                "12 active projects, shared with your workspace.",
            ),
            "tasks" => ("Assigned to me", "Your next steps across active projects."),
            "activity" => (
                "Activity log",
                "Recent changes from people in your workspace.",
            ),
            "settings" => (
                "Workspace settings",
                "Manage the preferences owned by this application.",
            ),
            "reports" => (
                "Reports and workspace insights",
                "A destination inserted while the navigation is mounted.",
            ),
            _ => ("Inbox", "Unread updates and decisions from your workspace."),
        };
        v_flex()
            .id("navigation-destination")
            .min_h_0()
            .overflow_y_scroll()
            .flex_1()
            .min_w_0()
            .p_4()
            .gap_3()
            .child(div().text_xl().font_semibold().child(title))
            .child(
                div()
                    .text_color(cx.theme().muted_foreground)
                    .child(description),
            )
            .child(div().h_1().bg(cx.theme().border))
            .children(
                [
                    (
                        "Design review",
                        "The navigation contract is ready for feedback.",
                    ),
                    (
                        "Workspace update",
                        "New projects and tasks stay keyed to their IDs.",
                    ),
                    (
                        "Application state",
                        "Selection and content change in the event handler.",
                    ),
                ]
                .into_iter()
                .map(|(title, description)| {
                    v_flex()
                        .gap_1()
                        .py_2()
                        .child(div().font_medium().child(title))
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().muted_foreground)
                                .child(description),
                        )
                }),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(format!(
                        "Accepted activations: {} · selected: {}",
                        self.activations,
                        self.selected.as_str()
                    )),
            )
            .into_any_element()
    }
}
impl Render for NavigationDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let controls = h_flex().flex_wrap().gap_1().children(
            ["Sidebar", "Dockbar", "Minidock", "Navbar", "Flyout"]
                .into_iter()
                .enumerate()
                .map(|(index, label)| {
                    Button::new(SharedString::from(format!("navigation-pattern-{index}")))
                        .small()
                        .outline()
                        .label(label)
                        .selected(self.pattern == index)
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.set_pattern(index, window, cx)
                        }))
                }),
        );
        let mut navigation = Navigation::new(&self.navigation)
            .accessibility_label("Workspace navigation")
            .trigger_label("Workspace menu")
            .side(if self.right { Side::Right } else { Side::Left })
            .empty(
                if self.pattern == 1
                    || self.pattern == 2
                    || (self.pattern == 0 && self.navigation.read(cx).presentation().is_collapsed())
                {
                    div().p_1().text_xs().child("Empty").into_any_element()
                } else {
                    div()
                        .p_3()
                        .text_sm()
                        .child("No destinations. Turn off Empty to restore your workspace.")
                        .into_any_element()
                },
            );
        if self.pattern != 2
            && !(self.pattern == 0 && self.navigation.read(cx).presentation().is_collapsed())
            && self.pattern != 1
        {
            navigation = navigation
                .header(div().p_2().font_semibold().child("Workspace"))
                .footer(div().p_2().text_xs().child("Local demo data"));
        }
        let surface = if self.pattern >= 3 {
            v_flex()
                .w_full()
                .h_96()
                .child(navigation)
                .child(self.destination(cx))
                .into_any_element()
        } else if self.right && self.pattern == 1 {
            h_flex()
                .items_stretch()
                .w_full()
                .h_96()
                .child(self.destination(cx))
                .child(navigation)
                .into_any_element()
        } else {
            h_flex()
                .items_stretch()
                .w_full()
                .h_96()
                .child(navigation)
                .child(self.destination(cx))
                .into_any_element()
        };
        v_flex()
            .id("navigation-demo")
            .w_full()
            .gap_3()
            .child(controls)
            .child(
                h_flex()
                    .flex_wrap()
                    .gap_2()
                    .child(
                        Button::new("navigation-empty")
                            .small()
                            .ghost()
                            .label("Empty")
                            .selected(self.empty)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.empty = !this.empty;
                                this.refresh_model(window, cx);
                            })),
                    )
                    .child(
                        Button::new("navigation-update")
                            .small()
                            .ghost()
                            .label("Update data")
                            .selected(self.updated)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.updated = !this.updated;
                                this.refresh_model(window, cx);
                            })),
                    )
                    .child(
                        Button::new("navigation-overlay")
                            .small()
                            .ghost()
                            .label("Overlay panel")
                            .selected(self.overlay)
                            .disabled(self.pattern != 1)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.overlay = !this.overlay;
                                this.set_pattern(this.pattern, window, cx);
                            })),
                    )
                    .child(
                        Button::new("navigation-right")
                            .small()
                            .ghost()
                            .label("Right dock")
                            .selected(self.right)
                            .disabled(self.pattern != 1)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.restore_focus(window, cx);
                                this.right = !this.right;
                                cx.notify();
                            })),
                    ),
            )
            .child(
                div()
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded(cx.theme().radius)
                    .overflow_hidden()
                    .child(surface),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(self.status.clone()),
            )
    }
}
