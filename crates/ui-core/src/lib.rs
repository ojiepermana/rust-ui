//! Small semantic contracts shared by the UI foundation.
use serde::{Deserialize, Serialize};

/// Requested application appearance, resolved using native window events.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeMode {
    /// Use the registered light theme.
    Light,
    /// Use the registered dark theme.
    Dark,
    /// Follow the operating system appearance.
    #[default]
    System,
}

/// Application content density. Standard component sizes remain upstream APIs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Density {
    /// Tight content spacing.
    Compact,
    /// Default desktop spacing.
    #[default]
    Comfortable,
}

impl Density {
    /// Multiplier for application owned spacing tokens.
    pub fn spacing_factor(self) -> f32 {
        match self {
            Self::Compact => 0.75,
            Self::Comfortable => 1.0,
        }
    }
}
