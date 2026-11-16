//! Contract tests use only the public facade, like an external application.
use ui_kit::gpui;
use ui_kit::{
    components::{
        checkbox::Checkbox,
        input::{Input, InputState},
    },
    prelude::*,
};
#[test]
fn public_crates_compose_without_showcase_types() {
    let icons = IconRegistry::built_in().unwrap();
    assert!(icons.lookup(&IconName::Search.id()).is_ok());
    let themes = ThemeRegistry::built_in().unwrap();
    assert_eq!(themes.themes().count(), 2);
    let _shell = AppShell::new(Button::new("save").primary().label("Save"))
        .sidebar(Checkbox::new("enabled").label("Enabled"));
    let _split =
        SplitWorkspace::master_detail("split", div().child("Master"), div().child("Detail"));
    let _constructor: fn(&gpui::Entity<InputState>) -> Input = Input::new;
}

// covers: AC-1, AC-2, AC-5, AC-10, AC-15. Import only the public facade.
#[test]
fn navigation_model_and_patterns_are_public_without_application_types() {
    use std::sync::Arc;
    use ui_kit::navigation::*;
    let inbox = NavigationId::new("inbox").unwrap();
    let model = Arc::new(
        NavigationModel::new(vec![
            NavigationItem::new(inbox.clone(), "Inbox")
                .icon(IconName::Book.id())
                .badge(NavigationBadge::count(120).with_description("unread messages"))
                .into(),
        ])
        .unwrap(),
    );
    assert!(model.item(&inbox).is_some());
    let _presentation = NavigationPresentation::default().with_selected(Some(inbox));
    let _metrics = NavigationMetrics::default()
        .with_sidebar_width(18.)
        .unwrap();
    let _patterns = [
        NavigationKind::Sidebar,
        NavigationKind::Minidock,
        NavigationKind::Dockbar(DockbarMode::Inline),
        NavigationKind::Dockbar(DockbarMode::Overlay),
        NavigationKind::Navbar,
        NavigationKind::Flyout,
    ];
    let _constructor: fn(&Entity<NavigationState>) -> Navigation = Navigation::new;
    assert!(NavigationId::new(" ").is_err());
}
