# Compatibility evidence and rationale

The empty checkout was inspected on 2026-11-16. Host: macOS 27.0.1, Apple Silicon, Xcode selected, rustc and cargo 1.99.1. Latest GitHub release is v0.9.1, published 2026-11-09, commit 39ab849a93b5f15507b969164d49891db8d5b470. crates.io cargo info agrees.

GPUI Kit's release workspace pins gpui-pre, gpui-pre-platform and gpui-pre-macros to =0.3.12. Its native platform enables font-kit, x11, wayland and runtime_shaders. The optional gpui-fast backend is deliberately unused. No gpui-shell or JavaScript extension dependency is selected.

The release manifest does not declare a rust-version. Its installation guide states Rust 1.92 or later for its locked graph (oo7 0.6.0 on Linux). Our baseline is conservatively 1.100, compiled with 1.100.0; no lower MSRV is claimed. Cargo.lock records our resolved transitive graph.

We inspected upstream skills/gpui-kit and gpui-kit-design-guides, coding/design guides, examples/hello_world, examples/ai_recipes, native UI test helpers, theme/mod.rs, theme/schema.rs, icon.rs and resizable.rs. APIs come from the pinned release source, not inferred web framework conventions.

## Options considered
Direct Zed Git dependencies would increase compatibility maintenance and complicate publication. Copying components would duplicate behavior. One gpui-kit dependency with native re-exports gives a registry backed, consistent dependency graph. Thin composition is preferred over a custom component engine.

## Important boundary
Theme::apply_semantic_config returns full tokens but projects only legacy representable fields onto components. Spacing, detailed typography, radius and elevation overrides remain available through our active resolved snapshot. Component geometry follows upstream sizing APIs; we do not claim arbitrary token overrides resize every upstream component. Native appearance observation is event driven.

## References
* [Release](https://github.com/longbridge/gpui-kit/releases/tag/v0.9.1)
* [Pinned manifest](https://github.com/longbridge/gpui-kit/blob/v0.9.1/Cargo.toml)
* [Installation](https://gpui-kit.com/docs/installation)
* [Application recipe](https://github.com/longbridge/gpui-kit/tree/v0.9.1/examples/ai_recipes)
* [Component catalog](https://gpui-kit.com/component)
* [Coding guides](https://gpui-kit.com/docs/coding-guides)
* [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
* [Rust API guidelines](https://rust-lang.github.io/api-guidelines/)
