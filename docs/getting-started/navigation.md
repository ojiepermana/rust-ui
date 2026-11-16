# Native navigation

Use `ui_kit::navigation` or the curated prelude for Sidebar, Dockbar, Minidock, Navbar and Flyout. Standard controls and overlays come from the pinned GPUI Kit 0.9.1. The runtime stays GPUI 0.3.12.

Run `cargo run -p navigation-app --locked` for a complete facade consumer. Run `cargo run -p showcase --locked`, open Components and select Navigation for the five patterns. The gallery changes destination content in its application event handler and demonstrates nested items, disabled branches, badges, compact rails, empty data and live model replacement.

Create one `Entity<NavigationState>` per instance in the view constructor. Give it a nonempty instance ID, a mounted application fallback `FocusHandle`, and a validated `Arc<NavigationModel>`. Keep IDs stable across inserts and reordering. Every node, including hidden nodes, separators and spacers, needs a unique ID within its model. Separate instances may reuse node IDs but need distinct instance IDs.

```rust,no_run
use std::sync::Arc;
use ui_kit::prelude::*;

let inbox = NavigationId::new("inbox")?;
let model = Arc::new(NavigationModel::new(vec![
    NavigationItem::new(inbox.clone(), "Inbox")
        .icon(IconName::Book.id())
        .badge(NavigationBadge::count(120).with_description("unread messages"))
        .into(),
])?);
# Ok::<(), NavigationError>(())
```

Initialize `ui_kit::gpui::init`, `ThemeService::init` and `ui_kit::navigation::init` once, in that order. Keep one upstream Root per window using `gpui::open_window`.

Retain two subscriptions in the owning view. `cx.subscribe_in` receives `NavigationEvent` requests with a Window for focus-aware setters. `cx.observe` calls `cx.notify()` on the displaying view when the navigation snapshot changes. A `RenderOnce` component reading an entity does not install this observer automatically.

On `Activate { id }`, map the ID to application content, then call `set_presentation` with the selected ID and your chosen panel closing policy. On expanded, panel, collapsed and flyout requests, accept or reject the requested value in the application handler. Rendering and setters emit no interaction events. A rejected popup request is reconciled on the next frame. Configuration and model setters validate before changing anything; invalid Minidock trees leave the previous configuration intact. Equivalent configuration is a no-op.

Call setters outside render. Retain entities, focus handles and subscriptions in the constructor, including any state in header/footer slots. Slot elements themselves are composed each frame. Use `Navigation::new(&state)` with `header`, `footer`, `empty`, `accessibility_label`, `trigger_label` and `side` as appropriate.

Minidock accepts only a flat model. Build that model in the application with the same leaf IDs as your tree. It never widens on hover or focus. Sidebar collapse produces a compact rail, while shell collapse hides the entire sidebar region. Those are independent operations.

Before removing or hiding the navigation region, call its `restore_focus` helper. It moves focus only when the instance owns current focus. A mounted fallback is an application prerequisite. The navigation-app consumer retains a bounds observer and uses the same `ShellConfig::collapse_below()` threshold as AppShell, handing off focus before the sidebar disappears. Its content destination buttons remain available at narrow widths. Do the same before replacing focused slots or discarding the view.

Use expanded Sidebar in the shell sidebar slot. Compose a rail or Dockbar plus content with `h_flex().items_stretch()` in the shell content region so its preferred width can take effect. Navbar and Flyout can live in a header or toolbar. AppShell's width validation and seven existing layout compositions are unchanged.

Registry icons use the supplied `IconRegistry`. Missing icons fall back to two Unicode characters from the label. The asset registry's default `IconSize` is a fixed logical pixel size; navigation projects the rendered glyph to the control/rem scale. Count badges hide zero, cap visible text at `99+`, and keep the actual count in the accessible name. `Inbox` with count 120 and description `unread messages` is named `Inbox, 120 unread messages`. Descriptions should omit repeated numbers and be localized by the application.

Use the dependency override described in the [Showcase getting started guide](showcase.md) when consuming this facade from another Cargo workspace. Cargo patches are not transitive. Navigation introduces no new vendor changes or runtime upgrade.

Tests of models and synthetic GPUI input, native macOS operation, and assistive technology results are distinct evidence. See the [spec verification checklist](../specs/_root/0002-native-navigation/verify.md) and the navigation checkpoint under docs/agent-team for the current results. Windows, Linux and screen reader behavior require their own platform runs.

The current pinned macOS bridge does not expose the Button selected/expanded states required by spec 0002. The crate supplies the GPUI metadata and tests it, while full native accessibility acceptance remains open. See the [compatibility boundary](../architecture/compatibility.md#native-navigation-accessibility-boundary).
