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
