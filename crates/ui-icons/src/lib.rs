//! Semantic names and validated SVG packs rendered by GPUI Kit.
use gpui_kit::component::{ActiveTheme, Icon, Sizable};
use gpui_kit::{App, SharedString, Styled, px};
use std::{collections::BTreeMap, sync::Arc};
use thiserror::Error;

/// Built in semantic icon names. Assets are original line drawings under Apache-2.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconName {
    /// Search a collection.
    Search,
    /// Application configuration.
    Settings,
    /// Expand a disclosure.
    ChevronDown,
    /// Overview destination.
    Home,
    /// Component collection.
    Components,
    /// Theme destination.
    Palette,
    /// Layout destination.
    Layout,
    /// Icon collection.
    Icons,
    /// Interactive playground.
    Playground,
    /// Documentation destination.
    Book,
    /// Copy a value.
    Copy,
    /// Confirm a change.
    Check,
    /// Collapse navigation.
    Menu,
}
impl IconName {
    /// Complete built in catalog in a stable order.
    pub const ALL: [Self; 13] = [
        Self::Search,
        Self::Settings,
        Self::ChevronDown,
        Self::Home,
        Self::Components,
        Self::Palette,
        Self::Layout,
        Self::Icons,
        Self::Playground,
        Self::Book,
        Self::Copy,
        Self::Check,
        Self::Menu,
    ];
    /// Stable short name within the `ui` namespace.
    pub fn name(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::Settings => "settings",
            Self::ChevronDown => "chevron-down",
            Self::Home => "home",
            Self::Components => "components",
            Self::Palette => "palette",
            Self::Layout => "layout",
            Self::Icons => "icons",
            Self::Playground => "playground",
            Self::Book => "book",
            Self::Copy => "copy",
            Self::Check => "check",
            Self::Menu => "menu",
        }
    }
    /// Qualified strongly typed identifier.
    pub fn id(self) -> IconId {
        IconId(format!("ui:{}", self.name()))
    }
    fn svg(self) -> &'static [u8] {
        match self {
            Self::Search => include_bytes!("../assets/search.svg"),
            Self::Settings => include_bytes!("../assets/settings.svg"),
            Self::ChevronDown => include_bytes!("../assets/chevron-down.svg"),
            Self::Home => include_bytes!("../assets/home.svg"),
            Self::Components => include_bytes!("../assets/components.svg"),
            Self::Palette => include_bytes!("../assets/palette.svg"),
            Self::Layout => include_bytes!("../assets/layout.svg"),
            Self::Icons => include_bytes!("../assets/icons.svg"),
            Self::Playground => include_bytes!("../assets/playground.svg"),
            Self::Book => include_bytes!("../assets/book.svg"),
            Self::Copy => include_bytes!("../assets/copy.svg"),
            Self::Check => include_bytes!("../assets/check.svg"),
            Self::Menu => include_bytes!("../assets/menu.svg"),
        }
    }
}
/// Validated `namespace:name` identity used by custom packs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IconId(String);
impl IconId {
    /// Validate exactly two nonempty lowercase ASCII name segments.
    pub fn new(namespace: &str, name: &str) -> Result<Self, IconError> {
        for part in [namespace, name] {
            if part.is_empty()
                || part.len() > 64
                || !part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            {
                return Err(IconError::InvalidId(format!("{namespace}:{name}")));
            }
        }
        Ok(Self(format!("{namespace}:{name}")))
    }
    /// Qualified registry key.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
/// Registry and SVG validation errors.
#[derive(Debug, Error)]
pub enum IconError {
    /// Invalid namespace or semantic name.
    #[error("invalid icon identifier: {0}")]
    InvalidId(String),
    /// Registration would overwrite an existing asset.
    #[error("icon already registered: {0}")]
    Duplicate(String),
    /// No asset is registered under the requested name.
    #[error("icon not found: {0}")]
    Missing(String),
    /// SVG is malformed or outside the supported small static SVG subset.
    #[error("invalid SVG: {0}")]
    InvalidSvg(String),
}
/// Visual state for semantic coloring; does not replace a control's disabled behavior.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum IconState {
    /// Current foreground.
    #[default]
    Default,
    /// Primary accent.
    Active,
    /// Muted foreground.
    Disabled,
}
/// Finite supported icon dimensions in logical pixels.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum IconSize {
    /// 14 pixels.
    Small,
    /// 18 pixels.
    #[default]
    Medium,
    /// 28 pixels.
    Large,
}
impl IconSize {
    /// Logical pixel extent.
    pub fn pixels(self) -> f32 {
        match self {
            Self::Small => 14.,
            Self::Medium => 18.,
            Self::Large => 28.,
        }
    }
}
/// Small deterministic registry; SVG parsing/rendering remains GPUI's responsibility.
#[derive(Debug, Clone, Default)]
pub struct IconRegistry {
    icons: BTreeMap<IconId, Arc<[u8]>>,
}
impl IconRegistry {
    /// Validate and register the packaged built in catalog.
    pub fn built_in() -> Result<Self, IconError> {
        let mut registry = Self::default();
        for name in IconName::ALL {
            registry.register(name.id(), name.svg())?;
        }
        Ok(registry)
    }
    /// Register a custom SVG. Duplicate keys never replace the previous asset.
    pub fn register(&mut self, id: IconId, svg: &[u8]) -> Result<(), IconError> {
        if self.icons.contains_key(&id) {
            return Err(IconError::Duplicate(id.0));
        }
        validate_svg(svg)?;
        self.icons.insert(id, Arc::from(svg));
        Ok(())
    }
    /// Resolve an asset with a useful missing icon error.
    pub fn lookup(&self, id: &IconId) -> Result<&[u8], IconError> {
        self.icons
            .get(id)
            .map(AsRef::as_ref)
            .ok_or_else(|| IconError::Missing(id.0.clone()))
    }
    /// Search by qualified name. An empty query returns the complete catalog.
    pub fn search<'a>(&'a self, query: &'a str) -> impl Iterator<Item = &'a IconId> {
        let query = query.to_ascii_lowercase();
        self.icons.keys().filter(move |id| id.0.contains(&query))
    }
    /// Build an upstream Icon from a validated SVG and the active theme.
    pub fn render(
        &self,
        id: &IconId,
        size: IconSize,
        state: IconState,
        cx: &App,
    ) -> Result<Icon, IconError> {
        let color = match state {
            IconState::Default => cx.theme().foreground,
            IconState::Active => cx.theme().primary,
            IconState::Disabled => cx.theme().muted_foreground,
        };
        Ok(Icon::default()
            .data(self.lookup(id)?)
            .with_size(px(size.pixels()))
            .text_color(color))
    }
    /// Copyable public API usage for an identifier.
    pub fn usage(&self, id: &IconId) -> Result<SharedString, IconError> {
        self.lookup(id)?;
        let (namespace, name) =
            id.0.split_once(':')
                .ok_or_else(|| IconError::InvalidId(id.0.clone()))?;
        Ok(format!("icons.render(&IconId::new(\"{namespace}\", \"{name}\")?, IconSize::Medium, IconState::Default, cx)?").into())
    }
}
/// Validate XML, an SVG root/viewBox, size bounds and local static references.
/// The registry deliberately accepts a small static subset, not arbitrary embedded documents.
pub fn validate_svg(svg: &[u8]) -> Result<(), IconError> {
    if svg.len() > 64 * 1024 {
        return Err(IconError::InvalidSvg("asset exceeds 64 KiB".into()));
    }
    let text = std::str::from_utf8(svg).map_err(|e| IconError::InvalidSvg(e.to_string()))?;
    let document =
        roxmltree::Document::parse(text).map_err(|e| IconError::InvalidSvg(e.to_string()))?;
    let root = document.root_element();
    if root.tag_name().name() != "svg"
        || root.tag_name().namespace() != Some("http://www.w3.org/2000/svg")
    {
        return Err(IconError::InvalidSvg(
            "expected SVG root and namespace".into(),
        ));
    }
    let numbers: Vec<f32> = root
        .attribute("viewBox")
        .unwrap_or_default()
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(|_| IconError::InvalidSvg("invalid viewBox".into()))?;
    if numbers.len() != 4
        || numbers.iter().any(|n| !n.is_finite())
        || numbers[2] <= 0.
        || numbers[3] <= 0.
    {
        return Err(IconError::InvalidSvg(
            "finite, positive viewBox extent required".into(),
        ));
    }
    for node in root.descendants().filter(|node| node.is_element()) {
        if ![
            "svg", "g", "path", "circle", "rect", "ellipse", "line", "polyline", "polygon", "defs",
            "clipPath", "title", "desc",
        ]
        .contains(&node.tag_name().name())
        {
            return Err(IconError::InvalidSvg(format!(
                "unsupported element {}",
                node.tag_name().name()
            )));
        }
        for attribute in node.attributes() {
            if attribute.name().starts_with("on")
                || attribute.name() == "href"
                || attribute.value().contains("url(")
            {
                return Err(IconError::InvalidSvg(
                    "external references and executable styles are unsupported".into(),
                ));
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn catalog_is_valid_unique_and_searchable() {
        let mut r = IconRegistry::built_in().unwrap();
        assert_eq!(r.search("").count(), IconName::ALL.len());
        assert_eq!(r.search("SEARCH").next().unwrap().as_str(), "ui:search");
        assert!(matches!(
            r.register(IconName::Search.id(), IconName::Search.svg()),
            Err(IconError::Duplicate(_))
        ));
        assert!(matches!(
            r.lookup(&IconId::new("custom", "absent").unwrap()),
            Err(IconError::Missing(_))
        ));
        assert!(
            r.usage(&IconName::Search.id())
                .unwrap()
                .contains("IconId::new")
        );
    }
    #[test]
    fn rejects_malformed_assets_and_identifiers() {
        for bytes in [
            b"not SVG".as_slice(),
            b"<svg/>",
            br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><script/></svg>"#,
            br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 -1 2"/>"#,
        ] {
            assert!(validate_svg(bytes).is_err());
        }
        assert!(IconId::new("", "x").is_err());
        assert!(IconId::new("a", "../b").is_err());
    }
}
