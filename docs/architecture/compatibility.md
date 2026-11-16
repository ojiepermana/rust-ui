# Compatibility and supported platforms

Evidence date: 2026-11-16. GPUI Kit =0.9.1 comes from crates.io, with default component and asset features. Cargo.lock is included as source material for reproducibility. We use its re-exported GPUI APIs, never a separate Git fork or runtime pin. The release pins gpui-pre packages at =0.3.12. Optional gpui-fast packages may appear in Cargo.lock through test feature resolution; they are not selected by the normal application graph. The dependency validator checks the active normal runtime graph.

The official stable Rust distribution manifest dated 2026-11-12 identifies 1.100.0. The toolchain file pins that release. GPUI Kit does not declare rust-version; its installation guide names 1.92 for its own locked graph. We claim only this repository's tested 1.100 baseline, not an independently verified lower MSRV.

| Platform | Upstream status | Local build | Local runtime | Repository status |
| --- | --- | --- | --- | --- |
| macOS 15+ / Apple Silicon | Supported upstream | Debug build verified | Launch, selected interactions and native divider dragging verified | Locally verified subset |
| macOS Intel | Upstream native implementation | Not verified | Not verified | Experimental in this repository |
| Windows 10+ / x86_64 MSVC | Supported upstream | CI configured, not executed here | Not verified | Experimental until CI and native checks pass |
| Linux / x86_64 Wayland | Supported upstream | Ubuntu 24.04 CI configured | Not verified | Experimental until CI and desktop checks pass |
| Linux / x86_64 X11 | Supported upstream | Same native Linux dependency build | Not verified | Experimental until separate session checks pass |
| Mobile and WebAssembly | Outside this desktop scope | Not configured | Not verified | Unsupported by this workspace |

## Native dependencies

macOS: Xcode or the required development tools, a selected SDK, and a Metal capable desktop session. Upstream minimum is macOS 15.

Windows: Visual Studio 2022 Desktop development with C++, Windows SDK, MSVC Rust target and CMake on PATH. Use the native Windows build; a macOS cross compile is not a runtime test.

Ubuntu 24.04:

```sh
sudo apt update
sudo apt install -y gcc g++ clang libfontconfig-dev libwayland-dev   libwebkit2gtk-4.1-dev libxkbcommon-x11-dev libx11-xcb-dev libssl-dev   libzstd-dev vulkan-validationlayers libvulkan1
```

A graphical Wayland or X11 session and a functioning Vulkan driver are needed to show a window. ALSA is only required if a consumer enables upstream speech. This workspace does not enable speech or a webview. The package list follows upstream installation guidance and intentionally avoids inventing Linux backend support.

## Feature matrix

| Capability | Implementation | Verification boundary |
| --- | --- | --- |
| Standard components | Full upstream module re-export, 22 live stories | All declared stories rendered in headless production view |
| Theme integration | Light, dark, system, named registration, validated JSON, semantic and component alias projection | Native switching and headless validation tests |
| Typography and scales | Spacing, typography weights/families, radius and elevation snapshot | Application owned scales, not every upstream legacy component field |
| Icons | 13 typed originals, namespaced custom packs, static SVG validation, three sizes and semantic states | Registry contracts and native rendering |
| Layouts | Shell, navigation, dashboard, master detail, inspector, split and tab compositions | Native preview and headless panel drag/geometry tests |
| Stateful controls | Retained Input, Slider and Select entities and controlled toggles | Production UI interactions and focus tests |
| Development restart | watchexec 2.8.0 driven rebuild/restart script | Source and bundled documentation restart verified locally |
| Distribution | Paths, Git package selection, versioned registry alternatives, manual release artifacts | Actual registry publication and signed installers are deferred |

## Native divider compatibility patch

Native macOS input can deliver down, one held move, then up. GPUI Base 0.9.1 discarded that first move when it started a panel drag, so no resize occurred without a second move. The workspace carries the published gpui-base 0.9.1 source with one callback changed to apply the initial sample through the existing resize_panel_at_handle function. GPUI Kit remains =0.9.1 and the renderer/runtime remains GPUI =0.3.12.

[Patch provenance, complete diff and removal criteria](../../vendor/gpui-base/PATCHES.md) are recorded with the vendor package. The dependency validator verifies the original source hashes and the single intentional patch. No resize arithmetic or component implementation was replaced. The clean native build was verified in both drag directions, at minimum/maximum limits and on the inspector's second divider.

Cargo patches are not transitive. External path or Git consumers need the same override in their own workspace, as shown in the getting started guide. Before registry publication, select an upstream fixed release and remove this override. Do not replace the patch with an unrelated runtime or fork.

## Known upstream API boundaries

Theme::apply_semantic_config returns a full token snapshot but projects only legacy representable fields onto standard components. The integration explicitly projects component color aliases (buttons, lists, tables, tabs and other demonstrated controls) from the same semantic source, because upstream does not infer those aliases on a semantic edit. Custom spacing, full typography, elevations and radius details are consumed by repository layouts through ThemeService::tokens. Component density uses upstream explicit sizing; it is not a promise to resize every upstream component globally.

Published Dialog uses its title for the accessible name; the installed 0.9.1 type has no public accessibility_label setter despite a tagged release example referencing one. Our code follows the installed artifact. Reduced motion uses GPUI's flag and upstream support; actual OS settings and screen reader behavior still need platform verification.

## Sources

[Release](https://github.com/longbridge/gpui-kit/releases/tag/v0.9.1), [installation](https://gpui-kit.com/docs/installation), [pinned release manifest](https://github.com/longbridge/gpui-kit/blob/v0.9.1/Cargo.toml), [official Rust stable manifest](https://static.rust-lang.org/dist/channel-rust-stable.toml).
