//! Unified native UI foundation.
//!
//! ```no_run
//! use ui_kit::prelude::*;
//! let action = Button::new("save").primary().label("Save");
//! let shell = AppShell::new(action);
//! ```
//! Initialize [`gpui::init`] and [`theme::ThemeService::init`] before creating
//! views. Use [`gpui::open_window`] for the upstream Root and overlay lifecycle.
/// The one compatible GPUI runtime, platform and asset entry point.
pub use gpui_kit as gpui;
/// Supported upstream component catalog, preserving module paths.
pub use ui_components as components;
/// Shared semantic contracts.
pub use ui_core as core;
/// Typed SVG registration and semantic rendering.
pub use ui_icons as icons;
/// Reusable application compositions.
pub use ui_layout as layout;
/// Controlled native navigation compositions.
pub use ui_navigation as navigation;
/// Application owned theme definitions and upstream integration.
pub use ui_theme as theme;
/// Common imports for applications. Larger upstream APIs remain in `components`.
pub mod prelude {
    pub use gpui_kit::prelude::*;
    pub use gpui_kit::{
        App, AppContext, Context, Entity, IntoElement, Render, SharedString, Subscription, Window,
        div,
    };
    pub use ui_components::{
        ActiveTheme, Disableable, Selectable, Sizable, StyledExt, WindowExt,
        button::{Button, ButtonVariants},
        h_flex, v_flex,
    };
    pub use ui_core::{Density, ThemeMode};
    pub use ui_icons::{IconId, IconName, IconRegistry, IconSize, IconState};
    pub use ui_layout::{
        AppShell, ShellConfig, SplitWorkspace, TabbedWorkspace, content_container, dashboard,
        header, navigation_group, sidebar_navigation,
    };
    pub use ui_navigation::{
        DockbarMode, Navigation, NavigationBadge, NavigationBranch, NavigationError,
        NavigationEvent, NavigationGroup, NavigationId, NavigationItem, NavigationKind,
        NavigationMetrics, NavigationModel, NavigationNode, NavigationPresentation,
        NavigationState,
    };
    pub use ui_theme::{ThemeDefinition, ThemeRegistry, ThemeService, content_gap};
}
