//! Consumer composed entirely from the public layout and component facade.
use ui_kit::{
    gpui::{self, WindowOptions},
    prelude::*,
};
struct Dashboard {
    selected: usize,
    _appearance: Subscription,
}
impl Render for Dashboard {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let names = ["Projects", "Members", "Tasks"];
        AppShell::new(
            content_container("dashboard-content")
                .child(div().text_2xl().child(names[self.selected]))
                .child(
                    dashboard().children(
                        [("Projects", "12"), ("Members", "48"), ("Tasks", "216")]
                            .into_iter()
                            .map(|(label, value)| {
                                v_flex()
                                    .w_48()
                                    .p_5()
                                    .gap_2()
                                    .rounded(cx.theme().radius)
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .child(label)
                                    .child(div().text_3xl().child(value))
                            }),
                    ),
                ),
        )
        .header(header().child("Workspace dashboard"))
        .sidebar(
            sidebar_navigation().child(navigation_group("WORKSPACE").children(
                names.into_iter().enumerate().map(|(index, name)| {
                    Button::new(SharedString::from(format!("destination-{index}")))
                        .ghost()
                        .label(name)
                        .selected(self.selected == index)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.selected = index;
                            cx.notify();
                        }))
                }),
            )),
        )
        .footer(
            div()
                .p_2()
                .text_xs()
                .child("Example data · local library source"),
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
                cx.new(|cx| Dashboard {
                    selected: 0,
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
