---
type: "query"
date: "2026-11-16T08:38:28.671638+00:00"
question: "What regression verifies selection marker expiry and auto-scroll cleanup when a whole TextView is virtualized out?"
contributor: "graphify"
outcome: "useful"
source_nodes: ["RenderedMarker", "TextView", "VirtualList", "WindowSelectionState"]
---

# Q: What regression verifies selection marker expiry and auto-scroll cleanup when a whole TextView is virtualized out?

## Answer

Added whole_text_view_virtualization_expires_marker_and_stops_auto_scroll in vendor/gpui-base/src/text_selection.rs, inside the existing cfg(test) module. It uses real TextView and VirtualList elements, real held mouse input, frame drawing, and production deferred selection cleanup. The test retains text and selection entities while row 0 leaves the virtualized range. It verifies old RenderedMarker weak-reference expiry, stale registration removal, selection and gesture cleanup, window auto-scroll deactivation and cleared drag position, AutoScroll(None) delivery to the anchor, and creation of a fresh marker without resurrecting selection on remount. A retained-row control verifies that marker and active auto-scroll survive before removal.

Focused run on 2026-11-16: 1 passed, 0 failed, 0 ignored. Command from repository root: CARGO_TARGET_DIR="$PWD/target" CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_TEST_INCREMENTAL=false cargo test --manifest-path vendor/gpui-base/Cargo.toml --locked --features test-support --lib whole_text_view_virtualization_expires_marker_and_stops_auto_scroll.

Only test code was added to the vendor source. vendor/gpui-base/selection-virtualization-test.patch records the complete addition, vendor/gpui-base/source-integrity.json records the exact source hash, and vendor/gpui-base/PATCHES.md documents reproduction. Source integrity, workspace build, and docs passed in that run; formatting, Clippy and workspace test compilation encountered unrelated navigation edits. No native GPU or accessibility verification was performed. This verification record supplements the older graph snapshot; it does not assert the regression node is already indexed.

## Outcome

- Signal: useful

## Source Nodes

- RenderedMarker
- TextView
- VirtualList
- WindowSelectionState