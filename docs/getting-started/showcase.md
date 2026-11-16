# Using Rust UI Kit

Add the local facade to your application's Cargo.toml:

```toml
[dependencies]
ui-kit = { path = "../gpui/crates/ui-kit" }

# Temporary same-version compatibility fixes for divider input and macOS metadata.
[patch.crates-io]
gpui-base = { path = "../gpui/vendor/gpui-base" }
accesskit_macos = { path = "../gpui/vendor/accesskit_macos" }
```

Import `ui_kit::prelude::*` and `ui_kit::gpui`. When declaring GPUI actions through the facade, also import `use ui_kit::gpui as gpui_kit;`, which the upstream derive macro resolves by name. Initialize `gpui::init(cx)` and `ThemeService::init(cx)` before opening a window with `gpui::open_window`. Register native appearance observation with `ThemeService::sync_system(window, cx)`.

Cargo patches are not transitive. The temporary overrides preserve the native divider fix and macOS selected/expanded metadata in consuming workspaces. Versions stay GPUI Kit 0.9.1, GPUI 0.3.12 and AccessKit macOS 0.26.3. They patch one drag-start callback and the native accessibility adapter, without replacing component rendering.

## Components

`ui_kit::components` preserves GPUI Kit's original module paths. For example, `ui_kit::components::input::{Input, InputState}`. Keep InputState entities and subscriptions in your owning view. The gallery documents the demonstrated subset, while the full upstream catalog remains re-exported.

## Themes

`ThemeRegistry::built_in()` loads packaged light and dark definitions. Parse custom JSON with `ThemeDefinition::from_json`, register through `ThemeService::register`, and apply with `ThemeService::select_named`. The Themes section registers a primary color override. Read the complete resolved scales through `ThemeService::global(cx).tokens()`. Upstream legacy theme storage limits which scales affect standard components.

## Layouts

`AppShell` supplies header, sidebar, toolbar, content, inspector and footer slots. `SplitWorkspace` reuses upstream resizing. `TabbedWorkspace` composes a controlled TabBar with content. Narrow windows collapse the sidebar; Ctrl+Tab still changes Showcase sections.

## Icons

`IconRegistry::built_in()` loads 13 original SVGs. Use `IconName::Search.id()` for a typed built in ID. Register a pack with `IconId::new("product", "name")` and validated SVG bytes. The renderer accepts `IconSize` and `IconState` and returns an upstream Icon suitable for a Button.

## Development

Run `cargo run -p showcase`, `cargo test --workspace`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo doc --workspace --no-deps`. `scripts/dev.sh` uses watchexec to rebuild and restart the native process. Runtime property changes do not compile Rust. Source edits reuse Cargo's crate level build cache and perform a full process restart; there is no Rust hot reload.

## Platforms

Upstream documents macOS 15+, Windows 10+ with MSVC and CMake, and Linux with Wayland or X11 and a Vulkan driver. Only the available macOS host is locally verified. Windows and Linux CI results and runtime checks must be recorded separately. Preserve native focus behavior; no accessibility compliance certification is claimed.

## Distribution

All internal paths also have registry version requirements. Future publication order: ui-core, ui-components, ui-theme, ui-icons, ui-layout, ui-kit. Registry names are provisional until availability is checked. Git consumption selects package ui-kit from a tagged workspace repository; no public URL or release is created automatically.
