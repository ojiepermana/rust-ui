# GPUI Base 0.9.1 compatibility patch

Source: the crates.io gpui-base 0.9.1 package, package SHA-256 `f3e9ae5237737cfd110d0ff513fcd9c20f97d98ebabd0a73164068177f0aca4f`. Original source provenance is retained in .cargo_vcs_info.json. License: Apache-2.0, retained unchanged in LICENSE-APACHE.

Only src/resizable/panel.rs changes production behavior. coalesced-divider.patch records that complete code delta against the published package. src/text_selection.rs also adds a test fixture and regression case inside its existing cfg(test) module, recorded in selection-virtualization-test.patch. The package remains version 0.9.1 and uses its original exact GPUI 0.3.12 dependencies. No component implementation or resize arithmetic is replaced.

The drag constructor previously set the active panel index but discarded the move that started the drag. If macOS delivers down, one held move, then up, the pane never changes width. The patch applies that first pointer sample through the existing resize_panel_at_handle engine, preserving constraints and redistribution. Subsequent moves and release keep their existing handlers.

Regression: apps/showcase/src/tests.rs, coalesced_native_divider_drag_applies_the_first_move. It replays the captured single move gesture without helper-inserted events or frames and failed against the published package.

## Selection virtualization regression

`text_selection::tests::whole_text_view_virtualization_expires_marker_and_stops_auto_scroll` draws a real TextView inside an enclosing VirtualList and starts a held selection drag with active ancestor auto-scroll. It retains the text model, selection handle and old registration while scrolling the entire row out of the rendered range. It asserts that the old RenderedMarker expires, the registration is swept, the gesture and selected text clear, window auto-scroll stops and the anchor receives AutoScroll(None). Scrolling back creates a different live marker without restoring the old selection or scrolling. A preceding frame checks that a still-rendered row preserves its marker and active auto-scroll.

Run this test separately because the vendored package is excluded from the root workspace:

```sh
CARGO_TARGET_DIR="$PWD/target" CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_TEST_INCREMENTAL=false \
  cargo test --manifest-path vendor/gpui-base/Cargo.toml --locked --features test-support \
  --lib whole_text_view_virtualization_expires_marker_and_stops_auto_scroll
```

Passed on 2026-11-16 with the pinned GPUI 0.3.12 test platform: one case, zero failures or ignored cases. This is headless element lifecycle and input coverage, not native GPU or accessibility verification. No production selection code changed. source-integrity.json records the exact hash of the test-only source addition while retaining its original package hash.

The workspace patches crates.io gpui-base to this directory. Cargo patches are not transitive: external consumers need the same patch in their own workspace until an upstream fixed release is selected. Before public registry publication, use an upstream release containing the fix and remove this vendor override. Do not update unrelated vendored files or reformat this directory.
