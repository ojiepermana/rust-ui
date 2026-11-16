# Navigation delivery verification

Date: 2026-11-16, Asia/Jakarta. This follow-up closes the local implementation
delivery requested by the user, including commit and push authorization. The
initial implementation and platform acceptance record remains in
[navigation.md](navigation.md). It is not replaced by this follow-up.

## Current checkout checks

The current checkout includes the previously committed selection virtualization
regression and its source integrity rules. No additional vendor or runtime change
was needed for navigation.

| Check | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed |
| `cargo test --workspace --locked` | Passed, 45 cases including doctests, zero failures or ignored cases |
| `cargo build --workspace --locked` | Passed |
| `cargo doc --workspace --no-deps --locked` | Passed |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps --locked` | Passed, matching the CI documentation gate |
| `python3 scripts/validate-dependencies.py` | Passed |
| `cargo examples --locked` | Passed, including navigation-app |
| `python3 scripts/check-release.py v0.1.0` | Passed, validation only |
| `git diff --check` | Passed |

The 45 cases include 18 navigation crate cases, the facade contract, the actual
Showcase Navigation story and the facade-only consumer's resize focus handoff.
The workspace continues to use GPUI Kit 0.9.1 and one normal GPUI 0.3.12 runtime.
The vendored regression is excluded from workspace membership and has its own
[reproduction command](../../../vendor/gpui-base/PATCHES.md).

## Native Showcase follow-up

Host: macOS 27.0.1 build 26A434, Apple Silicon. The current checkout was launched
with `cargo run -p showcase --locked`, then built with
`scripts/bundle-macos.sh` for native inspection as `dev.rust-ui-kit.showcase`.
Window content is configured at 1200 by 820 logical pixels. Native screenshots
and accessibility trees were inspected inline through CUA.

| Surface | Observed behavior |
| --- | --- |
| Sidebar | Projects changed the destination content and activation count. Collapse produced an icon rail. Work opened its nested popup; Escape dismissed it and Enter reopened it without another pointer action. |
| Dockbar | Work opened an Inline panel on the left. Right dock moved the panel and rail to the right. Overlay mode opened a popup to the left of the right rail; clicking outside dismissed it. |
| Minidock | The compact rail showed the New badge and a 99+ count. Assigned to me changed the content and selected destination. Billing remained exposed as disabled. |
| Navbar | The horizontal controls retained their height. Work opened a popup; Activity log activation updated the destination and closed the popup. |
| Flyout | Workspace menu opened nested content. Escape closed it and Enter reopened it. Settings activation closed the popup and updated application content. The explicit Close menu button also dismissed it. |
| Data and empty state | Update data inserted Reports and changed Inbox's accessible name from 120 to 7 unread messages. Empty removed destination controls while preserving the application-owned selected destination. Restoring data restored the controls. |
| Theme | Light and System appearance both rendered. System was restored, resolving to dark on this host. No operating system appearance setting was changed. |

The Showcase was left on populated expanded Sidebar with System selected.
These observations supplement the automated checks. They are not a pixel
comparison, exhaustive native matrix or screen reader result.

## Acceptance boundary

The local implementation and automated delivery gates are complete. Spec 0002
and its Verify scope box remain open for native acceptance: the pinned macOS
bridge exposes names, roles, IDs and disabled state but not the required selected
and expanded properties on these Buttons. Headless metadata assertions do not
close that gap. VoiceOver, other operating systems and unchecked exhaustive
scenarios remain unverified, as recorded in the existing checklist.

No alternate role, runtime upgrade or new vendor behavior was introduced to
claim those criteria passed. This delivery makes the working implementation and
its evidence available in Git while retaining that explicit limitation.
