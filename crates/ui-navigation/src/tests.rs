use super::*;
use gpui::{
    AppContext, Context, Entity, FocusHandle, IntoElement, ParentElement, Render, Styled,
    Subscription, TestAppContext, Window, div, rems,
};
use gpui_kit as gpui;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use ui_components::{Root, WindowExt, button::Button};
fn id(s: &str) -> NavigationId {
    NavigationId::new(s.to_owned()).unwrap()
}
fn model() -> Arc<NavigationModel> {
    Arc::new(
        NavigationModel::new(vec![
            NavigationBranch::new(
                id("branch"),
                "Projects",
                vec![NavigationItem::new(id("leaf"), "Project").into()],
            )
            .into(),
            NavigationItem::new(id("home"), "Home").into(),
        ])
        .unwrap(),
    )
}
struct Harness {
    state: Entity<NavigationState>,
    fallback: FocusHandle,
    events: Rc<RefCell<Vec<NavigationEvent>>>,
    accept_flyout: bool,
    vertical: bool,
    external_activations: usize,
    _subscriptions: Vec<Subscription>,
}
impl Harness {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let fallback = cx.focus_handle();
        let state = cx.new(|cx| NavigationState::new(id("test"), fallback.clone(), window, cx));
        state.update(cx, |s, cx| {
            s.configure(
                NavigationKind::Sidebar,
                model(),
                NavigationPresentation::default(),
                window,
                cx,
            )
            .unwrap()
        });
        let events = Rc::new(RefCell::new(vec![]));
        let recorded = events.clone();
        let request = cx.subscribe_in(
            &state,
            window,
            move |this, _, event: &NavigationEvent, window, cx| {
                recorded.borrow_mut().push(event.clone());
                if this.accept_flyout {
                    this.state.update(cx, |state, cx| {
                        let p = match event {
                            NavigationEvent::FlyoutOpenChange { open } => {
                                state.presentation().clone().flyout_open(*open)
                            }
                            NavigationEvent::PanelChange { id } => {
                                state.presentation().clone().with_open_panel(id.clone())
                            }
                            _ => return,
                        };
                        state.set_presentation(p, window, cx);
                    });
                }
            },
        );
        let observe = cx.observe(&state, |_, _, cx| cx.notify());
        Self {
            state,
            fallback,
            events,
            accept_flyout: false,
            vertical: false,
            external_activations: 0,
            _subscriptions: vec![request, observe],
        }
    }
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.vertical {
            return div()
                .flex()
                .flex_col()
                .h(rems(12.))
                .w(rems(20.))
                .child(Navigation::new(&self.state))
                .child(
                    div()
                        .flex_1()
                        .child(div().h(rems(40.)).child("Tall application content")),
                )
                .into_any_element();
        }
        div()
            .flex()
            .h(rems(24.))
            .w(rems(40.))
            .child(
                Button::new("fallback")
                    .label("Content")
                    .track_focus(&self.fallback)
                    .on_click(cx.listener(|this, _, _, _| this.external_activations += 1)),
            )
            .child(Navigation::new(&self.state).h_full())
            .into_any_element()
    }
}
fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        ui_theme::ThemeService::init(cx).unwrap();
        init(cx);
    });
}
#[gpui::test]
fn equivalent_configuration_is_noop_and_invalid_model_swap_is_atomic(cx: &mut TestAppContext) {
    init_test(cx);
    let (root, cx) = cx.add_window_view(|window, cx| {
        let view = cx.new(|cx| Harness::new(window, cx));
        Root::new(view, window, cx)
    });
    let view = cx.update(|_, cx| root.read(cx).view().clone().downcast::<Harness>().unwrap());
    cx.update(|window, cx| {
        let state = view.read(cx).state.clone();
        state.update(cx, |s, cx| {
            let revision = s.revision;
            let focus = s.item_focus_handle(&id("home")).unwrap();
            s.configure(
                s.kind(),
                Arc::new((**s.model()).clone()),
                s.presentation().clone(),
                window,
                cx,
            )
            .unwrap();
            assert_eq!(s.revision, revision);
            assert_eq!(s.item_focus_handle(&id("home")).unwrap(), focus);
            assert!(s.set_kind(NavigationKind::Minidock, window, cx).is_err());
            assert_eq!(s.kind(), NavigationKind::Sidebar);
            assert_eq!(s.revision, revision);
        });
        assert!(view.read(cx).events.borrow().is_empty());
    });
}
#[gpui::test]
fn collapsing_branch_and_removing_focused_item_restore_retained_targets(cx: &mut TestAppContext) {
    init_test(cx);
    let (root, cx) = cx.add_window_view(|window, cx| {
        let view = cx.new(|cx| Harness::new(window, cx));
        Root::new(view, window, cx)
    });
    let view = cx.update(|_, cx| root.read(cx).view().clone().downcast::<Harness>().unwrap());
    cx.update(|window, cx| {
        let state = view.read(cx).state.clone();
        state.update(cx, |s, cx| {
            s.set_presentation(
                NavigationPresentation::default().with_expanded([id("branch")]),
                window,
                cx,
            )
        });
        window.draw(cx).clear(cx);
        state
            .read(cx)
            .item_focus_handle(&id("leaf"))
            .unwrap()
            .focus(window, cx);
        state.update(cx, |s, cx| {
            s.set_presentation(NavigationPresentation::default(), window, cx)
        });
        assert!(
            state
                .read(cx)
                .item_focus_handle(&id("branch"))
                .unwrap()
                .is_focused(window)
        );
        state.update(cx, |s, cx| {
            s.set_model(Arc::new(NavigationModel::default()), window, cx)
                .unwrap()
        });
        assert!(view.read(cx).fallback.is_focused(window));
        window.draw(cx).clear(cx);
    });
}
#[gpui::test]
fn rejected_popup_open_reconciles_without_echo_and_removed_popup_keeps_fallback(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let (root, cx) = cx.add_window_view(|window, cx| {
        let view = cx.new(|cx| Harness::new(window, cx));
        Root::new(view, window, cx)
    });
    let view = cx.update(|_, cx| root.read(cx).view().clone().downcast::<Harness>().unwrap());
    cx.update(|window, cx| {
        let state = view.read(cx).state.clone();
        state.update(cx, |s, cx| {
            s.set_kind(NavigationKind::Navbar, window, cx).unwrap()
        });
        window.draw(cx).clear(cx);
        state
            .read(cx)
            .item_focus_handle(&id("branch"))
            .unwrap()
            .focus(window, cx);
    });
    cx.simulate_keystrokes("enter");
    cx.update(|window, cx| {
        window.draw(cx).clear(cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.draw(cx).clear(cx);
        let state = view.read(cx).state.clone();
        assert!(state.read(cx).presentation().open_panel().is_none());
        assert_eq!(
            view.read(cx).events.borrow().as_slice(),
            &[NavigationEvent::PanelChange {
                id: Some(id("branch"))
            }]
        );
    });
    cx.update(|window, cx| {
        let state = view.read(cx).state.clone();
        state.update(cx, |s, cx| {
            s.set_presentation(
                NavigationPresentation::default().with_open_panel(Some(id("branch"))),
                window,
                cx,
            )
        });
        window.draw(cx).clear(cx);
        state
            .read(cx)
            .item_focus_handle(&id("leaf"))
            .unwrap()
            .focus(window, cx);
        state.update(cx, |s, cx| {
            s.set_model(Arc::new(NavigationModel::default()), window, cx)
                .unwrap()
        });
        assert!(view.read(cx).fallback.is_focused(window));
        window.draw(cx).clear(cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.draw(cx).clear(cx);
        assert!(view.read(cx).fallback.is_focused(window));
    });
}
#[gpui::test]
fn external_open_does_not_take_focus_from_a_higher_dialog(cx: &mut TestAppContext) {
    init_test(cx);
    let (root, cx) = cx.add_window_view(|window, cx| {
        let view = cx.new(|cx| Harness::new(window, cx));
        Root::new(view, window, cx)
    });
    let view = cx.update(|_, cx| root.read(cx).view().clone().downcast::<Harness>().unwrap());
    cx.update(|window, cx| {
        window.draw(cx).clear(cx);
        window.open_dialog(cx, |d, _, _| d.title("Modal").child("Higher modal"));
        window.draw(cx).clear(cx);
        let modal_focus = window.focused(cx).unwrap();
        let state = view.read(cx).state.clone();
        state.update(cx, |s, cx| {
            s.configure(
                NavigationKind::Navbar,
                model(),
                NavigationPresentation::default().with_open_panel(Some(id("branch"))),
                window,
                cx,
            )
            .unwrap()
        });
        window.draw(cx).clear(cx);
        assert!(modal_focus.is_focused(window));
        state.update(cx, |s, cx| {
            s.configure(
                NavigationKind::Flyout,
                model(),
                NavigationPresentation::default().flyout_open(true),
                window,
                cx,
            )
            .unwrap()
        });
        window.draw(cx).clear(cx);
        assert!(modal_focus.is_focused(window));
        assert!(state.read(cx).presentation().is_flyout_open());
    });
}

#[gpui::test]
fn ignored_popup_close_returns_to_snapshot_without_echo(cx: &mut TestAppContext) {
    init_test(cx);
    let (root, cx) = cx.add_window_view(|window, cx| {
        let view = cx.new(|cx| Harness::new(window, cx));
        Root::new(view, window, cx)
    });
    let view = cx.update(|_, cx| root.read(cx).view().clone().downcast::<Harness>().unwrap());
    cx.update(|window, cx| {
        let state = view.read(cx).state.clone();
        state.update(cx, |s, cx| {
            s.configure(
                NavigationKind::Navbar,
                model(),
                NavigationPresentation::default().with_open_panel(Some(id("branch"))),
                window,
                cx,
            )
            .unwrap()
        });
        window.draw(cx).clear(cx);
        state
            .read(cx)
            .item_focus_handle(&id("leaf"))
            .unwrap()
            .focus(window, cx);
    });
    cx.simulate_keystrokes("escape");
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.draw(cx).clear(cx);
        let state = view.read(cx).state.clone();
        assert_eq!(
            state.read(cx).presentation().open_panel(),
            Some(&id("branch"))
        );
        assert_eq!(
            view.read(cx).events.borrow().as_slice(),
            &[NavigationEvent::PanelChange { id: None }]
        );
        assert!(state.read(cx).scopes["panel:branch"].contains_focused(window, cx));
    });
}
#[gpui::test]
fn popup_batch_is_one_request_and_old_projection_callbacks_cannot_close_new_panel(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let (root, cx) = cx.add_window_view(|window, cx| {
        let view = cx.new(|cx| Harness::new(window, cx));
        Root::new(view, window, cx)
    });
    let view = cx.update(|_, cx| root.read(cx).view().clone().downcast::<Harness>().unwrap());
    let old_revision = cx.update(|window, cx| {
        let state = view.read(cx).state.clone();
        let tree = Arc::new(
            NavigationModel::new(vec![
                NavigationBranch::new(
                    id("a"),
                    "A",
                    vec![NavigationItem::new(id("a-leaf"), "A leaf").into()],
                )
                .into(),
                NavigationBranch::new(
                    id("b"),
                    "B",
                    vec![NavigationItem::new(id("b-leaf"), "B leaf").into()],
                )
                .into(),
            ])
            .unwrap(),
        );
        state.update(cx, |s, cx| {
            s.configure(
                NavigationKind::Navbar,
                tree,
                NavigationPresentation::default().with_open_panel(Some(id("a"))),
                window,
                cx,
            )
            .unwrap()
        });
        window.draw(cx).clear(cx);
        let revision = state.read(cx).revision;
        for (origin, open) in [("a", false), ("b", true)] {
            NavigationState::queue_popup(
                &state.downgrade(),
                crate::state::PopupRequest {
                    revision,
                    origin: Some(id(origin)),
                    open,
                    flyout: false,
                },
                window,
                cx,
            );
        }
        revision
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        assert_eq!(
            view.read(cx).events.borrow().as_slice(),
            &[NavigationEvent::PanelChange { id: Some(id("b")) }]
        );
        let state = view.read(cx).state.clone();
        state.update(cx, |s, cx| {
            s.set_presentation(
                NavigationPresentation::default().with_open_panel(Some(id("b"))),
                window,
                cx,
            )
        });
        NavigationState::queue_popup(
            &state.downgrade(),
            crate::state::PopupRequest {
                revision: old_revision,
                origin: Some(id("a")),
                open: false,
                flyout: false,
            },
            window,
            cx,
        );
    });
    cx.run_until_parked();
    cx.update(|_, cx| {
        assert_eq!(view.read(cx).events.borrow().len(), 1);
        assert_eq!(
            view.read(cx).state.read(cx).presentation().open_panel(),
            Some(&id("b"))
        );
    });
}
#[gpui::test]
fn compact_spacer_pushes_last_item_and_navbar_focus_scrolls_to_last_root(cx: &mut TestAppContext) {
    use gpui_kit::test::TestWindowExt;
    init_test(cx);
    let (root, cx) = cx.add_window_view(|window, cx| {
        let view = cx.new(|cx| Harness::new(window, cx));
        Root::new(view, window, cx)
    });
    let view = cx.update(|_, cx| root.read(cx).view().clone().downcast::<Harness>().unwrap());
    cx.update(|window, cx| {
        let state = view.read(cx).state.clone();
        let flat = Arc::new(
            NavigationModel::new(vec![
                NavigationItem::new(id("top"), "Top").into(),
                NavigationNode::spacer(id("spacer")),
                NavigationItem::new(id("bottom"), "Bottom").into(),
            ])
            .unwrap(),
        );
        state.update(cx, |s, cx| {
            s.configure(
                NavigationKind::Minidock,
                flat,
                NavigationPresentation::default(),
                window,
                cx,
            )
            .unwrap()
        });
        window.render_frame(cx);
        let top = window
            .find(navigation_element_id(&id("test"), Some(&id("top")), "item"))
            .bounds();
        let bottom = window
            .find(navigation_element_id(
                &id("test"),
                Some(&id("bottom")),
                "item",
            ))
            .bounds();
        assert!(
            bottom.top() > top.bottom() + window.rem_size() * 8.,
            "spacer must occupy available vertical room: {top:?} {bottom:?}"
        );
        let flat = Arc::new(
            NavigationModel::new(
                (0..30)
                    .map(|n| {
                        NavigationItem::new(
                            id(&format!("root-{n}")),
                            format!("Destination number {n}"),
                        )
                        .into()
                    })
                    .collect(),
            )
            .unwrap(),
        );
        state.update(cx, |s, cx| {
            s.configure(
                NavigationKind::Navbar,
                flat,
                NavigationPresentation::default(),
                window,
                cx,
            )
            .unwrap()
        });
        window.render_frame(cx);
        state
            .read(cx)
            .item_focus_handle(&id("root-29"))
            .unwrap()
            .focus(window, cx);
        window.render_frame(cx);
        window.render_frame(cx);
        assert!(
            state.read(cx).scroll["root"].offset().x < gpui::px(0.),
            "focused last root must scroll horizontally; bounds {:?}, offset {:?}",
            state.read(cx).scroll["root"].bounds(),
            state.read(cx).scroll["root"].offset()
        );
    });
}
#[gpui::test]
fn pointer_flyout_trigger_stays_open_after_application_accepts_request(cx: &mut TestAppContext) {
    use gpui_kit::test::TestWindowExt;
    init_test(cx);
    let (root, cx) = cx.add_window_view(|window, cx| {
        let view = cx.new(|cx| Harness::new(window, cx));
        Root::new(view, window, cx)
    });
    let view = cx.update(|_, cx| root.read(cx).view().clone().downcast::<Harness>().unwrap());
    cx.update(|window, cx| {
        view.update(cx, |view, cx| {
            view.accept_flyout = true;
            view.state.update(cx, |s, cx| {
                s.set_kind(NavigationKind::Flyout, window, cx).unwrap()
            });
        });
        window.render_frame(cx);
        window.click(navigation_element_id(&id("test"), None, "trigger"), cx);
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        let state = view.read(cx).state.clone();
        assert!(state.read(cx).presentation().is_flyout_open());
        assert_eq!(
            view.read(cx).events.borrow().as_slice(),
            &[NavigationEvent::FlyoutOpenChange { open: true }]
        );
        assert!(
            window
                .try_find(navigation_element_id(
                    &id("test"),
                    Some(&id("home")),
                    "item"
                ))
                .is_some_and(|e| e.visible())
        );
    });
}
#[gpui::test]
fn coalesced_native_pointer_flyout_open_is_one_accepted_request(cx: &mut TestAppContext) {
    use gpui_kit::test::TestWindowExt;
    use gpui_kit::{InputEvent, MouseButton, MouseDownEvent, MouseUpEvent};
    init_test(cx);
    let (root, cx) = cx.add_window_view(|window, cx| {
        let view = cx.new(|cx| Harness::new(window, cx));
        Root::new(view, window, cx)
    });
    let view = cx.update(|_, cx| root.read(cx).view().clone().downcast::<Harness>().unwrap());
    cx.update(|window, cx| {
        view.update(cx, |view, cx| {
            view.accept_flyout = true;
            view.state.update(cx, |s, cx| {
                s.set_kind(NavigationKind::Flyout, window, cx).unwrap()
            });
        });
        window.render_frame(cx);
        let position = window
            .find(navigation_element_id(&id("test"), None, "trigger"))
            .bounds()
            .center();
        for input in [
            MouseDownEvent {
                position,
                button: MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }
            .to_platform_input(),
            MouseUpEvent {
                position,
                button: MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }
            .to_platform_input(),
        ] {
            window.dispatch_event(input, cx);
        }
    });
    cx.run_until_parked();
    cx.update(|window, cx| {
        window.render_frame(cx);
        let state = view.read(cx).state.clone();
        assert!(state.read(cx).presentation().is_flyout_open());
        assert_eq!(
            view.read(cx).events.borrow().as_slice(),
            &[NavigationEvent::FlyoutOpenChange { open: true }]
        );
        assert!(
            window
                .try_find(navigation_element_id(
                    &id("test"),
                    Some(&id("home")),
                    "item"
                ))
                .is_some_and(|e| e.visible())
        );
    });
}
#[gpui::test]
fn horizontal_surface_keeps_control_height_and_effective_disclosure_metadata(
    cx: &mut TestAppContext,
) {
    use gpui_kit::test::TestWindowExt;
    init_test(cx);
    let (root, cx) = cx.add_window_view(|window, cx| {
        let view = cx.new(|cx| Harness::new(window, cx));
        Root::new(view, window, cx)
    });
    let view = cx.update(|_, cx| root.read(cx).view().clone().downcast::<Harness>().unwrap());
    cx.update(|window, cx| {
        view.update(cx, |v, cx| {
            v.vertical = true;
            v.state.update(cx, |s, cx| {
                s.configure(
                    NavigationKind::Navbar,
                    model(),
                    NavigationPresentation::default().with_expanded([id("branch")]),
                    window,
                    cx,
                )
                .unwrap()
            });
            cx.notify();
        });
        window.render_frame(cx);
        let root_bounds = window
            .find(navigation_element_id(&id("test"), None, "root"))
            .bounds();
        let branch = window.find(navigation_element_id(
            &id("test"),
            Some(&id("branch")),
            "branch",
        ));
        assert_eq!(branch.expanded(), Some(false));
        assert_eq!(branch.selected(), Some(false));
        assert!(
            branch.bounds().top() >= root_bounds.top()
                && branch.bounds().bottom() <= root_bounds.bottom(),
            "full control height must fit root {root_bounds:?}, branch {:?}",
            branch.bounds()
        );
        let state = view.read(cx).state.clone();
        state.update(cx, |s, cx| {
            s.configure(
                NavigationKind::Sidebar,
                model(),
                NavigationPresentation::default()
                    .collapsed(true)
                    .with_expanded([id("branch")]),
                window,
                cx,
            )
            .unwrap()
        });
        window.render_frame(cx);
        assert_eq!(
            window
                .find(navigation_element_id(
                    &id("test"),
                    Some(&id("branch")),
                    "branch"
                ))
                .expanded(),
            Some(false)
        );
    });
}

#[gpui::test]
fn pointer_popup_escape_restores_its_trigger_before_enter_reopens(cx: &mut TestAppContext) {
    use gpui_kit::test::TestWindowExt;
    init_test(cx);
    for kind in [NavigationKind::Flyout, NavigationKind::Navbar] {
        let (root, cx) = cx.add_window_view(|window, cx| {
            let view = cx.new(|cx| Harness::new(window, cx));
            Root::new(view, window, cx)
        });
        let view = cx.update(|_, cx| root.read(cx).view().clone().downcast::<Harness>().unwrap());
        let (trigger_id, trigger_focus) = cx.update(|window, cx| {
            view.update(cx, |view, cx| {
                view.accept_flyout = true;
                view.state
                    .update(cx, |state, cx| state.set_kind(kind, window, cx).unwrap());
            });
            window.render_frame(cx);
            let fallback = view.read(cx).fallback.clone();
            fallback.focus(window, cx);
            assert!(view.read(cx).fallback.is_focused(window));
            let state = view.read(cx).state.read(cx);
            if kind == NavigationKind::Flyout {
                (
                    navigation_element_id(&id("test"), None, "trigger"),
                    state.scopes["trigger"].clone(),
                )
            } else {
                (
                    navigation_element_id(&id("test"), Some(&id("branch")), "branch"),
                    state.item_focus_handle(&id("branch")).unwrap(),
                )
            }
        });
        cx.update(|window, cx| {
            use gpui_kit::{InputEvent, MouseButton, MouseDownEvent, MouseUpEvent};
            let position = window.find(trigger_id).bounds().center();
            for input in [
                MouseDownEvent {
                    position,
                    button: MouseButton::Left,
                    click_count: 1,
                    ..Default::default()
                }
                .to_platform_input(),
                MouseUpEvent {
                    position,
                    button: MouseButton::Left,
                    click_count: 1,
                    ..Default::default()
                }
                .to_platform_input(),
            ] {
                window.dispatch_event(input, cx);
            }
        });
        cx.run_until_parked();
        cx.update(|window, cx| window.render_frame(cx));
        cx.simulate_keystrokes("escape");
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.render_frame(cx);
            assert!(trigger_focus.is_focused(window), "Escape must restore the clicked {kind:?} trigger, not the previously focused application control");
            let state = view.read(cx).state.read(cx);
            assert!(!state.presentation().is_flyout_open());
            assert!(state.presentation().open_panel().is_none());
        });
        cx.simulate_keystrokes("enter");
        cx.run_until_parked();
        cx.update(|window, cx| {
            window.render_frame(cx);
            let harness = view.read(cx);
            assert_eq!(harness.external_activations, 0);
            let expected = if kind == NavigationKind::Flyout {
                vec![
                    NavigationEvent::FlyoutOpenChange { open: true },
                    NavigationEvent::FlyoutOpenChange { open: false },
                    NavigationEvent::FlyoutOpenChange { open: true },
                ]
            } else {
                vec![
                    NavigationEvent::PanelChange {
                        id: Some(id("branch")),
                    },
                    NavigationEvent::PanelChange { id: None },
                    NavigationEvent::PanelChange {
                        id: Some(id("branch")),
                    },
                ]
            };
            assert_eq!(*harness.events.borrow(), expected);
            assert!(
                window
                    .try_find(navigation_element_id(
                        &id("test"),
                        Some(&id("home")),
                        "item"
                    ))
                    .is_some_and(|node| node.visible())
            );
        });
    }
}
