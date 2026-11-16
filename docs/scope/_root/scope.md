# Scope: Rust UI Kit

Native reusable Rust interfaces for application developers. This scope enrolls the selected navigation slice only.

**Build approach:** Tracer Bullet, integrate one working surface from crate to consumer before broadening.
**Workflow:** Beta, runtime verification and regression tests after implementation. Selected automatically under the user decision policy.

## At a glance

| # | Feature | Phase | Status |
| --- | --- | --- | --- |
| 1 | Navigasi native dengan GPUI Kit | Navigation | in-progress |

## Navigation

### 1. Navigasi native dengan GPUI Kit · in-progress

You can compose five native navigation patterns from one validated model and controlled application state.

**Done when:** all five patterns, stable identity, focus handoff, facade consumption, Showcase and spec 0002 verification are implemented with platform limitations recorded.

- [x] Design it (spec): `/architect Navigasi native dengan GPUI Kit`
- [x] Build it: `/develop Navigasi native dengan GPUI Kit`
  - [x] Model, retained state and facade
  - [x] Five native compositions and focus lifecycle
  - [x] Showcase, consumer and documentation
- [ ] Verify it: `/check verify Navigasi native dengan GPUI Kit`
- [x] Test it: `/test Navigasi native dengan GPUI Kit`

Spec [0002](../../specs/_root/0002-native-navigation/index.md).

Build and tests passed locally. The native selected/expanded bridge was repaired on the same version and separately reverified. Verify remains open for other unchecked scenarios in the [verification ledger](../../agent-team/checkpoints/navigation.md).

The user authorized completion, commit and push of the working implementation.
The [delivery follow-up](../../agent-team/checkpoints/navigation-delivery.md)
records current-checkout gates and the native Showcase checks without advancing
the incomplete native accessibility acceptance.
