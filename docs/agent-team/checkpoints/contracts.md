# Checkpoint: contracts

Updated: 2026-11-16T15:11:47.359708+07:00. Run and runtime provenance: ../state.json. Workspace: /Users/ojiepermana/Development/ojiepermana/gpui. Branch: unborn main, no base commit. Existing source remains untracked. Only Leader coordination files were written. No integration or commit occurred.

## Static evidence returned by native helper

No definite missing implementation or contradiction was found. This is source evidence, not runtime proof.

* AC-1: Cargo.toml:1 and :20 define workspace and versioned local paths; apps/showcase/Cargo.toml:12 consumes ui-kit.
* AC-2: Cargo.toml:15 pins gpui-kit; crates/ui-components/src/lib.rs:9 re-exports upstream; crates/ui-kit/src/lib.rs:10 exports its runtime; Cargo.lock:2608 and :2640 resolve 0.9.1 and gpui-pre 0.3.12. Cargo.toml:43 declares the documented GPUI Base patch.
* AC-3: crates/ui-theme/src/lib.rs:31, :85 and :207 define validated serializable themes and registration; :293, :322, :334 and :352 implement selection, system sync and token projection. apps/showcase/src/lib.rs:221 retains the native appearance observer. Detailed upstream projection limits remain documented in docs/architecture/compatibility.md:52.
* AC-4: crates/ui-icons/src/lib.rs:94, :170 and :227 define typed identifiers and validated registration; :200 calls upstream rendering. Showcase Button icon integration is at apps/showcase/src/lib.rs:329 and :390.
* AC-5: crates/ui-layout/src/lib.rs:16, :89 and :152 define validated shell regions and responsive visibility; :223 and :242 supply navigation/dashboard; :247 and :288 use upstream panel resizing; :315 supplies caller controlled tabs.

The helper sent its initial boundary confirmation, received a Leader reply, and returned completion. It wrote no files and created no workers.
