use crate::*;
use gpui_kit::{
    App, Context, EventEmitter, FocusHandle, KeyBinding, ScrollHandle, SharedString, Window,
    actions,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

actions!(
    ui_navigation,
    [
        /// Close the focused instance inline panel.
        CloseNavigationPanel
    ]
);
/// Register instance-scoped inline panel Escape after GPUI Kit initialization.
pub fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new(
        "escape",
        CloseNavigationPanel,
        Some("NavigationPanel"),
    )]);
}
/// A request emitted by user interaction. No event mutates application values.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NavigationEvent {
    /// Request activation of a leaf.
    Activate {
        /// Stable leaf identity.
        id: NavigationId,
    },
    /// Request a branch's expansion state.
    ExpandedChange {
        /// Stable branch identity.
        id: NavigationId,
        /// Requested value.
        expanded: bool,
    },
    /// Request a root panel, or dismissal.
    PanelChange {
        /// Requested branch, or no panel.
        id: Option<NavigationId>,
    },
    /// Request compact sidebar presentation.
    CollapsedChange {
        /// Requested compact value.
        collapsed: bool,
    },
    /// Request flyout visibility.
    FlyoutOpenChange {
        /// Requested open value.
        open: bool,
    },
}
#[derive(Clone, Default)]
pub(crate) struct Projection {
    pub controls: BTreeSet<NavigationId>,
    pub panel: Option<NavigationId>,
}
pub(crate) fn roots(nodes: &[NavigationNode]) -> Vec<&NavigationNode> {
    let mut result = vec![];
    for n in nodes.iter().filter(|n| n.visible()) {
        match n {
            NavigationNode::Group(g) => result.extend(roots(&g.children)),
            _ => result.push(n),
        }
    }
    result
}
fn walk_controls(
    nodes: &[NavigationNode],
    p: &NavigationPresentation,
    disabled: bool,
    ids: &mut BTreeSet<NavigationId>,
) {
    for n in nodes.iter().filter(|n| n.visible()) {
        match n {
            NavigationNode::Item(i) => {
                if !disabled && !i.disabled {
                    ids.insert(i.id.clone());
                }
            }
            NavigationNode::Branch(b) => {
                let disabled = disabled || b.item.disabled;
                if !disabled {
                    ids.insert(b.item.id.clone());
                }
                if p.expanded.contains(&b.item.id) {
                    walk_controls(&b.children, p, disabled, ids);
                }
            }
            NavigationNode::Group(g) => walk_controls(&g.children, p, disabled, ids),
            _ => {}
        }
    }
}
pub(crate) fn projection(
    kind: NavigationKind,
    model: &NavigationModel,
    p: &NavigationPresentation,
) -> Projection {
    let mut result = Projection::default();
    match kind {
        NavigationKind::Sidebar if !p.collapsed => {
            walk_controls(model.nodes(), p, false, &mut result.controls)
        }
        NavigationKind::Flyout => {
            if p.flyout_open {
                walk_controls(model.nodes(), p, false, &mut result.controls);
            }
        }
        _ => {
            for n in roots(model.nodes()) {
                if let Some(i) = n.control()
                    && !i.disabled
                {
                    result.controls.insert(i.id.clone());
                    if kind != NavigationKind::Minidock
                        && p.open_panel.as_ref() == Some(&i.id)
                        && let NavigationNode::Branch(b) = n
                    {
                        result.panel = Some(i.id.clone());
                        walk_controls(&b.children, p, false, &mut result.controls);
                    }
                }
            }
        }
    }
    result
}
/// Retained per-instance state. Own this entity and subscriptions outside render.
pub struct NavigationState {
    pub(crate) instance: NavigationId,
    pub(crate) kind: NavigationKind,
    pub(crate) model: Arc<NavigationModel>,
    pub(crate) presentation: NavigationPresentation,
    pub(crate) metrics: NavigationMetrics,
    pub(crate) icons: Option<Arc<ui_icons::IconRegistry>>,
    pub(crate) focus: BTreeMap<NavigationId, FocusHandle>,
    pub(crate) scopes: BTreeMap<SharedString, FocusHandle>,
    pub(crate) scroll: BTreeMap<SharedString, ScrollHandle>,
    fallback: FocusHandle,
    pub(crate) revision: u64,
    pending: Vec<PopupRequest>,
    pending_scheduled: bool,
}
#[derive(Clone)]
pub(crate) struct PopupRequest {
    pub revision: u64,
    pub origin: Option<NavigationId>,
    pub open: bool,
    pub flyout: bool,
}
impl EventEmitter<NavigationEvent> for NavigationState {}
impl NavigationState {
    /// Create an empty valid sidebar. The fallback must remain mounted.
    pub fn new(
        instance_id: NavigationId,
        fallback_focus: FocusHandle,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut state = Self {
            instance: instance_id,
            kind: NavigationKind::Sidebar,
            model: Arc::new(NavigationModel::default()),
            presentation: NavigationPresentation::default(),
            metrics: NavigationMetrics::default(),
            icons: None,
            focus: BTreeMap::new(),
            scopes: BTreeMap::new(),
            scroll: BTreeMap::new(),
            fallback: fallback_focus,
            revision: 0,
            pending: vec![],
            pending_scheduled: false,
        };
        state.prepare_handles(cx);
        state
    }
    fn prepare_handles(&mut self, cx: &mut Context<Self>) {
        for id in ["root", "flyout", "trigger", "toggle", "close"] {
            self.scopes
                .entry(id.into())
                .or_insert_with(|| cx.focus_handle());
        }
        self.scroll.entry("root".into()).or_default();
        fn walk(
            nodes: &[NavigationNode],
            focus: &mut BTreeMap<NavigationId, FocusHandle>,
            scopes: &mut BTreeMap<SharedString, FocusHandle>,
            scroll: &mut BTreeMap<SharedString, ScrollHandle>,
            cx: &mut Context<NavigationState>,
        ) {
            for node in nodes {
                if node.control().is_some() {
                    focus
                        .entry(node.id().clone())
                        .or_insert_with(|| cx.focus_handle());
                }
                if matches!(node, NavigationNode::Branch(_)) {
                    let region: SharedString = format!("panel:{}", node.id().as_str()).into();
                    scopes
                        .entry(region.clone())
                        .or_insert_with(|| cx.focus_handle());
                    scroll.entry(region).or_default();
                }
                walk(node.children(), focus, scopes, scroll, cx);
            }
        }
        walk(
            self.model.nodes(),
            &mut self.focus,
            &mut self.scopes,
            &mut self.scroll,
            cx,
        );
    }
    /// Atomically validate and install kind, model and values, with a single notification.
    pub fn configure(
        &mut self,
        kind: NavigationKind,
        model: Arc<NavigationModel>,
        presentation: NavigationPresentation,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(), NavigationError> {
        model.validate_kind(kind)?;
        if self.kind == kind
            && (Arc::ptr_eq(&self.model, &model) || self.model == model)
            && self.presentation == presentation
        {
            return Ok(());
        }
        self.reconcile(kind, &model, &presentation, window, cx);
        self.kind = kind;
        self.model = model;
        self.presentation = presentation;
        self.revision += 1;
        self.prepare_handles(cx);
        self.focus.retain(|id, _| self.model.item(id).is_some());
        self.scopes
            .retain(|key, _| !key.starts_with("panel:") || self.model.index_contains_panel(key));
        self.scroll
            .retain(|key, _| key.as_ref() == "root" || self.model.index_contains_panel(key));
        cx.notify();
        Ok(())
    }
    /// Replace the model atomically, preserving input presentation values.
    pub fn set_model(
        &mut self,
        model: Arc<NavigationModel>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(), NavigationError> {
        self.configure(self.kind, model, self.presentation.clone(), window, cx)
    }
    /// Change composition atomically; a tree cannot be installed as Minidock.
    pub fn set_kind(
        &mut self,
        kind: NavigationKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(), NavigationError> {
        self.configure(
            kind,
            self.model.clone(),
            self.presentation.clone(),
            window,
            cx,
        )
    }
    /// Synchronize values from the application, without emitting requests.
    pub fn set_presentation(
        &mut self,
        presentation: NavigationPresentation,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let _ = self.configure(self.kind, self.model.clone(), presentation, window, cx);
    }
    /// Set validated rem widths; equal values do not notify.
    pub fn set_metrics(
        &mut self,
        metrics: NavigationMetrics,
        cx: &mut Context<Self>,
    ) -> Result<(), NavigationError> {
        metrics.validate()?;
        if self.metrics != metrics {
            self.metrics = metrics;
            cx.notify();
        }
        Ok(())
    }
    /// Read-only application icon registry. An identical Arc is a no-op.
    pub fn set_icons(&mut self, icons: Arc<ui_icons::IconRegistry>, cx: &mut Context<Self>) {
        if self
            .icons
            .as_ref()
            .is_none_or(|old| !Arc::ptr_eq(old, &icons))
        {
            self.icons = Some(icons);
            cx.notify();
        }
    }
    /// Replace the application's still-mounted fallback.
    pub fn set_fallback_focus(&mut self, focus: FocusHandle) {
        self.fallback = focus;
    }
    /// Current pattern.
    pub fn kind(&self) -> NavigationKind {
        self.kind
    }
    /// Current immutable model.
    pub fn model(&self) -> &Arc<NavigationModel> {
        &self.model
    }
    /// Last application values, including stale identities.
    pub fn presentation(&self) -> &NavigationPresentation {
        &self.presentation
    }
    /// Validated current widths.
    pub fn metrics(&self) -> NavigationMetrics {
        self.metrics
    }
    /// Root scope focus handle, including projected application slots.
    pub fn focus_handle(&self) -> FocusHandle {
        self.scopes["root"].clone()
    }
    /// Stable handle for a control, useful for programmatic focus.
    pub fn item_focus_handle(&self, id: &NavigationId) -> Option<FocusHandle> {
        self.focus.get(id).cloned()
    }
    /// Restore only if focus currently belongs to this instance, never another modal/view.
    pub fn restore_focus(&self, window: &mut Window, cx: &mut Context<Self>) {
        if self.owns_focus(window, cx) {
            self.fallback.focus(window, cx);
        }
    }
    fn owns_focus(&self, window: &Window, cx: &App) -> bool {
        self.scopes
            .values()
            .chain(self.focus.values())
            .any(|h| h.contains_focused(window, cx))
    }
    fn reconcile(
        &self,
        kind: NavigationKind,
        model: &NavigationModel,
        p: &NavigationPresentation,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.owns_focus(window, cx) {
            return;
        }
        let old = projection(self.kind, &self.model, &self.presentation);
        let next = projection(kind, model, p);
        let focused = self
            .focus
            .iter()
            .find(|(_, h)| h.is_focused(window))
            .map(|(id, _)| id.clone());
        if let Some(id) = focused.as_ref()
            && next.controls.contains(id)
            && kind == self.kind
        {
            return;
        }
        let closed_panel = old.panel.as_ref().filter(|id| {
            next.panel.as_ref() != Some(id)
                && self
                    .scopes
                    .get(format!("panel:{}", id.as_str()).as_str())
                    .is_some_and(|h| h.contains_focused(window, cx))
        });
        let popup_closed = self.kind == NavigationKind::Flyout
            && self.presentation.flyout_open
            && (kind != self.kind || !p.flyout_open)
            && self.scopes["flyout"].contains_focused(window, cx);
        if focused.is_none() && closed_panel.is_none() && !popup_closed && kind == self.kind {
            return;
        }
        if popup_closed && kind == NavigationKind::Flyout {
            self.scopes["trigger"].focus(window, cx);
            return;
        }
        let target = closed_panel.cloned().or(focused);
        if let Some(id) = target {
            let mut candidates = vec![id.clone()];
            candidates.extend(self.model.ancestors(&id).into_iter().rev());
            for candidate in candidates {
                if next.controls.contains(&candidate)
                    && let Some(handle) = self.focus.get(&candidate)
                {
                    handle.focus(window, cx);
                    return;
                }
            }
        }
        self.fallback.focus(window, cx);
    }
    pub(crate) fn emit_request(&mut self, event: NavigationEvent, cx: &mut Context<Self>) {
        cx.emit(event);
    }
    pub(crate) fn queue_popup(
        entity: &gpui_kit::WeakEntity<Self>,
        request: PopupRequest,
        window: &mut Window,
        cx: &mut App,
    ) {
        let weak = entity.clone();
        let scheduled = entity
            .update(cx, |state, _| {
                if request.revision != state.revision {
                    return false;
                }
                state.pending.push(request);
                if state.pending_scheduled {
                    false
                } else {
                    state.pending_scheduled = true;
                    true
                }
            })
            .unwrap_or(false);
        if !scheduled {
            return;
        }
        window.defer(cx, move |window, cx| {
            let _ = weak.update(cx, |state, cx| {
                state.pending_scheduled = false;
                let candidates = std::mem::take(&mut state.pending);
                let Some(request) = arbitrate(
                    &candidates,
                    state.revision,
                    projection(state.kind, &state.model, &state.presentation)
                        .panel
                        .as_ref(),
                ) else {
                    return;
                };
                let origin = request.origin.clone();
                let event = if request.flyout {
                    NavigationEvent::FlyoutOpenChange { open: request.open }
                } else {
                    NavigationEvent::PanelChange {
                        id: if request.open { origin.clone() } else { None },
                    }
                };
                cx.emit(event);
                let weak = cx.entity().downgrade();
                window.on_next_frame(move |window, cx| {
                    let _ = weak.update(cx, |state, cx| {
                        // A rejected opening may have focused upstream's mechanical popup.
                        // Restore to its still-mounted trigger before the controlled render closes it.
                        let projected = projection(state.kind, &state.model, &state.presentation);
                        if request.open
                            && !request.flyout
                            && projected.panel != origin
                            && let Some(id) = origin.as_ref()
                        {
                            let scope: SharedString = format!("panel:{}", id.as_str()).into();
                            if state
                                .scopes
                                .get(&scope)
                                .is_some_and(|h| h.contains_focused(window, cx))
                            {
                                if let Some(h) = state
                                    .focus
                                    .get(id)
                                    .filter(|_| projected.controls.contains(id))
                                {
                                    h.focus(window, cx);
                                } else {
                                    state.fallback.focus(window, cx);
                                }
                            }
                        }
                        if request.open
                            && request.flyout
                            && !state.presentation.flyout_open
                            && state.scopes["flyout"].contains_focused(window, cx)
                        {
                            state.scopes["trigger"].focus(window, cx);
                        }
                        cx.notify();
                    });
                });
            });
        });
    }
}
impl NavigationModel {
    fn index_contains_panel(&self, key: &str) -> bool {
        key.strip_prefix("panel:")
            .and_then(|s| NavigationId::new(s.to_owned()).ok())
            .and_then(|id| self.item(&id))
            .is_some_and(|n| matches!(n, NavigationNode::Branch(_)))
    }
}
fn arbitrate(
    candidates: &[PopupRequest],
    revision: u64,
    panel: Option<&NavigationId>,
) -> Option<PopupRequest> {
    candidates
        .iter()
        .rev()
        .find(|r| r.revision == revision && r.open)
        .or_else(|| {
            candidates
                .iter()
                .find(|r| r.revision == revision && (r.flyout || r.origin.as_ref() == panel))
        })
        .cloned()
}
/// Collision-free length-prefixed namespace used by controls and popup regions.
pub fn navigation_element_id(
    instance: &NavigationId,
    node: Option<&NavigationId>,
    part: &str,
) -> SharedString {
    let n = node.map_or("", NavigationId::as_str);
    format!(
        "{}:{}{}:{}{}:{}",
        instance.as_str().len(),
        instance.as_str(),
        n.len(),
        n,
        part.len(),
        part
    )
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn id(s: &str) -> NavigationId {
        NavigationId::new(s.to_owned()).unwrap()
    }
    fn tree() -> Arc<NavigationModel> {
        Arc::new(
            NavigationModel::new(vec![
                NavigationGroup::new(
                    id("work"),
                    "Work",
                    vec![
                        NavigationBranch::new(
                            id("projects"),
                            "Projects",
                            vec![NavigationItem::new(id("leaf"), "Project").into()],
                        )
                        .into(),
                        NavigationBranch::new(
                            id("other"),
                            "Other",
                            vec![NavigationItem::new(id("other-leaf"), "Other leaf").into()],
                        )
                        .into(),
                    ],
                )
                .into(),
            ])
            .unwrap(),
        )
    }
    #[test]
    fn projection_preserves_stale_inputs_and_descendant_rules() {
        let model = tree();
        let p = NavigationPresentation::default()
            .with_selected(Some(id("missing")))
            .with_open_panel(Some(id("projects")));
        assert!(
            projection(NavigationKind::Sidebar, &model, &p)
                .panel
                .is_none()
        );
        let compact = projection(NavigationKind::Sidebar, &model, &p.clone().collapsed(true));
        assert_eq!(compact.panel, Some(id("projects")));
        assert!(compact.controls.contains(&id("leaf")));
        assert!(!compact.controls.contains(&id("other-leaf")));
        let nested = NavigationPresentation::default().with_open_panel(Some(id("leaf")));
        assert!(
            projection(NavigationKind::Navbar, &model, &nested)
                .panel
                .is_none()
        );
        assert_eq!(p.selected().unwrap().as_str(), "missing");
    }
    #[test]
    fn deferred_open_wins_dismiss_and_stale_callbacks_are_discarded() {
        let a = id("a");
        let b = id("b");
        let close = PopupRequest {
            revision: 7,
            origin: Some(a.clone()),
            open: false,
            flyout: false,
        };
        let open = PopupRequest {
            revision: 7,
            origin: Some(b.clone()),
            open: true,
            flyout: false,
        };
        for candidates in [
            vec![close.clone(), open.clone()],
            vec![open.clone(), close.clone()],
        ] {
            assert_eq!(
                arbitrate(&candidates, 7, Some(&a)).unwrap().origin,
                Some(b.clone())
            );
        }
        assert!(arbitrate(std::slice::from_ref(&close), 8, Some(&a)).is_none());
        assert!(arbitrate(std::slice::from_ref(&close), 7, Some(&b)).is_none());
        assert!(!arbitrate(&[close], 7, Some(&a)).unwrap().open);
    }
    #[test]
    fn namespace_is_unambiguous_for_separator_like_ids() {
        assert_ne!(
            navigation_element_id(&id("a:b"), Some(&id("c")), "item"),
            navigation_element_id(&id("a"), Some(&id("b:c")), "item")
        );
        assert_ne!(
            navigation_element_id(&id("a"), Some(&id("b")), "popup"),
            navigation_element_id(&id("other"), Some(&id("b")), "popup")
        );
    }
}
