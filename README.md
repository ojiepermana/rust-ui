# Rust UI Kit

A native Rust desktop UI workspace with a reusable design system and an interactive Showcase. GPUI Kit **0.9.1** supplies standard components and pins its GPUI snapshot to **0.3.12**. This repository owns theme configuration, application layouts and semantic SVG registration.

## Run

Install Rust through rustup and your native toolchain. The workspace pins Rust **1.100.0** (latest stable checked on 2026-11-16). On macOS, select a complete Xcode installation. Upstream documents macOS 15+, Windows 10+ with MSVC/CMake, and Linux with Wayland or X11 plus Vulkan. See [platform requirements](docs/architecture/compatibility.md).

```sh
cargo run -p showcase
```

Showcase links the local libraries directly. It includes Overview, 22 component stories, Themes, seven Layouts, searchable Icons, a live Playground and bundled Documentation. The toolbar switches light, dark and system appearance. Ctrl+Tab changes section; Command+B on macOS (Control+B elsewhere) collapses navigation.

## Workspace

| Crate | Responsibility |
| --- | --- |
| ui-core | Small appearance and density contracts |
| ui-components | Unmodified upstream component module re-exports |
| ui-theme | Validated JSON tokens, registry, global selection and upstream projection |
| ui-icons | Typed semantic IDs, SVG validation, registration and upstream rendering |
| ui-layout | Application shell, navigation, dashboard, split and tab compositions |
| ui-kit | Public facade and compatible runtime re-export |
| showcase | Local source workbench and retained demo state |

On macOS, `scripts/bundle-macos.sh` also prepares an unsigned development bundle at `target/native/Showcase.app`. It is a convenience for local inspection, not a signed installer.

The three consumer binaries are `minimal-app`, `dashboard-app` and `theme-playground`. Each depends on the facade through a workspace path.

## Use in another project

```toml
[dependencies]
ui-kit = { path = "../gpui/crates/ui-kit" }

# Required temporarily to carry the native coalesced-drag fix into a consumer workspace.
[patch.crates-io]
gpui-base = { path = "../gpui/vendor/gpui-base" }
```

```rust
use ui_kit::{gpui, prelude::*};

// In a Render implementation:
// AppShell::new(Button::new("save").primary().label("Save"))
```

See [the complete minimal consumer](examples/minimal-app/src/main.rs) for initialization, appearance observation and opening a native window.

The root workspace carries a narrowly scoped GPUI Base 0.9.1 behavior fix for a native gesture containing only one held move. See [patch provenance and removal criteria](vendor/gpui-base/PATCHES.md). Cargo patches are not transitive, so external path or Git consumers need the override in their own workspace; the example above uses a local checkout. GPUI Kit 0.9.1 and the GPUI 0.3.12 runtime remain pinned.

Git consumption uses the same package from a tagged workspace. Replace the example URL with your actual hosted repository after an authorized push and tag:

```toml
ui-kit = { git = "https://github.com/your-org/rust-ui-kit", tag = "v0.1.0" }
```

No remote, release, tag or registry package is created by this implementation. Crate names are provisional for future registry publication. Internal dependencies have both paths and registry version alternatives. See [release preparation](docs/contributing/releases.md).

## Development

```sh
cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo examples
cargo doc --workspace --no-deps --locked
python3 scripts/validate-dependencies.py
```

Install `watchexec-cli` separately with `cargo install watchexec-cli --version 2.8.0 --locked`, then run `scripts/dev.sh`. It rebuilds and restarts the entire process on relevant changes. Cargo reuses compiled dependencies. See [development details](docs/contributing/development.md) for incremental code generation options. This is an automatic restart workflow; it is not Rust hot reload. Runtime property edits in the Playground update retained GPUI state immediately.

## Documentation and evidence

* [Architecture and graph](docs/architecture/README.md)
* [Compatibility and platform matrix](docs/architecture/compatibility.md)
* [Validation evidence and remaining gaps](docs/architecture/validation.md)
* [Component coverage](docs/components/README.md)
* [Themes](docs/themes/README.md), [layouts](docs/layouts/README.md), [icons](docs/icons/README.md)
* [Getting started](docs/getting-started/showcase.md)
* [Contributing](CONTRIBUTING.md)

The initial story set demonstrates the architecture; it does not exhaust the upstream component catalog. Platform builds and native accessibility are reported only where observed. Upstream APIs remain available without copied implementation code. Licensed under Apache-2.0.
