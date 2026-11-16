---
type: "query"
date: "2026-11-16T08:20:07.766802+00:00"
question: "Yes, trace how is_rendered() distinguishes an unmounted selection participant from one merely outside the viewport."
contributor: "graphify"
outcome: "useful"
source_nodes: ["TextSelectionRegistration", "RenderedMarker", ".is_rendered()", ".with_rendered_element()", ".stale_participants()", "schedule_selection_frame()"]
---

# Q: Yes, trace how is_rendered() distinguishes an unmounted selection participant from one merely outside the viewport.

## Answer

Expanded from original query via graph vocabulary: [rendered, unmounted, selection, participant, viewport, registration, visible, retained]. The graph established stale_participants -> TextSelectionRegistration.is_rendered and a registration dependency on WeakEntity<RenderedMarker>, but did not capture the boolean predicate. Inspected the pointed local source locations to resolve the requested viewport distinction; the details below are source-verified, not additional invented graph edges.

is_rendered is a retained-element lifetime check, not a visibility check. At vendor/gpui-base/src/text_selection.rs L276-L280 it returns self.rendered.as_ref().is_some_and(|marker| marker.upgrade().is_some()). with_rendered_element at L253 creates a RenderedMarker using window.use_keyed_state, keyed by text-selection-participant plus participant entity ID, and stores a downgraded weak reference. The documentation at L207-L213 describes retained cached-frame replay keeping the marker alive even when the element's paint does not run. The marker dies when GPUI drops the retained element.

Consequences: cached or offscreen/clipped elements whose marker remains alive return true, regardless of viewport position; actual element removal or virtualized removal that drops the marker returns false; merely retaining the TextSelectionHandle does not keep the separate rendered-element marker alive. An element being outside the viewport is not itself proof that its marker survives: virtualization may remove retained element state.

stale_participants at L1537-L1551 removes a participant if its entity is dead, or if generation was missed AND !is_rendered AND (full_frame OR a marker had been attached). Registrations without with_rendered_element have no marker; they use the full-frame fallback when a generation is missed, as documented at L247-L252. For an endpoint departure the cleanup chain described previously applies, including stop_anchor_auto_scroll before clearing the anchor.

Correction/clarification to preceding graph traces: finish_frame at L1523 is cfg(test), not the production sweep entry. Production schedule_selection_frame at L3189-L3206 uses window.defer and stale_participants(layer_painted). Local source also confirms stop_anchor_auto_scroll at L2737-L2742 first calls self.auto_scroll.stop() and then sets the live anchor participant's auto-scroll to None; clear_state calls this at L1762 before clearing anchor at L1763.

Source validation only; no tests or native UI verification performed in this turn. A regression test at text/text_view.rs L4155 asserts copying clipped-out charlie/delta text; this confirms the test contract and is not a runtime pass claim.

Follow-up: trace the virtualization boundary where an offscreen participant's retained state is actually dropped.

## Outcome

- Signal: useful

## Source Nodes

- TextSelectionRegistration
- RenderedMarker
- .is_rendered()
- .with_rendered_element()
- .stale_participants()
- schedule_selection_frame()