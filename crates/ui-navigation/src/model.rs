use gpui_kit::SharedString;
use std::collections::{BTreeMap, BTreeSet};
use ui_icons::IconId;

/// A validation failure. Diagnostics retain the offending identity or tree path.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum NavigationError {
    /// Empty identity.
    #[error("invalid navigation ID: {0:?}")]
    InvalidId(String),
    /// Repeated identity, including hidden and structural nodes.
    #[error("duplicate navigation ID at {0}")]
    DuplicateId(String),
    /// Empty visible label.
    #[error("empty navigation label at {0}")]
    EmptyLabel(String),
    /// A branch must own children.
    #[error("empty navigation branch at {0}")]
    EmptyBranch(String),
    /// A pattern cannot represent the supplied node.
    #[error("unsupported navigation node at {0}")]
    UnsupportedNode(String),
    /// Non-finite or inconsistent dimensions.
    #[error("invalid navigation metrics: {0}")]
    InvalidMetrics(String),
}
impl NavigationError {
    /// Offending identity, path, or metric diagnostic.
    pub fn diagnostic(&self) -> &str {
        match self {
            Self::InvalidId(s)
            | Self::DuplicateId(s)
            | Self::EmptyLabel(s)
            | Self::EmptyBranch(s)
            | Self::UnsupportedNode(s)
            | Self::InvalidMetrics(s) => s,
        }
    }
}
/// Stable unnormalized identity, independent of labels and ordering.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NavigationId(SharedString);
impl NavigationId {
    /// Validate without silently trimming the stored identity.
    pub fn new(value: impl Into<SharedString>) -> Result<Self, NavigationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(NavigationError::InvalidId(value.to_string()));
        }
        Ok(Self(value))
    }
    /// Original identity.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
/// Panel presentation adjacent to a dockbar rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DockbarMode {
    /// Panel participates in layout.
    Inline,
    /// Nonmodal upstream popup.
    Overlay,
}
/// Native composition selected by the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum NavigationKind {
    /// Expandable vertical list.
    #[default]
    Sidebar,
    /// Icon rail and optional panel.
    Dockbar(DockbarMode),
    /// Permanently compact flat list.
    Minidock,
    /// Horizontal root controls.
    Navbar,
    /// One trigger opening a list.
    Flyout,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum BadgeValue {
    Count(usize),
    Text(SharedString),
}
/// Application supplied metadata; counts are never accumulated by this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationBadge {
    value: BadgeValue,
    description: Option<SharedString>,
}
impl NavigationBadge {
    /// A count; zero is hidden and not announced.
    pub fn count(value: usize) -> Self {
        Self {
            value: BadgeValue::Count(value),
            description: None,
        }
    }
    /// A nonempty text badge.
    pub fn text(value: impl Into<SharedString>) -> Result<Self, NavigationError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(NavigationError::EmptyLabel("badge".into()));
        }
        Ok(Self {
            value: BadgeValue::Text(value),
            description: None,
        })
    }
    /// Localized optional description following the original count or text.
    pub fn with_description(mut self, value: impl Into<SharedString>) -> Self {
        self.description = Some(value.into());
        self
    }
    /// Original count, if numeric.
    pub fn count_value(&self) -> Option<usize> {
        match self.value {
            BadgeValue::Count(n) => Some(n),
            _ => None,
        }
    }
    /// Visible text, capping counts at 99+.
    pub fn display_text(&self) -> Option<SharedString> {
        match &self.value {
            BadgeValue::Count(0) => None,
            BadgeValue::Count(n) => Some(if *n > 99 {
                "99+".into()
            } else {
                n.to_string().into()
            }),
            BadgeValue::Text(t) => Some(t.clone()),
        }
    }
    /// Accessible full node name, preserving the original unabridged count.
    pub fn accessible_label(&self, label: &str) -> SharedString {
        if self.display_text().is_none() {
            return label.to_owned().into();
        }
        let value = match &self.value {
            BadgeValue::Count(n) => n.to_string(),
            BadgeValue::Text(t) => t.to_string(),
        };
        let suffix = self
            .description
            .as_ref()
            .filter(|s| !s.trim().is_empty())
            .map(|s| format!(" {s}"))
            .unwrap_or_default();
        format!("{label}, {value}{suffix}").into()
    }
}
/// An activatable destination, with no application action or route stored inside.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationItem {
    pub(crate) id: NavigationId,
    pub(crate) label: SharedString,
    pub(crate) subtitle: Option<SharedString>,
    pub(crate) tooltip: Option<SharedString>,
    pub(crate) icon: Option<IconId>,
    pub(crate) badge: Option<NavigationBadge>,
    pub(crate) visible: bool,
    pub(crate) disabled: bool,
}
/// A disclosure that requests expansion or a panel rather than leaf selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationBranch {
    pub(crate) item: NavigationItem,
    pub(crate) children: Vec<NavigationNode>,
}
/// A noninteractive labelled grouping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationGroup {
    pub(crate) id: NavigationId,
    pub(crate) label: SharedString,
    pub(crate) children: Vec<NavigationNode>,
    pub(crate) visible: bool,
}
impl NavigationItem {
    /// Construct a destination; model construction validates labels.
    pub fn new(id: NavigationId, label: impl Into<SharedString>) -> Self {
        Self {
            id,
            label: label.into(),
            subtitle: None,
            tooltip: None,
            icon: None,
            badge: None,
            visible: true,
            disabled: false,
        }
    }
    /// Secondary visible description.
    pub fn subtitle(mut self, value: impl Into<SharedString>) -> Self {
        self.subtitle = Some(value.into());
        self
    }
    /// Stable identity.
    pub fn id(&self) -> &NavigationId {
        &self.id
    }
    /// Full name.
    pub fn label(&self) -> &str {
        &self.label
    }
    /// Optional description.
    pub fn subtitle_text(&self) -> Option<&str> {
        self.subtitle.as_deref()
    }
    /// Optional tooltip.
    pub fn tooltip_text(&self) -> Option<&str> {
        self.tooltip.as_deref()
    }
    /// Optional registered icon.
    pub fn icon_id(&self) -> Option<&IconId> {
        self.icon.as_ref()
    }
    /// Optional metadata.
    pub fn badge_value(&self) -> Option<&NavigationBadge> {
        self.badge.as_ref()
    }
    /// Visibility, before ancestor filtering.
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    /// Disabled, before ancestor inheritance.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
    /// Full accessible name.
    pub fn accessible_label(&self) -> SharedString {
        self.badge
            .as_ref()
            .map_or_else(|| self.label.clone(), |b| b.accessible_label(&self.label))
    }
    /// Unicode-safe icon fallback.
    pub fn fallback_label(&self) -> SharedString {
        self.label.chars().take(2).collect::<String>().into()
    }
}
macro_rules! item_builders {
    ($ty:ty, $field:ident) => {
        impl $ty {
            /// Registered icon identity.
            pub fn icon(mut self, value: IconId) -> Self {
                self.$field.icon = Some(value);
                self
            }
            /// Metadata supplied by the application.
            pub fn badge(mut self, value: NavigationBadge) -> Self {
                self.$field.badge = Some(value);
                self
            }
            /// Optional tooltip.
            pub fn tooltip(mut self, value: impl Into<SharedString>) -> Self {
                self.$field.tooltip = Some(value.into());
                self
            }
            /// Hide this entire subtree.
            pub fn visible(mut self, value: bool) -> Self {
                self.$field.visible = value;
                self
            }
            /// Disable this control and its descendants.
            pub fn disabled(mut self, value: bool) -> Self {
                self.$field.disabled = value;
                self
            }
        }
    };
}
// Item builders are spelled out to keep the public type's fields private.
impl NavigationItem {
    /// Registered icon identity.
    pub fn icon(mut self, value: IconId) -> Self {
        self.icon = Some(value);
        self
    }
    /// Application metadata.
    pub fn badge(mut self, value: NavigationBadge) -> Self {
        self.badge = Some(value);
        self
    }
    /// Optional tooltip.
    pub fn tooltip(mut self, value: impl Into<SharedString>) -> Self {
        self.tooltip = Some(value.into());
        self
    }
    /// Visibility.
    pub fn visible(mut self, value: bool) -> Self {
        self.visible = value;
        self
    }
    /// Disabled state.
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
}
item_builders!(NavigationBranch, item);
impl NavigationBranch {
    /// Construct a branch; model construction validates children and labels.
    pub fn new(
        id: NavigationId,
        label: impl Into<SharedString>,
        children: Vec<NavigationNode>,
    ) -> Self {
        Self {
            item: NavigationItem::new(id, label),
            children,
        }
    }
    /// Stable branch identity.
    pub fn id(&self) -> &NavigationId {
        &self.item.id
    }
    /// Full branch name.
    pub fn label(&self) -> &str {
        &self.item.label
    }
    /// Child nodes in source order.
    pub fn children(&self) -> &[NavigationNode] {
        &self.children
    }
    /// Disclosure metadata.
    pub fn metadata(&self) -> &NavigationItem {
        &self.item
    }
}
impl NavigationGroup {
    /// Construct a labelled noninteractive group.
    pub fn new(
        id: NavigationId,
        label: impl Into<SharedString>,
        children: Vec<NavigationNode>,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            children,
            visible: true,
        }
    }
    /// Hide this grouping and its entire subtree.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
    /// Stable grouping identity.
    pub fn id(&self) -> &NavigationId {
        &self.id
    }
    /// Group heading.
    pub fn label(&self) -> &str {
        &self.label
    }
    /// Children in source order.
    pub fn children(&self) -> &[NavigationNode] {
        &self.children
    }
    /// Group visibility.
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}
/// Owned tree nodes. Structural nodes also require unique stable IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NavigationNode {
    /// Leaf destination.
    Item(NavigationItem),
    /// Expandable branch.
    Branch(NavigationBranch),
    /// Noninteractive group.
    Group(NavigationGroup),
    /// Divider.
    Separator(NavigationId),
    /// Flexible vertical remainder.
    Spacer(NavigationId),
}
impl From<NavigationItem> for NavigationNode {
    fn from(v: NavigationItem) -> Self {
        Self::Item(v)
    }
}
impl From<NavigationBranch> for NavigationNode {
    fn from(v: NavigationBranch) -> Self {
        Self::Branch(v)
    }
}
impl From<NavigationGroup> for NavigationNode {
    fn from(v: NavigationGroup) -> Self {
        Self::Group(v)
    }
}
impl NavigationNode {
    /// Stable divider.
    pub fn separator(id: NavigationId) -> Self {
        Self::Separator(id)
    }
    /// Flexible remainder in vertical lists.
    pub fn spacer(id: NavigationId) -> Self {
        Self::Spacer(id)
    }
    /// Stable identity for every variant.
    pub fn id(&self) -> &NavigationId {
        match self {
            Self::Item(i) => &i.id,
            Self::Branch(b) => &b.item.id,
            Self::Group(g) => &g.id,
            Self::Separator(id) | Self::Spacer(id) => id,
        }
    }
    pub(crate) fn children(&self) -> &[Self] {
        match self {
            Self::Branch(b) => &b.children,
            Self::Group(g) => &g.children,
            _ => &[],
        }
    }
    pub(crate) fn visible(&self) -> bool {
        match self {
            Self::Item(i) => i.visible,
            Self::Branch(b) => b.item.visible,
            Self::Group(g) => g.visible,
            _ => true,
        }
    }
    pub(crate) fn control(&self) -> Option<&NavigationItem> {
        match self {
            Self::Item(i) => Some(i),
            Self::Branch(b) => Some(&b.item),
            _ => None,
        }
    }
}
/// Validated immutable model with a precomputed identity index.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NavigationModel {
    nodes: Vec<NavigationNode>,
    index: BTreeMap<NavigationId, Vec<usize>>,
}
impl NavigationModel {
    /// Validate all nodes, including hidden subtrees, before creating the model.
    pub fn new(nodes: Vec<NavigationNode>) -> Result<Self, NavigationError> {
        fn walk(
            nodes: &[NavigationNode],
            parent: &[usize],
            index: &mut BTreeMap<NavigationId, Vec<usize>>,
        ) -> Result<(), NavigationError> {
            for (i, node) in nodes.iter().enumerate() {
                let mut path = parent.to_vec();
                path.push(i);
                let diagnostic = format!("{} ({path:?})", node.id().as_str());
                if index.insert(node.id().clone(), path.clone()).is_some() {
                    return Err(NavigationError::DuplicateId(diagnostic));
                }
                let label = match node {
                    NavigationNode::Item(i) => Some(&i.label),
                    NavigationNode::Branch(b) => Some(&b.item.label),
                    NavigationNode::Group(g) => Some(&g.label),
                    _ => None,
                };
                if label.is_some_and(|s| s.trim().is_empty()) {
                    return Err(NavigationError::EmptyLabel(diagnostic));
                }
                if matches!(node, NavigationNode::Branch(b) if b.children.is_empty()) {
                    return Err(NavigationError::EmptyBranch(diagnostic));
                }
                walk(node.children(), &path, index)?;
            }
            Ok(())
        }
        let mut index = BTreeMap::new();
        walk(&nodes, &[], &mut index)?;
        Ok(Self { nodes, index })
    }
    /// Root nodes in source order.
    pub fn nodes(&self) -> &[NavigationNode] {
        &self.nodes
    }
    /// Any node, including structural and hidden nodes.
    pub fn item(&self, id: &NavigationId) -> Option<&NavigationNode> {
        let mut nodes = self.nodes();
        let mut found = None;
        for i in self.index.get(id)? {
            let n = nodes.get(*i)?;
            nodes = n.children();
            found = Some(n);
        }
        found
    }
    pub(crate) fn ancestors(&self, id: &NavigationId) -> Vec<NavigationId> {
        let Some(path) = self.index.get(id) else {
            return vec![];
        };
        let mut nodes = self.nodes();
        let mut result = vec![];
        for i in path.iter().take(path.len().saturating_sub(1)) {
            let n = &nodes[*i];
            result.push(n.id().clone());
            nodes = n.children();
        }
        result
    }
    pub(crate) fn is_effectively_visible(&self, id: &NavigationId) -> bool {
        self.item(id).is_some_and(NavigationNode::visible)
            && self
                .ancestors(id)
                .iter()
                .all(|parent| self.item(parent).is_some_and(NavigationNode::visible))
    }
    pub(crate) fn validate_kind(&self, kind: NavigationKind) -> Result<(), NavigationError> {
        if kind == NavigationKind::Minidock {
            for n in &self.nodes {
                if matches!(n, NavigationNode::Group(_) | NavigationNode::Branch(_)) {
                    return Err(NavigationError::UnsupportedNode(n.id().as_str().into()));
                }
            }
        }
        Ok(())
    }
}
/// Application-owned values; invalid or hidden identities remain stored without effects.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NavigationPresentation {
    pub(crate) selected: Option<NavigationId>,
    pub(crate) expanded: BTreeSet<NavigationId>,
    pub(crate) open_panel: Option<NavigationId>,
    pub(crate) collapsed: bool,
    pub(crate) flyout_open: bool,
}
impl NavigationPresentation {
    /// Set the requested selected destination.
    pub fn with_selected(mut self, id: Option<NavigationId>) -> Self {
        self.selected = id;
        self
    }
    /// Set expanded branch identities.
    pub fn with_expanded(mut self, ids: impl IntoIterator<Item = NavigationId>) -> Self {
        self.expanded = ids.into_iter().collect();
        self
    }
    /// Set the requested root panel identity.
    pub fn with_open_panel(mut self, id: Option<NavigationId>) -> Self {
        self.open_panel = id;
        self
    }
    /// Sidebar compact presentation.
    pub fn collapsed(mut self, value: bool) -> Self {
        self.collapsed = value;
        self
    }
    /// Flyout popup state.
    pub fn flyout_open(mut self, value: bool) -> Self {
        self.flyout_open = value;
        self
    }
    /// Selected input identity.
    pub fn selected(&self) -> Option<&NavigationId> {
        self.selected.as_ref()
    }
    /// Expanded input identities.
    pub fn expanded(&self) -> &BTreeSet<NavigationId> {
        &self.expanded
    }
    /// Requested root panel.
    pub fn open_panel(&self) -> Option<&NavigationId> {
        self.open_panel.as_ref()
    }
    /// Sidebar compact input value.
    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }
    /// Flyout input value.
    pub fn is_flyout_open(&self) -> bool {
        self.flyout_open
    }
}
/// Preferred widths in rem, resolved on each frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMetrics {
    sidebar: f32,
    rail: f32,
    panel: f32,
    flyout: f32,
}
impl Default for NavigationMetrics {
    fn default() -> Self {
        Self {
            sidebar: 16.,
            rail: 3.,
            panel: 18.,
            flyout: 24.,
        }
    }
}
impl NavigationMetrics {
    /// Check finite positive widths, a minimum 3rem rail, and surface widths >= rail.
    pub fn validate(&self) -> Result<(), NavigationError> {
        if [self.sidebar, self.rail, self.panel, self.flyout]
            .iter()
            .any(|n| !n.is_finite() || *n <= 0.)
            || self.rail < 3.
            || [self.sidebar, self.panel, self.flyout]
                .iter()
                .any(|n| *n < self.rail)
        {
            return Err(NavigationError::InvalidMetrics(format!(
                "{self:?}; rail >= 3, surfaces >= rail, all finite"
            )));
        }
        Ok(())
    }
    /// Preferred sidebar width in rem.
    pub fn sidebar_width(&self) -> f32 {
        self.sidebar
    }
    /// Preferred rail width in rem.
    pub fn rail_width(&self) -> f32 {
        self.rail
    }
    /// Preferred dock/navbar panel width in rem.
    pub fn panel_width(&self) -> f32 {
        self.panel
    }
    /// Preferred flyout width in rem.
    pub fn flyout_width(&self) -> f32 {
        self.flyout
    }
    /// Replace sidebar width, validating the entire result.
    pub fn with_sidebar_width(mut self, value: f32) -> Result<Self, NavigationError> {
        self.sidebar = value;
        self.validate()?;
        Ok(self)
    }
    /// Replace rail width, validating the entire result.
    pub fn with_rail_width(mut self, value: f32) -> Result<Self, NavigationError> {
        self.rail = value;
        self.validate()?;
        Ok(self)
    }
    /// Replace panel width, validating the entire result.
    pub fn with_panel_width(mut self, value: f32) -> Result<Self, NavigationError> {
        self.panel = value;
        self.validate()?;
        Ok(self)
    }
    /// Replace flyout width, validating the entire result.
    pub fn with_flyout_width(mut self, value: f32) -> Result<Self, NavigationError> {
        self.flyout = value;
        self.validate()?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn id(s: &str) -> NavigationId {
        NavigationId::new(s.to_owned()).unwrap()
    }
    #[test]
    fn validates_hidden_structural_nodes_and_reports_identity() {
        assert!(NavigationId::new(" \n").is_err());
        assert_eq!(NavigationId::new(" kept ").unwrap().as_str(), " kept ");
        let hidden = NavigationBranch::new(
            id("parent"),
            "Parent",
            vec![NavigationItem::new(id("same"), "One").visible(false).into()],
        )
        .visible(false);
        let error =
            NavigationModel::new(vec![hidden.into(), NavigationNode::separator(id("same"))])
                .unwrap_err();
        assert!(matches!(error, NavigationError::DuplicateId(_)));
        assert!(error.diagnostic().contains("same"));
        assert!(matches!(
            NavigationModel::new(vec![
                NavigationItem::new(id("empty"), " ").visible(false).into()
            ]),
            Err(NavigationError::EmptyLabel(_))
        ));
        assert!(matches!(
            NavigationModel::new(vec![
                NavigationBranch::new(id("empty"), "Empty", vec![]).into()
            ]),
            Err(NavigationError::EmptyBranch(_))
        ));
    }
    #[test]
    fn badge_names_preserve_original_count_and_hide_zero() {
        for (count, display) in [
            (0, None),
            (1, Some("1")),
            (99, Some("99")),
            (120, Some("99+")),
        ] {
            let badge = NavigationBadge::count(count).with_description("unread messages");
            assert_eq!(badge.display_text().as_deref(), display);
            assert_eq!(
                badge.accessible_label("Inbox").as_ref(),
                if count == 0 {
                    "Inbox".to_owned()
                } else {
                    format!("Inbox, {count} unread messages")
                }
            );
        }
        assert_eq!(
            NavigationBadge::text("New")
                .unwrap()
                .with_description("release")
                .accessible_label("Updates")
                .as_ref(),
            "Updates, New release"
        );
        assert!(NavigationBadge::text(" ").is_err());
        assert_eq!(
            NavigationItem::new(id("unicode"), "🧭界place")
                .fallback_label()
                .as_ref(),
            "🧭界"
        );
    }
    #[test]
    fn validates_finite_and_consistent_metrics() {
        let defaults = NavigationMetrics::default();
        for value in [0., -1., f32::NAN, f32::INFINITY, 2.9] {
            assert!(defaults.with_rail_width(value).is_err());
        }
        assert!(defaults.with_sidebar_width(2.).is_err());
        assert!(defaults.with_panel_width(2.).is_err());
        assert!(defaults.with_flyout_width(2.).is_err());
        assert!(defaults.with_panel_width(30.).is_ok());
    }
    #[test]
    fn whole_tree_equivalence_and_flat_validation() {
        let flat = NavigationModel::new(vec![
            NavigationItem::new(id("inbox"), "Inbox").into(),
            NavigationNode::spacer(id("space")),
        ])
        .unwrap();
        assert_eq!(flat, flat.clone());
        assert!(flat.validate_kind(NavigationKind::Minidock).is_ok());
        let tree = NavigationModel::new(vec![
            NavigationGroup::new(id("group"), "Work", flat.nodes().to_vec()).into(),
        ])
        .unwrap();
        assert!(tree.validate_kind(NavigationKind::Minidock).is_err());
        assert_eq!(tree.ancestors(&id("inbox")), vec![id("group")]);
    }
}
