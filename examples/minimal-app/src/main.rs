//! Small consumer proving local facade installation and native component rendering.
use ui_kit::{
    gpui::{self, WindowOptions},
    prelude::*,
};
struct Minimal {
    _appearance: Subscription,
}
impl Render for Minimal {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        AppShell::new(
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .gap_4()
                .child("A native Rust application")
                .child(
                    Button::new("hello")
                        .primary()
                        .label("Say hello")
                        .on_click(|_, window, cx| {
                            window.push_notification("Hello from ui-kit", cx)
                        }),
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
            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            if let Err(error) = gpui::open_window(WindowOptions::default(), cx, |window, cx| {
                cx.new(|cx| Minimal {
                    _appearance: cx.observe_window_appearance(window, |_, window, cx| {
                        if let Err(error) = ThemeService::sync_system(window, cx) {
                            eprintln!("{error}");
                        }
                    }),
                })
            }) {
                eprintln!("{error}");
                cx.quit();
            }
            cx.activate(true);
        });
}
