# Themes

The authoritative built in definitions are packaged in crates/ui-theme/themes/light.json and dark.json. No application keeps another palette. ThemeDefinition wraps GPUI Kit's serializable semantic configuration with name, appearance, density, feedback colors and reduced motion preference.

```rust
use ui_kit::prelude::*;
let mut themes = ThemeRegistry::built_in()?;
let ocean = themes.get("light")?.clone().with_primary("ocean", "#006677")?;
themes.register(ocean)?;
# Ok::<(), ui_kit::theme::ThemeError>(())
```

Use ThemeDefinition::from_json and to_json to exchange application configuration. Names are nonempty and unique. Colors accept #RRGGBB. Scales must be finite, ordered and nonnegative. Typography sizes and pixel line heights are positive; families cannot be empty. Duplicate registration is an error, so a bad configuration never silently replaces a working theme. Shadow configuration uses upstream structured BoxShadow values. No arbitrary CSS is accepted.

Initialize GPUI first, then ThemeService::init. Register custom definitions with ThemeService::register and switch with select_named. `ThemeService::select(ThemeMode::System, Some(window), cx)` selects by native appearance. Retain a `cx.observe_window_appearance` subscription calling `ThemeService::sync_system`. No polling is used. See examples/theme-playground for a complete consumer.

The integration maps component color aliases from the same resolved semantic source, including primary button normal, hover and active tokens. No separate component palette is maintained.

Theme::update synchronizes the upstream Base projection and refreshes all windows. Read the full application snapshot with ThemeService::global(cx).tokens(). That snapshot includes spacing, typography weights, all radius values and shadow scales. Legacy component storage accepts the subset described in compatibility.md. Upstream sizing remains authoritative for standard components. `content_gap(cx)` applies application density to the resolved medium spacing token.

When reduced_motion is true, the integration sets GPUI's reduced motion flag and zero transition durations and distances. Otherwise it delegates to upstream's OS preference adapter. macOS and Windows settings are read during initialization/application; Linux follows portal events where available. No claim of live macOS/Windows reduced motion preference notification is made. System font fallback is resolved by upstream, so no bundled font is necessary.

The native Themes section offers light/dark selection, system following, custom primary registration, token swatches, typography, spacing and radius explorers. A primary override also updates the focus ring; consumers should choose accessible foreground combinations and verify contrast in their product.
