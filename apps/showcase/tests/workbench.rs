//! Headless integration tests driving the actual production workbench.
use gpui_kit::test::TestWindowExt;
use gpui_kit::{
    self as gpui, AppContext, Bounds, Entity, TestAppContext, WindowBounds, WindowHandle,
    WindowOptions, px, size,
};
use showcase::{Section, Showcase};
use ui_kit::{components::ActiveTheme, theme::ThemeService};
fn open(cx: &mut TestAppContext) -> (WindowHandle<gpui::base::Root>, Entity<Showcase>) {
    cx.update(|cx| {
        gpui::init(cx);
        ThemeService::init(cx).unwrap();
        showcase::init(cx);
        let (handle, view) = gpui::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    Default::default(),
                    size(px(1200.), px(900.)),
                ))),
                ..Default::default()
            },
            cx,
            |window, cx| cx.new(|cx| Showcase::new(window, cx)),
        )
        .unwrap();
        (handle.downcast().unwrap(), view)
    })
}
#[gpui_kit::test]
fn playground_accepts_clicks_and_rejects_disabled_activation(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        window.click("nav-playground", cx);
        window.click("preview-button", cx);
        assert_eq!(view.read(cx).clicks(), 1);
        assert_eq!(
            window.find("activation-count").label(),
            Some("Activations: 1")
        );
        window.click("property-disabled", cx);
        assert_eq!(window.find("preview-button").disabled(), Some(true));
        window.click("preview-button", cx);
        assert_eq!(view.read(cx).clicks(), 1);
        window.click("property-disabled", cx);
        window.click("playground-label", cx);
        window.press("secondary-a", cx);
        window.input("Publish draft", cx);
        assert_eq!(window.find("preview-button").label(), Some("Publish draft"));
        window.click("preview-button", cx);
        assert_eq!(view.read(cx).clicks(), 2);
    })
    .unwrap();
}
#[gpui_kit::test]
fn native_theme_controls_update_component_and_base_tokens(cx: &mut TestAppContext) {
    let (handle, _) = open(cx);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        window.click("theme-dark", cx);
        assert!(cx.theme().is_dark());
        assert_eq!(cx.theme().button_primary, cx.theme().primary);
        assert_eq!(cx.theme().tokens.button_primary.color, cx.theme().primary);
        assert_eq!(ThemeService::global(cx).selected(), "dark");
        assert_eq!(
            cx.theme().primary,
            ThemeService::global(cx).tokens().colors.primary
        );
        assert_eq!(
            gpui::base::Theme::global(cx).tokens.colors.primary,
            cx.theme().primary
        );
        window.click("theme-light", cx);
        assert!(!cx.theme().is_dark());
        window.click("theme-system", cx);
        assert_eq!(
            ThemeService::global(cx).mode(),
            ui_kit::core::ThemeMode::System
        );
    })
    .unwrap();
}
#[gpui_kit::test]
fn icon_search_and_clipboard_use_the_production_registry(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        window.click("nav-icons", cx);
        window.click("icon-search", cx);
        window.input("search", cx);
        assert_eq!(view.read(cx).icon_matches(cx), 1);
        window.click("copy-ui:search", cx);
        let copied = cx
            .read_from_clipboard()
            .and_then(|item| item.text())
            .unwrap();
        assert!(copied.contains("IconId::new(\"ui\", \"search\")"));
        window.click("icon-search", cx);
        window.press("secondary-a", cx);
        window.input("no-such-symbol", cx);
        assert_eq!(view.read(cx).icon_matches(cx), 0);
    })
    .unwrap();
}
#[gpui_kit::test]
fn keyboard_navigation_survives_narrow_window_and_scale_changes(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        window.press("ctrl-tab", cx);
        assert_eq!(view.read(cx).section(), Section::Components);
    })
    .unwrap();
    cx.simulate_window_resize(handle.into(), size(px(680.), px(700.)));
    cx.simulate_window_scale_factor_change(handle.into(), 2.);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        assert!(window.try_find("nav-components").is_none());
        window.press("ctrl-tab", cx);
        assert_eq!(view.read(cx).section(), Section::Themes);
        assert!(window.find("theme-dark").visible());
    })
    .unwrap();
}
#[gpui_kit::test]
fn theme_editor_rejects_invalid_values_before_registering(cx: &mut TestAppContext) {
    let (handle, _) = open(cx);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        window.click("nav-themes", cx);
        let before = ThemeService::global(cx).registry().themes().count();
        window.click("custom-primary", cx);
        window.press("secondary-a", cx);
        window.input("bad-color", cx);
        window.click("register-theme", cx);
        assert_eq!(ThemeService::global(cx).registry().themes().count(), before);
        assert!(
            window
                .find("app-status")
                .label()
                .unwrap()
                .contains("expected #RRGGBB")
        );
        window.click("custom-primary", cx);
        window.press("secondary-a", cx);
        window.input("#006677", cx);
        window.click("register-theme", cx);
        assert_eq!(ThemeService::global(cx).selected(), "custom-1");
        assert_eq!(
            ThemeService::global(cx).registry().themes().count(),
            before + 1
        );
    })
    .unwrap();
}

#[gpui_kit::test]
fn section_shortcuts_survive_unmounting_the_focused_input(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        window.click("nav-playground", cx);
        window.click("playground-label", cx);
        window.input("Edited", cx);
        assert_eq!(window.find("playground-label").focused(), Some(true));
        window.click("nav-themes", cx);
        window.click("toggle-navigation", cx);
        window.press("ctrl-tab", cx);
        assert_eq!(view.read(cx).section(), Section::Layouts);
        window.press("ctrl-shift-tab", cx);
        assert_eq!(view.read(cx).section(), Section::Themes);
    })
    .unwrap();
}

#[gpui_kit::test]
fn every_navigation_destination_renders_including_bundled_documentation(cx: &mut TestAppContext) {
    let (handle, view) = open(cx);
    cx.update_window(handle.into(), |_, window, cx| {
        window.render_frame(cx);
        for section in Section::ALL {
            let id = format!("nav-{}", section.title().to_ascii_lowercase());
            window.click(id, cx);
            assert_eq!(view.read(cx).section(), section);
            assert!(window.find("app-status").visible());
        }
        assert!(window.try_find("copy-install").is_some());
        assert!(window.try_find("open-upstream-docs").is_some());
    })
    .unwrap();
}
