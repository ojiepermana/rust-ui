# GPUI Base 0.9.1 compatibility patch

Source: the crates.io gpui-base 0.9.1 package, package SHA-256 `f3e9ae5237737cfd110d0ff513fcd9c20f97d98ebabd0a73164068177f0aca4f`. Original source provenance is retained in .cargo_vcs_info.json. License: Apache-2.0, retained unchanged in LICENSE-APACHE.

Only src/resizable/panel.rs is changed. coalesced-divider.patch records the complete code delta against the published package. The package remains version 0.9.1 and uses its original exact GPUI 0.3.12 dependencies. No component implementation or resize arithmetic is replaced.

The drag constructor previously set the active panel index but discarded the move that started the drag. If macOS delivers down, one held move, then up, the pane never changes width. The patch applies that first pointer sample through the existing resize_panel_at_handle engine, preserving constraints and redistribution. Subsequent moves and release keep their existing handlers.

Regression: apps/showcase/src/tests.rs, coalesced_native_divider_drag_applies_the_first_move. It replays the captured single move gesture without helper-inserted events or frames and failed against the published package.

The workspace patches crates.io gpui-base to this directory. Cargo patches are not transitive: external consumers need the same patch in their own workspace until an upstream fixed release is selected. Before public registry publication, use an upstream release containing the fix and remove this vendor override. Do not update unrelated vendored files or reformat this directory.
