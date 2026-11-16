---
type: "query"
date: "2026-11-16T08:13:26.691543+00:00"
question: "Yes, trace why App bridges selection lifecycle, input state, dock tabs, and the other communities."
contributor: "graphify"
outcome: "useful"
source_nodes: ["App", "WindowSelectionState", "InputBaseState<M>", "TabGroupContext", "GlobalState", "ActiveTheme"]
---

# Q: Yes, trace why App bridges selection lifecycle, input state, dock tabs, and the other communities.

## Answer

Expanded from original query via graph vocabulary: [app, selection, lifecycle, input, dock, tab, global, state]. Queried the existing graph, then inspected App's exact neighborhood.

App is a hub because many APIs accept the same application context. It has 702 graph connections across 161 communities (160 outside its assigned community). Of those, 683 (97%) are parameter_type references; 697 total are references, three imports_from, one implements and one method. All 702 edges are tagged EXTRACTED. These are mainly shared type dependencies; they do not establish a runtime selection-to-dock call chain.

Representative paths: WindowSelectionState.finish_frame -> App via parameter_type at vendor/gpui-base/src/text_selection.rs source_location L1523; InputBaseState<M>.has_selection_focus -> App via parameter_type at vendor/gpui-base/src/input/base/state.rs L927; TabGroupContext.select_tab -> App via parameter_type at vendor/gpui-base/src/dock/tab_group.rs L849. App's assigned Selection App Lifecycle community contributes 43 direct neighbors; Input State Context six; Dock Tab Groups twenty. Additional input-related communities exist, including Input Element Rendering with 22 direct neighbors.

A concrete selection call path is WindowSelectionState.finish_frame -> clear_state (call at text_selection.rs L1526), then clear_state -> stop_anchor_auto_scroll (call at L1762). The last hop crosses from Selection App Lifecycle into Text Selection State. The graph records these calls but does not prove unconditional execution or a full event sequence.

The global/theme connection is also explicit: GlobalState.global and global_mut reference App at global_state.rs L95 and L99. App implements ActiveTheme at theme.rs L53 and its theme method calls Theme.global at L56. This supports interpreting App as shared framework context. Its placement in the Selection App Lifecycle community is a clustering result, not an ownership claim. This is a graph-only trace and not runtime verification.

Follow-up: trace how removing a selection participant leads to cleanup and stopping auto-scroll.

## Outcome

- Signal: useful

## Source Nodes

- App
- WindowSelectionState
- InputBaseState<M>
- TabGroupContext
- GlobalState
- ActiveTheme