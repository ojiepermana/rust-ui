use super::{STORIES, Section, Showcase};
use gpui_kit::test::{TestAppContextExt, TestWindowExt};
use gpui_kit::{
    AppContext, Bounds, Entity, TestAppContext, WindowBounds, WindowHandle, WindowOptions, point,
    px, size,
};
use std::time::Duration;
use ui_kit::theme::ThemeService;

fn open(
    cx: &mut TestAppContext,
    section: Section,
    story: usize,
) -> (WindowHandle<gpui_kit::base::Root>, Entity<Showcase>) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        ThemeService::init(cx).unwrap();
        super::init(cx);
        cx.set_reduce_motion(true);
        let (handle, view) = gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    Default::default(),
                    size(px(1200.), px(900.)),
                ))),
                ..Default::default()
            },
            cx,
            |window, cx| {
                cx.new(|cx| {
                    let mut view = Showcase::new(window, cx);
                    view.section = section;
                    view.story = story;
                    view
                })
            },
        )
        .unwrap();
        (handle.downcast().unwrap(), view)
    })
}
#[gpui_kit::test]
fn every_declared_story_renders_the_real_production_preview(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, Section::Components, 0);
    for (story, definition) in STORIES.iter().enumerate() {
        cx.update_window(handle.into(), |_, window, cx| {
            view.update(cx, |view, cx| {
                view.story = story;
                cx.notify();
            });
            window.render_frame(cx);
            assert!(window.find("live-preview").visible(), "{}", definition.name);
        })
        .unwrap();
    }
}
#[gpui_kit::test]
async fn dialog_opens_dismisses_with_escape_and_restores_focus(cx: &mut TestAppContext) {
    let (handle, _) = open(cx, Section::Components, 8);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        focus_by_tab(window, cx, "open-dialog");
        window.press("enter", cx);
    })
    .unwrap();
    cx.wait_for(handle.into(), Duration::from_secs(1), |window, _| {
        window
            .try_find("dialog")
            .is_some_and(|dialog| dialog.visible())
    })
    .await;
    cx.update_window(handle.into(), |_, window, cx| {
        assert!(window.find("dialog").visible());
        window.press("escape", cx);
    })
    .unwrap();
    cx.wait_for(handle.into(), Duration::from_secs(1), |window, _| {
        window.try_find("dialog").is_none()
    })
    .await;
    cx.update_window(handle.into(), |_, window, _| {
        assert_eq!(window.find("open-dialog").focused(), Some(true));
    })
    .unwrap();
}
#[gpui_kit::test]
fn checkbox_uses_keyboard_focus_and_native_checked_state(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, Section::Components, 2);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        window.click("demo-checkbox", cx);
        assert!(!view.read(cx).checked);
        assert_eq!(window.find("demo-checkbox").checked(), Some(false));
    })
    .unwrap();
    cx.run_until_parked();
    cx.update_window(handle.into(), |_, window, cx| {
        focus_by_tab(window, cx, "demo-checkbox");
        window.press("space", cx);
        assert!(view.read(cx).checked);
        assert_eq!(window.find("demo-checkbox").focused(), Some(true));
    })
    .unwrap();
}
#[gpui_kit::test]
async fn layout_selection_and_native_panel_drag_change_geometry(cx: &mut TestAppContext) {
    let (handle, view) = open(cx, Section::Layouts, 0);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        window.click("layout-4", cx);
    })
    .unwrap();
    cx.wait_for(handle.into(), Duration::from_secs(1), |window, _| {
        window
            .try_find("panel-Left panel")
            .is_some_and(|panel| panel.visible())
    })
    .await;
    cx.update_window(handle.into(), |_, window, cx| {
        let area = window.find("panel-Left panel").bounds();
        let before = area.size.width;
        let from = point(area.right(), area.top() + area.size.height / 2.);
        window.drag(from, from + point(px(60.), px(0.)), cx);
        assert!(window.find("panel-Left panel").bounds().size.width > before + px(30.));
        window.click("layout-5", cx);
        window.within("workspace-tabs").click(1usize, cx);
        assert_eq!(view.read(cx).tab, 1);
    })
    .unwrap();
}

fn focus_by_tab(window: &mut gpui_kit::Window, cx: &mut gpui_kit::App, id: &str) {
    for _ in 0..64 {
        window.press("tab", cx);
        if window.find(id.to_owned()).focused() == Some(true) {
            return;
        }
    }
    panic!("{id} is not reachable by Tab");
}

#[gpui_kit::test]
async fn coalesced_native_divider_drag_applies_the_first_move(cx: &mut TestAppContext) {
    use gpui_kit::{InputEvent, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent};
    let (handle, _) = open(cx, Section::Layouts, 0);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        window.click("layout-4", cx);
    })
    .unwrap();
    cx.wait_for(handle.into(), Duration::from_secs(1), |window, _| {
        window
            .try_find("panel-Left panel")
            .is_some_and(|panel| panel.visible())
    })
    .await;
    cx.update_window(handle.into(), |_, window, cx| {
        let before = window.find("panel-Left panel").bounds();
        let from = point(before.right(), before.top() + before.size.height / 2.);
        let to = from + point(px(104.), px(0.));
        // Replay the observed macOS down/move/up sequence without the eight
        // intermediate frames/moves inserted by TestWindowExt::drag.
        for event in [
            MouseDownEvent {
                position: from,
                button: MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }
            .to_platform_input(),
            MouseMoveEvent {
                position: to,
                pressed_button: Some(MouseButton::Left),
                ..Default::default()
            }
            .to_platform_input(),
            MouseUpEvent {
                position: to,
                button: MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }
            .to_platform_input(),
        ] {
            window.dispatch_event(event, cx);
        }
        window.render_frame(cx);
        let after = window.find("panel-Left panel").bounds();
        assert!((after.size.width - before.size.width - px(104.)).abs() < px(1.));
        window.dispatch_event(
            MouseMoveEvent {
                position: to + point(px(50.), px(0.)),
                ..Default::default()
            }
            .to_platform_input(),
            cx,
        );
        window.render_frame(cx);
        assert_eq!(
            window.find("panel-Left panel").bounds().size.width,
            after.size.width
        );
    })
    .unwrap();
}

#[gpui_kit::test]
async fn coalesced_vertical_divider_preserves_constraints_and_release(cx: &mut TestAppContext) {
    use gpui_kit::test::TestSupportExt;
    use gpui_kit::{
        App, Context, InputEvent, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent,
        MouseUpEvent, Render, Window, div, prelude::*,
    };
    use ui_kit::components::{resizable_panel, v_resizable};
    struct VerticalSplit;
    impl Render for VerticalSplit {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().size_full().child(
                v_resizable("vertical-regression")
                    .child(
                        resizable_panel()
                            .size(px(160.))
                            .size_range(px(100.)..px(250.))
                            .child(div().id("upper-pane").test_support().size_full()),
                    )
                    .child(
                        resizable_panel()
                            .size_range(px(100.)..gpui_kit::Pixels::MAX)
                            .child(div().size_full()),
                    ),
            )
        }
    }
    let handle = cx.update(|cx: &mut App| {
        gpui_kit::init(cx);
        let (handle, _) = gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    Default::default(),
                    size(px(500.), px(500.)),
                ))),
                ..Default::default()
            },
            cx,
            |_, cx| cx.new(|_| VerticalSplit),
        )
        .unwrap();
        handle
    });
    cx.wait_for(handle, Duration::from_secs(1), |window, _| {
        window
            .try_find("upper-pane")
            .is_some_and(|pane| pane.visible())
    })
    .await;
    cx.update_window(handle, |_, window, cx| {
        for (delta, expected) in [(70., 230.), (1000., 250.), (-1000., 100.)] {
            let before = window.find("upper-pane").bounds();
            let from = point(before.left() + before.size.width / 2., before.bottom());
            let to = from + point(px(0.), px(delta));
            for event in [
                MouseDownEvent {
                    position: from,
                    button: MouseButton::Left,
                    click_count: 1,
                    ..Default::default()
                }
                .to_platform_input(),
                MouseMoveEvent {
                    position: to,
                    pressed_button: Some(MouseButton::Left),
                    ..Default::default()
                }
                .to_platform_input(),
                MouseUpEvent {
                    position: to,
                    button: MouseButton::Left,
                    click_count: 1,
                    ..Default::default()
                }
                .to_platform_input(),
            ] {
                window.dispatch_event(event, cx);
            }
            window.render_frame(cx);
            let height = window.find("upper-pane").bounds().size.height;
            assert!((height - px(expected)).abs() < px(1.));
            window.dispatch_event(
                MouseMoveEvent {
                    position: point(px(200.), px(300.)),
                    ..Default::default()
                }
                .to_platform_input(),
                cx,
            );
            window.render_frame(cx);
            assert_eq!(window.find("upper-pane").bounds().size.height, height);
        }
    })
    .unwrap();
}

// covers: AC-1, AC-3, AC-4..8, AC-15, AC-17. Real story, no fixture renderer.
#[gpui_kit::test]
fn navigation_gallery_uses_each_pattern_and_application_owned_destination(cx: &mut TestAppContext) {
    use ui_kit::navigation::navigation_element_id;
    let story = STORIES
        .iter()
        .position(|story| story.name == "Navigation")
        .unwrap();
    let (handle, view) = open(cx, Section::Components, story);
    for pattern in 0..5 {
        cx.update_window(handle.into(), |_, window, cx| {
            view.read(cx)
                .navigation_demo
                .clone()
                .update(cx, |demo, cx| demo.set_pattern(pattern, window, cx));
            window.render_frame(cx);
            if pattern == 4 {
                let trigger = navigation_element_id(
                    &super::navigation_demo::id("showcase-navigation"),
                    None,
                    "trigger",
                );
                window.click(trigger, cx);
            }
        })
        .unwrap();
        cx.run_until_parked();
        cx.update_window(handle.into(), |_, window, cx| {
            window.render_frame(cx);
            let inbox = navigation_element_id(
                &super::navigation_demo::id("showcase-navigation"),
                Some(&super::navigation_demo::id("inbox")),
                "item",
            );
            let button = window.find(inbox.clone());
            assert!(button.visible(), "pattern {pattern}");
            assert_eq!(button.label(), Some("Inbox, 120 unread messages"));
            assert_eq!(button.selected(), Some(true));
            assert_eq!(button.checked(), None, "selection must not fake a toggle");
            window.click(inbox, cx);
        })
        .unwrap();
        cx.run_until_parked();
        cx.update_window(handle.into(), |_, window, cx| {
            window.render_frame(cx);
            let demo = view.read(cx).navigation_demo.read(cx);
            assert_eq!(demo.activations, pattern + 1, "one request per click");
            assert_eq!(demo.selected.as_str(), "inbox");
        })
        .unwrap();
    }
}

// covers: AC-3, AC-9, AC-13, AC-15, AC-16. Retained ID and shell focus handoff.
#[gpui_kit::test]
fn navigation_model_update_retains_focus_and_leaving_story_restores_shell_shortcuts(
    cx: &mut TestAppContext,
) {
    use ui_kit::navigation::navigation_element_id;
    let story = STORIES
        .iter()
        .position(|story| story.name == "Navigation")
        .unwrap();
    let (handle, view) = open(cx, Section::Components, story);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        let navigation = view.read(cx).navigation_demo.read(cx).navigation.clone();
        let inbox = super::navigation_demo::id("inbox");
        navigation
            .read(cx)
            .item_focus_handle(&inbox)
            .unwrap()
            .focus(window, cx);
        navigation.update(cx, |state, cx| {
            state
                .set_model(
                    super::navigation_demo::model(false, true, false),
                    window,
                    cx,
                )
                .unwrap()
        });
        window.render_frame(cx);
        let control = navigation_element_id(
            &super::navigation_demo::id("showcase-navigation"),
            Some(&inbox),
            "item",
        );
        assert_eq!(window.find(control).focused(), Some(true));
        view.update(cx, |view, cx| view.navigate(Section::Overview, window, cx));
        window.render_frame(cx);
        assert!(view.read(cx).focus.is_focused(window));
        window.press("ctrl-tab", cx);
        assert_eq!(view.read(cx).section(), Section::Components);
    })
    .unwrap();
}

// covers: AC-5, AC-12, AC-16, AC-17. Existing layout contract remains valid.
#[gpui_kit::test]
fn navigation_gallery_renders_theme_density_and_rem_matrix(cx: &mut TestAppContext) {
    use ui_kit::core::{Density, ThemeMode};
    let story = STORIES
        .iter()
        .position(|story| story.name == "Navigation")
        .unwrap();
    let (handle, view) = open(cx, Section::Components, story);
    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        for density in [Density::Comfortable, Density::Compact] {
            for rem in [16., 20., 24.] {
                for reduced in [false, true] {
                    cx.update_window(handle.into(), |_, window, cx| {
                        ThemeService::select(mode, Some(window), cx).unwrap();
                        ThemeService::set_density(density, cx);
                        cx.set_reduce_motion(reduced);
                        window.set_rem_size(px(rem));
                        for pattern in 0..5 {
                            view.read(cx)
                                .navigation_demo
                                .clone()
                                .update(cx, |demo, cx| demo.set_pattern(pattern, window, cx));
                            window.render_frame(cx);
                            assert!(window.find("navigation-pattern-4").visible());
                            let inbox_id = super::navigation_demo::id("inbox");
                            let target = ui_kit::navigation::navigation_element_id(
                                &super::navigation_demo::id("showcase-navigation"),
                                if pattern == 4 { None } else { Some(&inbox_id) },
                                if pattern == 4 { "trigger" } else { "item" },
                            );
                            let control = window.find(target);
                            assert!(control.visible(), "native control should be visible in pattern {pattern}");
                            assert!(control.bounds().size.height >= px(20.), "native control height should remain intrinsic in pattern {pattern}");
                            let region = window.find(ui_kit::navigation::navigation_element_id(
                                &super::navigation_demo::id("showcase-navigation"), None, "root")).bounds();
                            assert!(control.bounds().top() >= region.top() - px(0.5)
                                && control.bounds().bottom() <= region.bottom() + px(0.5),
                                "control must fit fully inside navigation height in pattern {pattern}");
                        }
                    })
                    .unwrap();
                }
            }
        }
    }
}
