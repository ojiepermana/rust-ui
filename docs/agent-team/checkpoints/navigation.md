> Follow-up, 2026-11-16: the user authorized a same-version macOS bridge patch. The selected/expanded adapter limitation below is resolved by the [targeted repair](macos-bridge.md). Native app selected and native ABI expanded evidence are recorded separately there. Other unchecked scenarios remain open.

# Native navigation validation

Date: 2026-11-16, Asia/Jakarta. Implementation is complete; spec 0002 remains In Progress because native accessibility acceptance is blocked and the exhaustive checklist is not fully executed. No navigation commit, push, publication or release was performed.

## Runtime and coordination

Codex desktop, Native Subagents, four available contexts including Leader. One implementing child and one read-only conformance helper, with dependency-driven dispatch. Implementation used explicit native_task_adaptation. Both requested gpt-6.1-sol, high for implementation and medium for conformance. Actual execution settings were not exposed; no substitutions. Leader-owned scope, integration, tests and sync ran inline with unchanged session settings, whose exact model/effort were unavailable. Advisory workflow efforts did not change the session. Auto policy came from user --session autodecision. No sidebar tree requirement or independent review claim.

## Source and validation boundaries

Initial source base 1287fc9. A separate task introduced and committed vendor selection work as 4308072 while this run was active. That work was preserved. Target implementation was first validated at /tmp/gpui-navigation-integration with original vendor/divider baseline and navigation-only source changes. Main checkout was then validated again after source integration, including its existing selection patch. No vendor source was changed by navigation. Isolated validator expectation retained the original one-patch set; main validator retained the external two-patch set.

Test-writing inputs were the selected navigation source/facade/Showcase/consumer changes. Pre-existing graphify output and sibling vendor work were excluded from target writing. Project-wide command execution additionally covers the current checkout. Existing foundation evidence is retained in the archived prior run, not rewritten as new evidence.

## Final automated gates

- `cargo fmt --all --check`: passed, log `/tmp/gpui-navigation-validation/checkout-01-fmt.log`.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: passed, log `/tmp/gpui-navigation-validation/checkout-02-clippy.log`.
- `cargo test --workspace --locked`: passed, log `/tmp/gpui-navigation-validation/checkout-03-test.log`.
- `cargo build --workspace --locked`: passed, log `/tmp/gpui-navigation-validation/checkout-04-build.log`.
- `cargo doc --workspace --no-deps --locked`: passed, log `/tmp/gpui-navigation-validation/checkout-05-doc.log`.
- `python3 scripts/validate-dependencies.py`: passed, log `/tmp/gpui-navigation-validation/checkout-06-dependencies.log`.

**45 test/doctest cases passed**, zero failures or ignored cases. The new crate has 18 cases, the full gallery has three added integration cases, the facade has one added contract case, and the new consumer has one actual AppShell resize case. The headless matrix runs 2 themes × 2 densities × 3 rem sizes × 2 reduced-motion flags × 5 patterns, checking actual controls and complete containment in the navigation region.

`cargo run -p showcase --locked` reached the Running command and remained alive for the smoke interval, then was stopped deliberately by Ctrl-C (exit130). Native window evidence below comes from separately identified bundles. This stopped process is not reported as a failed build or as proof of a screen reader.

## Native macOS observations

Host macOS 27.0.1, Apple Silicon. Configured client size1200×820 logical pixels; screenshots include native titlebar. CUA operated a dedicated Navigation Verification app, identifier dev.rust-ui-kit.navigation-verification, avoiding another task's Showcase. Screenshots and AX states were inspected inline, not exported.

Observed across the final crate and focused followups: Sidebar expanded and compact; nested branch popup; Dockbar left Inline and right Overlay; Minidock fixed rail and complete New/99+ badge; Navbar horizontal controls without vertical clipping and its Work popup; Flyout pointer opening and nested contents. Native Projects activation changed application content once; clicking disabled Billing did not increment activations. Updating data inserted Reports with Re fallback and changed Inbox badge/name120→7. Empty rail uses a short Empty label, without vertical word splitting. Light and dark app surfaces were inspected; OS appearance was not changed.

A real native pointer-focus bug was fixed after initial checks: Escape restored the old story picker. Raw no-frame-between down/up replay reproduced it. The final native sequence pointer-open→Escape→Enter reopened the same Flyout, proving focus returned to its trigger. Ctrl+Tab moved from Components to Themes, and Ctrl+Shift+Tab returned to the retained Navigation gallery. Final app is left open on Sidebar, System appearance, populated data.

Native bundle identities and SHA256 records are in /tmp/gpui-navigation-validation/native-bundle.json. Final current-checkout executable SHA256371a1cd086a88692b698f8a0593c990bd4037e1d8c9293939da48605fd6355df. Previous isolated focus-fix bundle SHA256ddc263755b99d835cd0f4e417b879ad672866d0bf48d53265d936e610bbf7375. Initial screenshots with clipping/focus defects are not counted as passing final evidence.

## Initial native accessibility blocker (resolved by follow-up)

Expected: ordinary Button role, selected only on chosen leaf, expanded on branch/popup trigger, without toggle substitution. GPUI metadata tests assert these fields and no toggled state. Native CUA exposed names, IDs, Button/Heading/Dialog roles and disabled controls, including Inbox, 120 unread messages, but did not expose selected/expanded on these Buttons.

Pinned source inspected: accesskit_macos0.26.3 src/node.rs lines1031–1037 restrict isAccessibilitySelected to item-like roles. The adapter has no expanded mapping for these controls. This is a pinned native bridge boundary, not an excuse to infer accessibility from color or headless state. Per spec, AC-12/AC-14 are not marked complete; no role substitution, upgrade or vendor patch was added. VoiceOver output was not tested.

## Criterion ledger

| Criterion | Result | Evidence/boundary |
| --- | --- | --- |
| AC-1 | passed | 12 packages, seven libraries, facade-only normal consumer; locked runtime and boundaries checked. |
| AC-2 | passed | Pure validation plus atomic configure/model/kind regression. |
| AC-3 | passed locally | Request-only state, no-op, arbitration, stale callbacks and rejected popup reconciliation tests. |
| AC-4 | passed observed subset | Native expanded/collapsed Sidebar, nested branch, count, disabled and popup surfaces. |
| AC-5 | passed locally | Flat validation, fixed rail, native leaf activation/disabled rejection, spacer geometry. |
| AC-6 | passed observed subset | Native left Inline and right Overlay; earlier right Inline observation, source sides share composition. Full combination matrix not claimed. |
| AC-7 | passed locally | Native horizontal bar/popup; long bar programmatic focus scrolling and rem geometry tested headlessly. |
| AC-8 | passed observed subset | Native pointer open, Escape close, Enter reopen; nested content visible. Remaining checklist close/rapid scenarios not all driven natively. |
| AC-9 | passed automated subset | Collision-free namespace, stale projection, no-op handles and reorder/model-update focus. Two-instance manual scenario unexecuted. |
| AC-10 | passed observed subset | 99+ visible, original120 native name, zero omitted, text New and missing-icon Re fallback, updated7 name. |
| AC-11 | passed automated subset | Spacer/scroll geometry, fixed slot structure, bounded popup implementation. Long popup/scroll viewport native stress remains unchecked. |
| AC-12 | blocked for native metadata | Headless light/dark × both density × 16/20/24 rem × reduced-motion flags passes full control bounds. Native light/dark observed. Pinned macOS selected/expanded gap remains. |
| AC-13 | passed observed subset | Raw pointer focus restoration regression and native Escape/Enter proof; headless removal/modal/slot resize handoff. Full native resize/unmount matrix not claimed. |
| AC-14 | blocked for native metadata | Native keyboard activation/dismissal and standard upstream controls, but required native expanded/selected semantics unavailable on pinned macOS bridge. |
| AC-15 | passed | Facade contract, navigation-app compilation and real consumer AppShell resize slot focus test. |
| AC-16 | passed local regression suite | Layout APIs untouched; existing Showcase destinations, layout tests, split/tab/shortcuts remain green. |
| AC-17 | passed observed subset | Native five-pattern gallery, destination updates, disabled, nested, compact, empty, badges and live data replacement. |
| AC-18 | passed reporting/gates | All required commands pass; evidence types/platform limits separated. This is not full acceptance of unverified behavior. |

## Remaining evidence

Unchecked verify.md steps remain unclaimed, especially full native density/rem/reduced-motion and long-scroll/viewport stress, two-instance checks, native slot resize/unmount combinations, Windows/Linux/Intel macOS, physical multi-display DPI, IME and screen-reader output. The Beta independent review and release-document stage were not required and did not run. Runtime hierarchy is evidenced by native child paths and messages, not a sidebar tree.

The next acceptance decision concerns an upstream bridge capability or an explicit revision of that native contract. The current spec forbids silently upgrading/patching the runtime, so this run leaves the feature in progress rather than approving an incompatible workaround.

## Follow-up verification in the foundation chat

The user selected following the existing implementation and verifying its result here, rather than dispatching duplicate writers. The implementation chat completed before these observations. This follow-up used the delivered `dev.rust-ui-kit.navigation-verification` bundle and changed no application or vendor source. It is runtime verification, not an independent code review.

All 12 recorded source fingerprints match the current checkout, and the bundle executable matches recorded SHA256 `371a1cd086a88692b698f8a0593c990bd4037e1d8c9293939da48605fd6355df`. The recorded current-checkout test log was inspected and contains 45 passed, zero failed and zero ignored cases. The full suite was not repeated. Formatting, dependency/source-integrity validation and `git diff --check` were checked again successfully. Dependency validation confirmed 12 packages and one GPUI runtime.

Native screenshots and accessibility states were inspected inline in this chat, with no screenshot files exported:

| Check | Observed result |
| --- | --- |
| Sidebar | Projects activation changed content and selected ID to projects with one accepted activation. Collapsing produced the rail; Work opened a popup. Escape dismissed it and Enter reopened it from restored trigger focus. |
| Dockbar | Work opened an Inline panel on the left. Switching to right Overlay and activating Work opened its popup to the left of the rail. |
| Minidock | Fixed compact rail displayed complete 99+ and New badges; Billing was exposed as disabled and clicking it did not increment the activation count. Empty rendered as a short unbroken label. |
| Navbar | Horizontal controls fit in the visible region and Work opened a native popup. |
| Flyout | Pointer activation of Workspace menu opened nested content. Escape dismissed it; Enter reopened it, confirming trigger focus restoration. |
| Shell handoff | Ctrl+Tab reached Themes and Ctrl+Shift+Tab returned to the retained Navigation story. The app was left on populated expanded Sidebar with System selected. |
| Native metadata | Names, stable IDs, Button/Heading/Dialog roles and disabled state were visible. Selected Projects and expanded Work/Workspace menu did not expose selected/expanded properties in the native tree. The existing AC-12/AC-14 blocker remains. |

No new functional failure was observed in this subset. Unchecked `verify.md` steps, the full native matrix, native accessibility acceptance and other platform/screen reader limits remain open. No scope box or spec status was advanced, and no commit or push was performed by this follow-up.
