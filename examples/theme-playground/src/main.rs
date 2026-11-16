//! Consumer showing runtime appearance switching through the custom theme service.
use ui_kit::{
    gpui::{self, WindowOptions},
    prelude::*,
};
struct ThemePlayground {
    status: SharedString,
    _appearance: Subscription,
}
impl Render for ThemePlayground {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        AppShell::new(
            content_container("theme-content")
                .gap(content_gap(cx))
                .child(div().text_2xl().child("Appearance"))
                .child(
                    h_flex().gap_3().children(
                        [
                            (ThemeMode::Light, "Light"),
                            (ThemeMode::Dark, "Dark"),
                            (ThemeMode::System, "System"),
                        ]
                        .into_iter()
                        .map(|(mode, label)| {
                            Button::new(label)
                                .outline()
                                .label(label)
                                .selected(ThemeService::global(cx).mode() == mode)
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.status = match ThemeService::select(mode, Some(window), cx)
                                    {
                                        Ok(()) => format!(
                                            "Active: {}",
                                            ThemeService::global(cx).selected()
                                        )
                                        .into(),
                                        Err(e) => e.to_string().into(),
                                    };
                                    cx.notify();
                                }))
                        }),
                    ),
                )
                .child(Button::new("sample").primary().label("Themed component"))
                .child(self.status.clone()),
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
                cx.new(|cx| ThemePlayground {
                    status: "Follow the native appearance or choose a mode.".into(),
                    _appearance: cx.observe_window_appearance(
                        window,
                        |this: &mut ThemePlayground, window, cx| {
                            if let Err(error) = ThemeService::sync_system(window, cx) {
                                this.status = error.to_string().into();
                            }
                            cx.notify();
                        },
                    ),
                })
            }) {
                eprintln!("{error}");
                cx.quit();
            }
            cx.activate(true);
        });
}
