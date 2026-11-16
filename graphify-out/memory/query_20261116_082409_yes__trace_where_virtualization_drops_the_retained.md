---
type: "query"
date: "2026-11-16T08:24:09.993989+00:00"
question: "Yes, trace where virtualization drops the retained element state and its RenderedMarker."
contributor: "graphify"
outcome: "useful"
source_nodes: ["VirtualList", "TextView", "TextSelectionRegistration", "RenderedMarker", ".with_rendered_element()", ".is_rendered()", ".stale_participants()"]
---

# Q: Yes, trace where virtualization drops the retained element state and its RenderedMarker.

## Answer

Expanded from original query via graph vocabulary: [virtual, retained, element, state, marker, rendered, scroll, visible]. Queried the existing graph, then verified local GPUI Kit source and the Cargo.lock-pinned gpui-pre 0.3.12 source in the Cargo registry package gpui-pre 0.3.12 (locate its manifest_path using cargo metadata --locked --format-version 1). The runtime implementation is outside the scanned workspace graph; runtime details below are source-verified additions, not inferred graph edges.

The actual drop happens in GPUI Window.draw's old-frame cleanup, not an explicit RenderedMarker deletion in the virtualizer. Two virtualization levels must be distinguished. TextView's internal list renders document blocks (vendor/gpui-base/src/text/document.rs L278), while the outer TextView.paint still calls adapter.register at text/text_view.rs L1042. That attaches with_rendered_element at text/selection_adapter.rs L360. Scrolling internal blocks out therefore does not, by itself, remove the outer TextView participant marker. Whole-TextView virtualization by an enclosing list can omit the marker owner.

For the workspace VirtualList, prepaint computes visible_range at virtual_list.rs L704 and calls render_items only for that range at L706; layout/prepaint at L733-L735 and paint at L763 only traverse returned items. GPUI's own list also constructs a selected item_layouts set, with an explicit offscreen focused-item exception at runtime elements/list.rs L1240-L1258. Thus leaving viewport alone is insufficient: the whole participant must leave the drawn/replayed set.

The retained-state ownership chain:
1. with_rendered_element creates a keyed Entity<RenderedMarker> in window.use_keyed_state and stores only a WeakEntity in registration (text_selection.rs L253-L269).
2. Runtime use_keyed_state at window.rs L4587-L4603 stores the strong entity through with_element_state. take_element_state at L7606-L7617 records accessed state keys and retrieves prior state; insert_element_state at L7622-L7627 stores next-frame state.
3. Cached rendering preserves keys: reuse_prepaint at L4256 and reuse_paint at L4337 copy accessed_element_states into next_frame. This is why skipping paint through cached replay does not imply unmount.
4. If the whole participant is omitted and not replayed, its marker key is not accessed. Frame.finish at window.rs L1199-L1205 migrates only accessed old states.
5. Window.draw at L3745-L3751 finishes next frame, swaps rendered_frame and next_frame, then clears the old frame. Frame.clear at L1126-L1127 clears element_states. This drops the marker's stored strong entity.
6. Runtime AnyEntity.drop at app/entity_map.rs L440-L453 decrements the count and queues final release. WeakEntity upgrade fails once count is zero (L693-L701), even before the deferred entity payload cleanup finishes. Hence TextSelectionRegistration.is_rendered at text_selection.rs L276 becomes false.
7. Deferred schedule_selection_frame at text_selection.rs L3196-L3206 sweeps missed-generation participants whose marker died, and endpoint departures trigger clear_state. clear_state at L1762 stops auto-scroll before clearing anchor L1763.

Inline RetainedLayout cleanup is a separate cache and does not own RenderedMarker. No source edits, runtime tests, or native/accessibility verification were performed.
Follow-up: verify a whole TextView being virtualized out in a focused regression test.

## Outcome

- Signal: useful

## Source Nodes

- VirtualList
- TextView
- TextSelectionRegistration
- RenderedMarker
- .with_rendered_element()
- .is_rendered()
- .stale_participants()