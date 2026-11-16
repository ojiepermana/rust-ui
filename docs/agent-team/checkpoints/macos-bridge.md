# macOS selected/expanded bridge follow-up

Date: 2026-11-16, Asia/Jakarta. Targeted bridge repair and re-verification passed. This does not accept every remaining spec 0002 scenario or certify a screen reader.

## Authorization and boundaries

The user explicitly requested: “Lanjutkan perbaikan bridge macOS agar selected/expanded terekspos tanpa upgrade runtime, lalu verifikasi ulang.” This overrides the earlier no-vendor-change constraint only for the targeted metadata bridge. The architect amendment records that exception in spec 0002 AC-16. The overall feature stays In Progress for unchecked scenarios. No commit, push, publication or release was performed.

The repair ran inline through debug, followed by focused architect/sync maintenance. No new worker was spawned. The active session model/effort was unchanged and is not inferred from historical worker selections.

## Proven root cause and fix

Published accesskit_macos 0.26.3 allowed isAccessibilitySelected only on item-like roles. It did not implement or allow the optional expanded getter. Its event generator also omitted these read-only push-button states.

Only vendor/accesskit_macos/src/node.rs and src/event.rs change production behavior. Explicit Button/DefaultButton selection now has a read-only native getter. Existing item selection/setter behavior and Tab radio/value mapping remain intact. No checkbox role or AXValue is synthesized. Expanded reads its optional AccessKit field and is exposed only when supplied. Changes queue the existing native value-change notification once per node, including combined field changes; equivalent updates are silent.

The dependency override retains accesskit_macos 0.26.3. GPUI Kit 0.9.1, GPUI 0.3.12 and every other locked package version remain unchanged. Cargo.lock only removes the registry source/checksum for the same package, now provided by its documented local path. The original published archive SHA256 is pinned in source-integrity.json, with exact original, modified and added file inventories. Both existing gpui-base patches are untouched.

## Native ABI regression

`cargo test -p accesskit_macos --test native_bridge --locked` runs an explicit harness=false binary on the actual macOS main thread. It includes the production adapter modules and invokes the real Objective-C selectors on native PlatformNode objects. It also uses the production event generator. It does not copy a mapper, substitute a renderer, inspect an external application or bypass UI automation.

Six cases cover selected true/false, disabled Button state, real AXButton role without toggle value, expanded true/false, read-only setters/actions, unsupplied attributes, attribute removal/role changes, existing item actions/disabled behavior, Tab values, live notifications/coalescing and no-op updates.

The final test source was copied to /tmp/gpui-macos-bridge-control with only node.rs/event.rs restored byte-for-byte from the published archive. Cargo's command-line path override selected this negative control with the same lockfile and no version change. It failed exactly three cases: selected exposure, expanded exposure and state notifications. The actual patch passed all six immediately afterward. This is a native ABI test, not a headless GPUI metadata assertion.

Evidence: /tmp/gpui-macos-bridge-validation/negative-control.log, negative-control.json, after.log and 04-native-abi.log. The added CI macOS step runs this target separately because vendored dependencies are excluded from --workspace. Hosted CI has not run locally.

## Workspace validation

- `cargo fmt --all --check`: passed. Log `/tmp/gpui-macos-bridge-validation/01-fmt.log`.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: passed. Log `/tmp/gpui-macos-bridge-validation/02-clippy.log`.
- `cargo test --workspace --locked`: passed. Log `/tmp/gpui-macos-bridge-validation/03-test.log`.
- `cargo test -p accesskit_macos --test native_bridge --locked`: passed. Log `/tmp/gpui-macos-bridge-validation/04-native-abi.log`.
- `cargo clippy -p accesskit_macos --test native_bridge --locked -- -D warnings`: passed. Log `/tmp/gpui-macos-bridge-validation/05-native-clippy.log`.
- `cargo build --workspace --locked`: passed. Log `/tmp/gpui-macos-bridge-validation/06-build.log`.
- `cargo doc --workspace --no-deps --locked`: passed. Log `/tmp/gpui-macos-bridge-validation/07-doc.log`.
- `python3 scripts/validate-dependencies.py`: passed. Log `/tmp/gpui-macos-bridge-validation/08-dependencies.log`.

There are 45 workspace test/doctest cases plus 6 native ABI cases. These are separate commands, not 51 cases claimed from --workspace. The bridge regression target also passes Clippy with denied warnings. All package versions stayed fixed, original sources/provenance were checked, and the complete code/test patch applies in reverse check without mutation.

## Application observations

A dedicated unsigned BridgeVerification.app (display macOS Bridge Verification, identifier dev.rust-ui-kit.bridge-verification) was built from the current patched workspace and inspected through CUA on macOS 27.0.1/arm64. Executable SHA256 f89abebbcb7387159a7fff715ec9c1a4d0ce29afddbd14e089030fbac60b8c07. Screenshots/AX states were inspected inline; no screenshot export is claimed.

Observed app AX before activation: ordinary button (selected) Inbox, 120 unread messages; Work, Projects and the other unchosen controls are not selected. Activating Projects moved selected to Projects while Inbox and ancestor Work stayed unselected. Administration remained disabled. Controls retain Button role, not checkbox/toggle. CUA calls non-selected controls with an explicit false AXSelected “selectable”; the ABI regression separately proves the Button selection setter/AXPick are not allowed and only AXPress activates.

Flyout opened by pointer, Escape closed it, Enter reopened the same trigger, Escape then Space also reopened it, and Close menu closed it. Popup children showed Projects selected and Work unselected. This rechecks focus/keyboard behavior after the adapter notification changes.

AXExpanded evidence is specifically the native Objective-C getter and live true/false transitions in the six-case regression. CUA's app-tree string does not print AXExpanded for Button. Apple Accessibility Inspector was opened with the correct process selected, but its point-to-inspect control could not capture a node through available CUA pointer actions. No Inspector AXExpanded capture is claimed. The Inspector reticle was disabled, then Inspector and the dedicated bridge app were closed; the pre-existing Showcase was left running.

## Remaining limits

VoiceOver speech/order, other operating systems, hosted CI and the other previously unchecked navigation scenarios remain unverified. The targeted fix passes at the native adapter ABI and selected is observed in the real application tree. This is not a blanket accessibility compliance or full feature acceptance claim.

## Delivery and maintenance

External Cargo consumers need the accesskit_macos override alongside gpui-base because patches are not transitive. The guide and Showcase Copy local dependency recipe include both. Patch provenance, complete diff, native regression instructions and removal criteria are in vendor/accesskit_macos/PATCHES.md. The root validator checks its version/path and all source hashes/inventory; CI includes the macOS ABI test.
