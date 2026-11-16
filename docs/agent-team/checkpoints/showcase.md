# Checkpoint: showcase

Updated: 2026-11-16T15:11:47.359708+07:00. Run and runtime provenance: ../state.json. Workspace: /Users/ojiepermana/Development/ojiepermana/gpui. Branch: unborn main, no base commit. Existing source remains untracked. Only Leader coordination files were written. No integration or commit occurred.

## Static evidence returned by native helper

* AC-6: apps/showcase/src/lib.rs:47 defines seven destinations, and :958 dispatches them. State and subscriptions are retained at :136, :169, :221 and :261; :278 restores shell focus on navigation. Playground controls begin at :793, icon controls at :776, and seven layout choices at :653.
* AC-7: the three example packages are workspace members. docs/getting-started/showcase.md documents local consumption. scripts/check-release.py validates version without publishing; docs/contributing/releases.md documents deliberate release work.

Evidence limit: apps/showcase/tests/workbench.rs:176 checks section selection and shared status, plus Documentation controls, rather than asserting unique content on every destination. The Leader inspected distinct native page content separately. The headless icon clipboard test at :84 is separate from the native copy/paste observation. No definite missing implementation was found.

The helper returned completed analysis, wrote no files and created no workers.
