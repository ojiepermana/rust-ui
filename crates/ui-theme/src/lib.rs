//! Validated design configuration projected onto GPUI Kit's native theme.
//!
//! Configuration is authoritative. The retained resolved snapshot preserves
//! spacing and typography scales that upstream legacy theme fields cannot hold.
use gpui_kit::component::theme::{
    SemanticThemeConfig, SemanticThemeTokens, Theme, ThemeMode as KitMode,
};
use gpui_kit::{App, Global, Hsla, Window, WindowAppearance};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;
pub use ui_core::{Density, ThemeMode};

/// Theme validation, registration or lookup failure.
#[derive(Debug, Error)]
pub enum ThemeError {
    /// Serialized theme is not valid JSON for the supported schema.
    #[error("invalid theme JSON: {0}")]
    Json(#[from] serde_json::Error),
    /// A token violates the application contract.
    #[error("invalid theme token: {0}")]
    Invalid(String),
    /// Names are unique within a registry.
    #[error("theme already registered: {0}")]
    Duplicate(String),
    /// Requested theme does not exist.
    #[error("theme not found: {0}")]
    Missing(String),
}

/// Serializable theme configuration. Create with [`Self::from_json`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeDefinition {
    name: String,
    dark: bool,
    tokens: SemanticThemeConfig,
    success: String,
    warning: String,
    density: Density,
    reduced_motion: bool,
}

impl ThemeDefinition {
    /// Deserialize and validate every supplied token before use.
    pub fn from_json(json: &str) -> Result<Self, ThemeError> {
        let theme: Self = serde_json::from_str(json)?;
        theme.validate()?;
        Ok(theme)
    }
    /// Serialize a validated definition for application configuration.
    pub fn to_json(&self) -> Result<String, ThemeError> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    /// Stable registry name.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Whether this theme has dark appearance.
    pub fn is_dark(&self) -> bool {
        self.dark
    }
    /// Authoritative semantic configuration accepted by GPUI Kit.
    pub fn tokens(&self) -> &SemanticThemeConfig {
        &self.tokens
    }
    /// Default density for this theme.
    pub fn density(&self) -> Density {
        self.density
    }
    /// Return a named copy with a validated primary color and focus ring.
    pub fn with_primary(
        mut self,
        name: impl Into<String>,
        color: &str,
    ) -> Result<Self, ThemeError> {
        parse_color(color)?;
        self.name = name.into();
        self.tokens.colors.primary = Some(color.to_owned().into());
        self.tokens.colors.ring = Some(color.to_owned().into());
        self.validate()?;
        Ok(self)
    }
    /// Validate names, colors, finite scales, fonts and typography values.
    pub fn validate(&self) -> Result<(), ThemeError> {
        if self.name.trim().is_empty() || self.name.len() > 80 {
            return Err(ThemeError::Invalid(
                "name must contain 1 to 80 characters".into(),
            ));
        }
        parse_color(&self.success)?;
        parse_color(&self.warning)?;
        let value = serde_json::to_value(&self.tokens)?;
        if let Some(colors) = value["colors"].as_object() {
            for (key, value) in colors {
                if let Some(color) = value.as_str() {
                    parse_color(color).map_err(|_| {
                        ThemeError::Invalid(format!("colors.{key} must be #RRGGBB"))
                    })?;
                }
            }
        }
        validate_numbers(&value, "tokens")?;
        let t = &self.tokens;
        validate_scale(
            &[
                t.spacing.xxs,
                t.spacing.xs,
                t.spacing.sm,
                t.spacing.md,
                t.spacing.lg,
                t.spacing.xl,
                t.spacing.xxl,
            ],
            "spacing",
        )?;
        validate_scale(
            &[
                t.radius.none,
                t.radius.sm,
                t.radius.md,
                t.radius.lg,
                t.radius.xl,
                t.radius.full,
            ],
            "radius",
        )?;
        for family in [&t.typography.sans, &t.typography.mono]
            .into_iter()
            .flatten()
        {
            if family.trim().is_empty() {
                return Err(ThemeError::Invalid("font family must not be empty".into()));
            }
        }
        for style in [
            &t.typography.xs,
            &t.typography.sm,
            &t.typography.md,
            &t.typography.lg,
            &t.typography.xl,
            &t.typography.mono_md,
        ] {
            if style.size.is_some_and(|v| v <= 0.) || style.line_height.is_some_and(|v| v <= 0.) {
                return Err(ThemeError::Invalid(
                    "typography size and line height must be positive".into(),
                ));
            }
        }
        Ok(())
    }
}

fn validate_numbers(value: &serde_json::Value, path: &str) -> Result<(), ThemeError> {
    match value {
        serde_json::Value::Number(n) => {
            if !n
                .as_f64()
                .is_some_and(|n| n.is_finite() && (0.0..=10000.0).contains(&n))
            {
                return Err(ThemeError::Invalid(format!(
                    "{path} must be finite, nonnegative and at most 10000"
                )));
            }
        }
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                validate_numbers(v, &format!("{path}.{k}"))?;
            }
        }
        serde_json::Value::Array(values) => {
            for v in values {
                validate_numbers(v, path)?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn validate_scale(values: &[Option<f32>], name: &str) -> Result<(), ThemeError> {
    let mut previous = 0.;
    for value in values.iter().flatten() {
        if !value.is_finite() || *value < previous || *value > 10000. {
            return Err(ThemeError::Invalid(format!(
                "{name} scale must be finite and nondecreasing"
            )));
        }
        previous = *value;
    }
    Ok(())
}
/// Parse the supported portable color format, #RRGGBB.
pub fn parse_color(color: &str) -> Result<Hsla, ThemeError> {
    if color.len() != 7
        || !color.starts_with('#')
        || !color.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit)
    {
        return Err(ThemeError::Invalid(format!("{color}: expected #RRGGBB")));
    }
    let value =
        u32::from_str_radix(&color[1..], 16).map_err(|_| ThemeError::Invalid(color.into()))?;
    Ok(gpui_kit::rgb(value).into())
}

/// Application owned theme definitions; duplicate names are rejected atomically.
#[derive(Debug, Clone, Default)]
pub struct ThemeRegistry {
    themes: BTreeMap<String, ThemeDefinition>,
}
impl ThemeRegistry {
    /// Registry populated from the packaged authoritative token files.
    pub fn built_in() -> Result<Self, ThemeError> {
        let mut registry = Self::default();
        for json in [
            include_str!("../themes/light.json"),
            include_str!("../themes/dark.json"),
        ] {
            registry.register(ThemeDefinition::from_json(json)?)?;
        }
        Ok(registry)
    }
    /// Register a new validated theme without overwriting an existing one.
    pub fn register(&mut self, theme: ThemeDefinition) -> Result<(), ThemeError> {
        theme.validate()?;
        if self.themes.contains_key(theme.name()) {
            return Err(ThemeError::Duplicate(theme.name));
        }
        self.themes.insert(theme.name.clone(), theme);
        Ok(())
    }
    /// Resolve a name or return a useful missing theme error.
    pub fn get(&self, name: &str) -> Result<&ThemeDefinition, ThemeError> {
        self.themes
            .get(name)
            .ok_or_else(|| ThemeError::Missing(name.into()))
    }
    /// Registered definitions in deterministic name order.
    pub fn themes(&self) -> impl Iterator<Item = &ThemeDefinition> {
        self.themes.values()
    }
}

/// Current design snapshot and selection; GPUI Kit still owns the global renderer theme.
#[derive(Clone)]
pub struct ThemeService {
    registry: ThemeRegistry,
    mode: ThemeMode,
    selected: String,
    active: SemanticThemeTokens,
    density: Density,
}
impl Global for ThemeService {}
impl ThemeService {
    /// Initialize after `gpui_kit::init`, before building themed views.
    pub fn init(cx: &mut App) -> Result<(), ThemeError> {
        cx.set_global(Self {
            registry: ThemeRegistry::built_in()?,
            mode: ThemeMode::System,
            selected: "light".into(),
            active: Theme::global(cx).semantic_tokens(),
            density: Density::Comfortable,
        });
        Self::select(ThemeMode::System, None, cx)
    }
    /// Read the active application design state.
    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }
    /// Read the reusable theme registry.
    pub fn registry(&self) -> &ThemeRegistry {
        &self.registry
    }
    /// Requested appearance mode; system remains system after a native event.
    pub fn mode(&self) -> ThemeMode {
        self.mode
    }
    /// Currently resolved definition name.
    pub fn selected(&self) -> &str {
        &self.selected
    }
    /// Full resolved tokens, including application owned scales and elevations.
    pub fn tokens(&self) -> &SemanticThemeTokens {
        &self.active
    }
    /// Current content density.
    pub fn density(&self) -> Density {
        self.density
    }
    /// Register a validated application theme.
    pub fn register(theme: ThemeDefinition, cx: &mut App) -> Result<(), ThemeError> {
        cx.global_mut::<Self>().registry.register(theme)
    }
    /// Change appearance and repaint every window through upstream `Theme::update`.
    pub fn select(
        mode: ThemeMode,
        window: Option<&Window>,
        cx: &mut App,
    ) -> Result<(), ThemeError> {
        let dark = match mode {
            ThemeMode::Dark => true,
            ThemeMode::Light => false,
            ThemeMode::System => matches!(
                window.map_or_else(|| cx.window_appearance(), Window::appearance),
                WindowAppearance::Dark | WindowAppearance::VibrantDark
            ),
        };
        Self::apply(if dark { "dark" } else { "light" }, mode, cx)
    }
    /// Apply a registered custom definition. It resolves to the definition's explicit mode.
    pub fn select_named(name: &str, cx: &mut App) -> Result<(), ThemeError> {
        let dark = Self::global(cx).registry.get(name)?.dark;
        Self::apply(
            name,
            if dark {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
            cx,
        )
    }
    /// Reapply system mode from a native appearance notification, without polling.
    pub fn sync_system(window: &Window, cx: &mut App) -> Result<(), ThemeError> {
        if Self::global(cx).mode == ThemeMode::System {
            Self::select(ThemeMode::System, Some(window), cx)?;
        }
        Ok(())
    }
    /// Set content density and refresh windows. Components opt into their upstream size APIs.
    pub fn set_density(density: Density, cx: &mut App) {
        cx.global_mut::<Self>().density = density;
        cx.refresh_windows();
    }
    fn apply(name: &str, mode: ThemeMode, cx: &mut App) -> Result<(), ThemeError> {
        let definition = Self::global(cx).registry.get(name)?.clone();
        definition.validate()?;
        let success = parse_color(&definition.success)?;
        let warning = parse_color(&definition.warning)?;
        Theme::change(
            if definition.dark {
                KitMode::Dark
            } else {
                KitMode::Light
            },
            None,
            cx,
        );
        cx.set_reduce_motion(definition.reduced_motion);
        if !definition.reduced_motion {
            gpui_kit::base::apply_system_reduce_motion(cx);
        }
        let snapshot = Theme::update(cx, |theme| {
            theme.motion = Default::default();
            let tokens = theme.apply_semantic_config(&definition.tokens);
            theme.success = success;
            theme.warning = warning;
            project_component_colors(theme, &tokens);
            if definition.reduced_motion {
                theme.motion.duration_fast = std::time::Duration::ZERO;
                theme.motion.duration_normal = std::time::Duration::ZERO;
                theme.motion.duration_slow = std::time::Duration::ZERO;
                theme.motion.distance_short = gpui_kit::rems(0.);
                theme.motion.distance_medium = gpui_kit::rems(0.);
            }
            tokens
        });
        let service = cx.global_mut::<Self>();
        service.selected = name.into();
        service.mode = mode;
        service.active = snapshot;
        service.density = definition.density;
        Ok(())
    }
}

// GPUI Kit intentionally keeps component aliases separate from semantic roles.
// Project them here from the same resolved source so defaults cannot retain an
// unrelated palette. Theme::update reconciles the matching background tokens.
fn project_component_colors(theme: &mut Theme, tokens: &SemanticThemeTokens) {
    let c = tokens.colors;
    let dark = theme.is_dark();
    let shade = |mut color: Hsla, amount: f32| {
        color.l = (color.l + if dark { amount } else { -amount }).clamp(0., 1.);
        color
    };
    let primary_hover = shade(c.primary, 0.04);
    let primary_active = shade(c.primary, 0.08);
    let secondary_hover = shade(c.secondary, 0.03);
    let secondary_active = shade(c.secondary, 0.06);
    macro_rules! set {
        ($color:expr; $($field:ident),+ $(,)?) => { $(theme.colors.$field = $color;)+ };
    }
    set!(c.primary; button_primary, caret, progress_bar, slider_bar, slider_thumb, switch, link, sidebar_primary, table_active_border, list_active_border);
    set!(primary_hover; primary_hover, button_primary_hover, link_hover);
    set!(primary_active; primary_active, button_primary_active, link_active);
    set!(c.primary_foreground; button_primary_foreground, sidebar_primary_foreground);
    set!(c.secondary; button_secondary);
    set!(secondary_hover; secondary_hover, button_secondary_hover);
    set!(secondary_active; secondary_active, button_secondary_active);
    set!(c.secondary_foreground; button_secondary_foreground);
    set!(c.surface; button, group_box, list, table, tab, tab_bar, sidebar);
    set!(c.surface_foreground; button_foreground, group_box_foreground, tab_foreground, sidebar_foreground);
    set!(c.muted; button_hover, table_even, table_head, table_foot, list_head, list_even, skeleton, tab_bar_segmented);
    set!(c.foreground; table_head_foreground, table_foot_foreground);
    set!(c.accent; button_active, table_hover, table_active, list_hover, list_active, tab_active, sidebar_accent);
    set!(c.accent_foreground; tab_active_foreground, sidebar_accent_foreground);
    set!(c.border; table_row_border, sidebar_border, status_bar_border, title_bar_border, window_border);
    set!(c.destructive; button_danger);
    set!(c.destructive_foreground; button_danger_foreground);
    theme.danger_hover = shade(c.destructive, 0.04);
    theme.danger_active = shade(c.destructive, 0.08);
    theme.button_danger_hover = theme.danger_hover;
    theme.button_danger_active = theme.danger_active;
    theme.button_success = theme.success;
    theme.button_warning = theme.warning;
    theme.success_hover = shade(theme.success, 0.04);
    theme.success_active = shade(theme.success, 0.08);
    theme.button_success_hover = theme.success_hover;
    theme.button_success_active = theme.success_active;
    theme.warning_hover = shade(theme.warning, 0.04);
    theme.warning_active = shade(theme.warning, 0.08);
    theme.button_warning_hover = theme.warning_hover;
    theme.button_warning_active = theme.warning_active;
    set!(c.primary; info, button_info);
    set!(primary_hover; info_hover, button_info_hover);
    set!(primary_active; info_active, button_info_active);
    set!(c.primary_foreground; info_foreground, button_info_foreground, switch_thumb);
    theme.button_success_foreground = c.primary_foreground;
    theme.button_warning_foreground = c.primary_foreground;
    theme.success_foreground = c.primary_foreground;
    theme.warning_foreground = c.primary_foreground;
}

/// Application content spacing in logical pixels for the active density.
pub fn content_gap(cx: &App) -> gpui_kit::Pixels {
    let service = ThemeService::global(cx);
    service.active.spacing.md * service.density.spacing_factor()
}

#[cfg(test)]
mod tests {
    use super::{ThemeDefinition, ThemeError, ThemeRegistry, parse_color};
    #[test]
    fn builtins_roundtrip_and_duplicate_registration_is_atomic() {
        let mut registry = ThemeRegistry::built_in().unwrap();
        let theme = registry.get("light").unwrap().clone();
        assert_eq!(
            ThemeDefinition::from_json(&theme.to_json().unwrap())
                .unwrap()
                .name(),
            "light"
        );
        assert!(matches!(
            registry.register(theme),
            Err(ThemeError::Duplicate(_))
        ));
        assert_eq!(registry.themes().count(), 2);
        assert!(matches!(
            registry.get("missing"),
            Err(ThemeError::Missing(_))
        ));
    }
    #[test]
    fn rejects_bad_colors_names_and_scales() {
        for color in ["red", "#ffff", "#GG0000", "#１２３４５６"] {
            assert!(parse_color(color).is_err());
        }
        let json = include_str!("../themes/light.json");
        assert!(ThemeDefinition::from_json(&json.replace("#2563EB", "#GG0000")).is_err());
        assert!(ThemeDefinition::from_json(&json.replace("\"light\"", "\"\"")).is_err());
        let mut value: serde_json::Value = serde_json::from_str(json).unwrap();
        value["tokens"]["spacing"]["md"] = serde_json::json!(-1);
        assert!(ThemeDefinition::from_json(&value.to_string()).is_err());
    }
    #[test]
    fn primary_override_validates_and_keeps_source_unchanged() {
        let registry = ThemeRegistry::built_in().unwrap();
        let custom = registry
            .get("light")
            .unwrap()
            .clone()
            .with_primary("ocean", "#006677")
            .unwrap();
        assert_eq!(custom.name(), "ocean");
        assert_eq!(registry.get("light").unwrap().name(), "light");
    }
}
