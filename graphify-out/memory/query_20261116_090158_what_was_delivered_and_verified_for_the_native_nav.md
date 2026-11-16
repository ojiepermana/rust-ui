---
type: "query"
date: "2026-11-16T09:01:58.016867+00:00"
question: "What was delivered and verified for the native navigation implementation?"
contributor: "graphify"
outcome: "useful"
source_nodes: ["Navigation", "NavigationState", "GPUI Kit 0.9.1"]
---

# Q: What was delivered and verified for the native navigation implementation?

## Answer

Navigation is implemented in crates/ui-navigation and exported through ui_kit::navigation. The new library owns validated stable-ID models, application-controlled presentation, semantic request events, and retained focus/scroll coordination. It composes pinned GPUI Kit controls for Sidebar, Dockbar Inline/Overlay, Minidock, Navbar and Flyout. Showcase Components has a Navigation gallery; examples/navigation-app is a facade-only application with a resize focus-handoff regression. Consumers retain both event and observer subscriptions and call focus restoration before removing a region.

On 2026-11-16 the current checkout passed cargo fmt --all --check, denied-warning workspace Clippy, cargo test --workspace --locked (45 cases including doctests, zero failures/ignored), workspace build, documentation, dependency/source-integrity validation, cargo examples --locked and the release version validation script. The native Showcase follow-up operated all five patterns and observed destination updates, collapsed Sidebar popup Escape/Enter restoration, Dockbar left/right Inline plus right Overlay/outside dismissal, Minidock activation, Navbar activation/dismissal, Flyout close/Escape/Enter behavior, updated 120-to-7 badge names, inserted Reports, empty state and Light/System appearance. No native renderer or vendor selection source changed for navigation.

Acceptance remains In Progress for spec 0002 AC-12/AC-14 because the pinned macOS bridge does not expose selected/expanded properties on these Button roles. Headless metadata assertions do not certify native accessibility. VoiceOver and other platform/exhaustive scenarios remain unverified. See docs/agent-team/checkpoints/navigation-delivery.md, docs/agent-team/checkpoints/navigation.md, docs/getting-started/navigation.md and docs/specs/_root/0002-native-navigation/verify.md for portable evidence. The older graph snapshot does not yet index the new crate; this record supplements it rather than claiming a full graph refresh.

## Outcome

- Signal: useful

## Source Nodes

- Navigation
- NavigationState
- GPUI Kit 0.9.1