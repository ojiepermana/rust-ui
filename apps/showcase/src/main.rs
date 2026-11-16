//! Launch the native library workbench.
use ui_kit::{
    gpui::{self, AppContext, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size},
    theme::ThemeService,
};
fn main() {
    gpui::application()
        .with_assets(gpui::assets::Assets)
        .run(|cx| {
            gpui::init(cx);
            if let Err(error) = ThemeService::init(cx) {
                eprintln!("Theme initialization failed: {error}");
                cx.quit();
                return;
            }
            showcase::init(cx);
            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1200.), px(820.)),
                    cx,
                ))),
                window_min_size: Some(size(px(640.), px(520.))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Rust UI Kit · Showcase".into()),
                    ..Default::default()
                }),
                ..Default::default()
            };
            if let Err(error) = gpui::open_window(options, cx, |window, cx| {
                cx.new(|cx| showcase::Showcase::new(window, cx))
            }) {
                eprintln!("Could not open Showcase: {error}");
                cx.quit();
            }
            cx.activate(true);
        });
}
