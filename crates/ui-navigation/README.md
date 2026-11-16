# ui-navigation

Controlled native Sidebar, Dockbar (Inline/Overlay), Minidock, Navbar and Flyout,
composed from GPUI Kit 0.9.1 controls. Applications should import this crate via
`ui_kit::navigation`, retain one NavigationState entity per instance and store
both `cx.subscribe` (requests) and `cx.observe` (parent invalidation). Apply
application decisions through setters outside render. Keep instance IDs unique
within each window and model node IDs unique within each model.

Validation is atomic. Stale presentation IDs remain stored without choosing
replacement destinations. Equivalent configuration is a no-op, including a
new Arc with identical complete model contents. Interaction emits requests;
selection, expansion and panel state change only after application setters.
Popup callbacks are deferred and arbitrated: opening another panel wins over
dismissal of the previous panel in the same input batch; stale revisions cannot
dismiss newer panels. A bounded next-frame notification reconciles rejected
requests to the application snapshot without echo events.

The application fallback FocusHandle must remain mounted. Call restore_focus
before removing a navigation region, replacing focused application slots, or
dropping the entity. The instance tracks root/slot and popup scopes, leaf and
disclosure handles; model replacement and presentation changes reconcile before
old controls disappear. Native Popover owns Escape/outside-dismiss and its
focus lifecycle. Inline panel Escape is scoped to NavigationPanel and requires
`navigation::init(cx)` after GPUI Kit and ThemeService initialization.

Buttons retain upstream roles, disabled behavior and Enter/Space activation.
A small private forwarding composition sets explicit selected and expanded
metadata on the upstream Button interactivity, without a toggle role or
renderer changes. Native platform tree and screen reader behavior require
separate verification; compiling/headless tests do not prove assistive output.

Widths are preferred rem values. ThemeService supplies full semantic spacing,
radius, density and colors. Registered SVGs use the same ui-icons assets with a
1rem navigation size; ui-icons' default IconSize remains a fixed logical-pixel
API outside this projection. Missing assets use the first two Unicode chars of
the destination label. Count badges hide zero, cap visible values at 99+, and
retain the original value in the owning Button's accessible name.

Header/footer stay outside the vertical list scroll. Navbar includes them in
its horizontal bar. Popup panels have one bounded scrolling list; upstream
Popover handles viewport clamping, without promising automatic flipping.
`navigation_element_id(instance, node, part)` supplies collision-free IDs for
application diagnostics. Parts are `item`, `branch`, `popup`, `panel`, `close`;
root/flyout parts use no node identity (`root`, `trigger`, `popup`, `close`).

While a higher native dialog is active, popup mounting is deferred using the
upstream WindowExt dialog-state seam. The application's panel/flyout values are
retained; navigation updates do not pull focus out of that dialog. Navbar is
full-width by default and preserves intrinsic control height. Focused Navbar
controls reconcile the retained horizontal ScrollHandle using current-frame
bounds, including after reorder and programmatic focus, without saved indices.

Local evidence: 7 pure tests and 11 production GPUI headless tests cover model,
metrics, badge names, atomic/equivalent configuration, focus reconciliation,
request arbitration/stale callbacks, ignored open/close requests, higher modal
focus, normal/coalesced pointer Flyout activation, compact spacer geometry,
Navbar focus scrolling and full control height/effective native metadata.
These tests do not establish a macOS/Windows/Linux screen-reader result.

Enabled popup triggers take pointer focus before their ancestor Popover opens,
so Escape restores the actual trigger even if an unrelated application control
previously held focus. This mouse-down handler only focuses; Popover remains
the sole semantic request source. A higher active dialog blocks that focus
transfer. A production regression covers external focus, pointer open, accepted
Escape close, and Enter reopening both Flyout and Navbar without invoking the
external command.
