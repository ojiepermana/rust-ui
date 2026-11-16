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
