# Native UI foundation

**Status**: Accepted
**Date**: 2026-11-16
**Authorization**: The supplied implementation request authorizes dependency selection, architecture, implementation and local verification. No publication or push is authorized.

## Summary
Build a native Rust workspace with GPUI Kit components, application owned design tokens, SVG registration and reusable layouts. Establish a functional vertical slice before widening the gallery.

## Requirements
AC-1: Six reusable libraries and Showcase share one workspace and local versioned path dependencies.
AC-2: GPUI Kit 0.9.1 supplies components and its exact GPUI snapshot supplies the runtime.
AC-3: Validated serializable themes support light, dark, system and named registration. Native appearance events apply system changes. Token changes refresh components.
AC-4: A typed namespaced icon registry validates selected SVG assets and renders through GPUI Kit, including in a Button.
AC-5: Configurable application shell, navigation, dashboard, master detail, inspector, tabs and split composition reuse native elements and upstream resizing.
AC-6: Showcase has Overview, Components, Themes, Layouts, Icons, Playground and Documentation. Playground updates immediately. Icons support search, size, color and copying usage.
AC-7: Three compiled consumer examples, tests, documentation and deliberate release tooling make the libraries reusable.
AC-8: Run build, tests, formatting, Clippy, docs and native launch locally. Report platform and accessibility evidence without extending it beyond what was checked.

## Decision
Use gpui-kit =0.9.1, default component and assets features. Do not add GPUI separately. Rust 1.100.0 is the latest stable toolchain verified against the official 2026-11-12 manifest and the reproducible workspace baseline. Use versioned paths for internal dependencies. Preserve upstream module paths via a component re-export, rather than type renaming wrappers. Native OS window decorations remain owned by GPUI.

## Public API and ownership
ui-core owns ThemeMode and Density. ui-theme owns ThemeDefinition, ThemeRegistry and global application selection, and resolves its authoritative JSON semantic configuration onto upstream Theme::update. Retain the complete resolved semantic snapshot for application layouts because upstream legacy fields cannot retain every spacing, typography and shadow override. ui-icons owns IconId, IconName, IconRegistry and selected original SVG bytes. ui-layout owns stateless RenderOnce compositions plus validated ShellConfig. ui-components re-exports the upstream catalog. ui-kit exposes named modules, a curated prelude and the single compatible runtime.

Stateful Inputs, Slider and Select are retained by Showcase, with subscriptions retained in the owning entity. No state entities are created from render. IDs derive from section, component and icon names. All navigation is a Button or native Tab. Escape and overlays use upstream Root. View state is session local; persistent docking is optional and deferred.

## Showcase design
A quiet neutral desktop workbench with a blue accent, semantic light and dark surfaces, compact typography and visible borders. Sidebar navigation and a persistent theme toolbar surround a scrollable canvas. Overview derives counts from registries and the demonstrated component catalog. Component selection exposes a description, actual preview, documented states, a Rust snippet and API link. Playground edits button variant, size, disabled/loading state, label, density and semantic icon. Theme editor validates a hex primary override before registration. Icon search uses an Input and registry filtering, with an explicit empty state. Layout selection changes actual reusable compositions with upstream resizable handles. Documentation uses bundled Markdown rather than another database.

## Build plan
1. Inspect release source, record compatibility and establish valid workspace metadata (AC-1, AC-2).
2. Build Button through ui-components and ui-kit into Showcase, add theme, SVG and shell (AC-3, AC-4, AC-5).
3. Extend native interactive pages and reusable layout compositions (AC-6).
4. Add meaningful contract and real headless UI tests, consumer examples, docs and CI (AC-7).
5. Correct build errors, run all checks and launch the actual macOS application (AC-8).

## Consequences
Full restarts after source edits are supported with watchexec. Runtime property edits need no recompilation. There is no claim of Rust hot reload. Platform runtime verification is limited to available machines. Optional docking can use the exported upstream dock module without a new dock engine.

## Follow-up
Maintain the evidence and gaps in docs/architecture/validation.md. Optional full catalog stories, persisted layouts and other OS runtime passes may extend this baseline.

## Rationale
See [rationale.md](rationale.md).
