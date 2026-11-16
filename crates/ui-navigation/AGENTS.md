# Native navigation

Read ../../docs/specs/_root/0002-native-navigation/index.md before changing public behavior. Model validation and immutable data live in src/model.rs, retained mechanical coordination in src/state.rs, and upstream composition in src/view.rs. src/tests.rs drives the production renderer.

Use GPUI Kit Button, Popover, Collapsible, SidebarToggleButton, Badge and Tag. Do not copy their implementations or change vendor sources. Interactions request changes through NavigationEvent, application setters commit them. Allocate entities, subscriptions and focus handles outside render. Pointer popup triggers must receive focus before upstream captures its restoration target.

Run cargo test -p ui-navigation --lib --locked and the root validation commands. Keep raw coalesced input regressions, framed helpers can hide native focus ordering. Native macOS selected/expanded bridge support and screen reader output are separate from headless metadata assertions.

_Drafted by /sync from the introducing change, worth a quick human pass._
