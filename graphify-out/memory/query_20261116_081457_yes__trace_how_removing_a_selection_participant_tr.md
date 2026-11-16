---
type: "query"
date: "2026-11-16T08:14:57.483383+00:00"
question: "Yes, trace how removing a selection participant triggers cleanup and stops auto-scroll."
contributor: "graphify"
outcome: "useful"
source_nodes: ["WindowSelectionState", ".finish_frame()", ".stale_participants()", ".endpoint_departed()", ".clear_state()", ".stop_anchor_auto_scroll()", ".set_auto_scroll()", "AutoScroll"]
---

# Q: Yes, trace how removing a selection participant triggers cleanup and stops auto-scroll.

## Answer

Expanded from original query via graph vocabulary: [selection, participant, removing, unmount, clear, stop, auto, scroll]. Traversed the existing graph and inspected exact call-edge neighborhoods.

The graph points to endpoint-sensitive cleanup during frame sweeping. WindowSelectionState.finish_frame calls stale_participants (text_selection.rs source_location L1524), endpoint_departed (L1525), clear_state (L1526), remove_stale_participants (L1530), clear_departed_participants (L1531), finish_registrations (L1532), and publish_snapshots (L1533). stale_participants calls TextSelectionRegistration.is_rendered at L1548. The production schedule_selection_frame function contains a parallel path: stale_participants L3200, endpoint_departed L3205, clear_state L3206, remove_stale_participants L3210, clear_departed_participants L3211, then finishing registrations and snapshot publication.

clear_state calls stop_anchor_auto_scroll at L1762, TouchSelection.reset at L1769, touch_changed at L1770, prune_dead_participants at L1772, and participant update at L1777. stop_anchor_auto_scroll (definition L2737) calls anchor_participant at L2739 and updates the participant via set_auto_scroll at L2742. SelectableTextState.set_auto_scroll (definition L844) connects into the AutoScroll implementation at L845. The relevant edges are EXTRACTED. The call path crosses Text Selection State, Selection App Lifecycle, and Automatic Scrolling communities.

Regression-test names document intended contracts: removing_an_unselected_participant_preserves_the_current_selection L4620; removing_either_range_endpoint_discards_the_shared_selection L4643; clear_or_unmount_stops_auto_scroll_before_discarding_the_anchor L4673; unmounted_selection_does_not_revive_when_the_participant_returns L4582. The graph includes their calls to finish_frame. These tests were not executed in this trace. The graph does not retain condition expressions, setter argument values, or assertions, so endpoint branch behavior and stop-before-discard intent are indicated by method names, source-location ordering, and test names rather than verified runtime behavior.

Follow-up: trace how registration and is_rendered distinguish an unmounted participant from one outside the viewport.

## Outcome

- Signal: useful

## Source Nodes

- WindowSelectionState
- .finish_frame()
- .stale_participants()
- .endpoint_departed()
- .clear_state()
- .stop_anchor_auto_scroll()
- .set_auto_scroll()
- AutoScroll