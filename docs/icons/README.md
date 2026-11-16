# Icons

ui-icons owns 13 small original line SVGs. Their source and license follow the repository Apache-2.0 license. GPUI Kit's default assets remain available for its own internal controls under their upstream licenses; see THIRD_PARTY_NOTICES.md.

```rust
use ui_kit::prelude::*;
let mut icons = IconRegistry::built_in()?;
let id = IconId::new("product", "symbol")?;
icons.register(id, svg_bytes)?;
// In render:
// icons.render(&IconName::Search.id(), IconSize::Medium, IconState::Active, cx)?
```

Each registered ID is namespace:name. Lowercase ASCII letters, digits and hyphens are accepted, with nonempty segments up to 64 bytes. Names cannot collide. Lookup and render return explicit errors for missing icons. A caller may choose an empty upstream Icon as a visual fallback while keeping the error observable in its own status.

Validation checks UTF-8/XML, an SVG namespace/root, a finite positive viewBox, a 64 KiB size bound and a documented static shape subset. Scripts, embedded documents, event attributes and external references are rejected. This is asset contract validation; GPUI's SVG parser and renderer still perform the actual rendering. Register trusted product assets, not an arbitrary document upload service.

IconSize supplies small, medium and large logical dimensions. IconState selects foreground, primary or muted foreground from the active upstream theme. A disabled-looking icon does not disable a Button; apply the control's own disabled property as well. `IconRegistry::usage` returns a copyable Rust expression. The Icons section searches registry keys and demonstrates size and accent changes.
