# AccessKit macOS 0.26.3 metadata compatibility patch

User explicitly authorized this bridge fix on 2026-11-16, after native navigation verification found selected/expanded missing. GPUI Kit remains 0.9.1, GPUI remains 0.3.12, AccessKit macOS remains 0.26.3. No component renderer, role or domain selection policy is replaced.

## Provenance

Original crates.io archive SHA256 ce02dc63b43f0c9296af9ac946312a2dc8814427d7a64d2d600971dac55b6076. Published .cargo_vcs_info.json identifies AccessKit commit c88605b96d04431f9c3c792464a0f2f253480e94, platforms/macos. All original package files and hashes remain recorded in source-integrity.json. The archive omitted the license texts referenced by its source headers; Apache/MIT texts are retained from the pinned AccessKit repository and the Chromium BSD text from its official repository, with URLs and hashes in the same record.

## Production changes

Only src/node.rs and src/event.rs change production behavior. Native isAccessibilitySelected now accepts explicitly supplied selection on Button/DefaultButton while preserving the existing item selection and Tab value mapping. Selection on these push buttons is read only: AXPress remains their action, while AXSelected setters and AXPick remain restricted to existing selectable items. Disabled push buttons retain their supplied selection state without becoming enabled. No toggle value or checkbox role is synthesized.

Native isAccessibilityExpanded returns the explicit AccessKit expanded value. Its selector is allowed only when that optional property is supplied. Expanded remains application controlled, through normal button activation. Plain buttons do not acquire unsupplied states.

Changes to these read-only native states queue NSAccessibilityValueChangedNotification through the existing event path. Combined value/selected/expanded changes yield one notification per node; equivalent snapshots yield none. Existing item selection notifications and action behavior are preserved.

Cargo.toml adds only an explicit main-thread test target with harness=false; dependency declarations and versions are unchanged. tests/native_bridge.rs includes the actual production adapter modules and invokes their Objective-C selectors on native platform nodes, rather than copying a mapping or constructing a fake result. It also runs the production event generator. No external application or window is inspected by this test.

## Regression and validation

```sh
cargo test -p accesskit_macos --test native_bridge --locked
cargo clippy -p accesskit_macos --test native_bridge --locked -- -D warnings
```

The native main-thread regression has six cases covering supplied true/false values, live updates, absent attributes, removal/role changes, disabled state, Button role without toggle value, read-only versus activation actions, legacy list selection/Tab values, state-change notification coalescing and no-op updates. Against original production files, three cases fail: Button selection exposure, expanded exposure and notifications. All six pass after the patch. The ABI test runs in macOS CI separately because this vendored dependency is excluded from --workspace.

Native application AX inspection and screen-reader output are separate evidence. See docs/agent-team/checkpoints/macos-bridge.md for the current app observations and limits. Native bridge correctness is not a VoiceOver certification.

## Consumer and removal policy

Cargo patches are not transitive. A separate consuming workspace must add both the existing gpui-base override and accesskit_macos = { path = "../gpui/vendor/accesskit_macos" } under [patch.crates-io], as shown in the getting started guide.

Keep the same runtime versions for this fix. Remove this override only after selecting and verifying an upstream adapter release containing equivalent getter/selector/event behavior and rerunning the native regression plus application AX checks. No automatic upgrade, upstream publication, commit or push is implied.

## Native API sources

- [Apple selected property](https://developer.apple.com/documentation/appkit/nsaccessibility-c.protocol/accessibilityselected?language=objc)
- [Apple expanded property](https://developer.apple.com/documentation/appkit/nsaccessibility-c.protocol/accessibilityexpanded?language=objc)
- [Apple accessibility notifications](https://developer.apple.com/documentation/appkit/nsaccessibility-swift.struct/notification?language=objc)
