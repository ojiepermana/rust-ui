# Rust UI Kit

Native Rust workspace. Read docs/architecture/README.md before changing crate boundaries and docs/architecture/compatibility.md before updating GPUI Kit.

## Build and validation
Run cargo fmt --all --check, cargo clippy --workspace --all-targets --locked -- -D warnings, cargo test --workspace --locked, cargo build --workspace --locked and cargo doc --workspace --no-deps --locked. scripts/validate-dependencies.py checks local sources, version alternatives and runtime alignment. Run cargo run -p showcase for the native app. See docs/architecture/validation.md for evidence boundaries.

## Ownership
GPUI Kit owns standard components and rendering. ui-components re-exports its catalog. ui-theme owns configuration, validation and token projection. ui-icons owns SVG registration. ui-layout owns reusable compositions. ui-kit only assembles the public API. Showcase holds demo state and consumes local crates.

## Rules
Use the runtime exported by ui_kit::gpui (gpui-kit 0.9.1, GPUI snapshot 0.3.12). Keep internal dependencies as versioned paths. Do not copy upstream components or add another renderer. Retain state entities and subscriptions outside render. Restore shell focus when a focused page control leaves the element tree. Use semantic colors, stable IDs and native controls for commands. Document public APIs. Report native platform and accessibility verification separately from headless tests. No push, publication or release without explicit user authorization.

## Upstream references
When adding a component or changing GPUI state ownership, consult the pinned upstream coding guide and actual source. References and release commit are in docs/specs/_root/0001-native-ui-foundation/rationale.md. Do not silently upgrade the runtime.

## Git
integration: off
