# Checkpoint: native

Updated: 2026-11-16T15:11:47.359708+07:00. Run and runtime provenance: ../state.json. Workspace: /Users/ojiepermana/Development/ojiepermana/gpui. Branch: unborn main, no base commit. Existing source remains untracked. Only Leader coordination files were written. No integration or commit occurred.

## Native observation ledger

Environment: Apple Silicon macOS. Computer Use selected Rust UI Kit Showcase, bundle dev.rust-ui-kit.showcase. Native screenshots and accessibility states were inspected inline in this chat. They were not exported as screenshot files. The observations below refer to the named Computer Use operations in this run, rather than prior validation.md claims.

| Observation | Action and observed result | Status |
| --- | --- | --- |
| Overview | “Observing foundation overview” displayed 22 stories, two initial themes, seven compositions and 13 icons. | passed |
| Components | “Checking component catalog” displayed Button preview, description, snippet and properties. Button contained the check SVG. | passed |
| Theme validation | “Testing invalid theme input” entered #GGGGGG; status reported invalid theme token and palette stayed unchanged. | passed |
| Theme registration | “Testing registered theme tokens” entered #16A34A; custom-1 appeared, primary/focus token swatches changed, and subsequent Playground Button was green. | passed |
| Playground live edits | “Exercising Playground updates” changed label to Verify foundation, variant to Outline, size to Large, icon to search and compact density. Preview changed and click made Activations: 1. | passed |
| Disabled | “Checking disabled command rejection” enabled Disabled and clicked preview; accessibility marked it disabled and activation count stayed 1. | passed |
| Loading | “Checking Button loading state” showed a spinner after enabling loading. | passed |
| Retained state | “Checking retained Playground state” returned after other sections; label, activation count, compact density and loading persisted. | passed |
| Icons | “Testing icon filtering and appearance” searched search; one ui:search icon rendered at selected 28 px with accent color. | passed |
| OS clipboard and empty state | “Verifying native clipboard usage” clicked Copy Rust usage and pasted through native Command+V into search; value was icons.render(&IconId::new("ui", "search")?, IconSize::Medium, IconState::Default, cx)?. Status reported copied and the unmatched query rendered explicit empty state. Search was then reset to search. This proves this host’s copy/paste path, not universal clipboard certification. | passed |
| Shell | “Inspecting reusable layout compositions” displayed application header, sidebar, toolbar, content and footer. | passed |
| Sidebar | “Testing controlled navigation selection” selected Members and observed Members heading and selected background. | passed |
| Dashboard | “Checking dashboard composition” displayed Projects 12, Members 48, Tasks 216. | passed |
| Master detail | “Testing detail content selection” selected Native runtime and observed detail heading change. | passed |
| Split | “Testing native divider resizing” dragged (446,350) to (550,350); divider moved from x446 to x550 and text reflowed. | passed |
| Tabs | “Testing tab content switching” selected Code; tab accessibility changed and Code heading rendered. | passed |
| Inspector | “Testing inspector divider” dragged (958,350) to (878,350); second divider moved and trailing pane grew, first divider stayed fixed. | passed |
| Documentation | “Checking bundled documentation” rendered bundled guide including required local Cargo patch, imports and sections. | passed |
| Light and dark | “Checking light palette refresh” visibly switched to light and blue default Button. “Checking collapsed navigation shortcuts” switched to dark. | passed |
| Focus and collapse | “Checking focus after page removal” focused Playground label then Ctrl+Tab reached Documentation. After collapsing sidebar, another Ctrl+Tab reached Overview. | passed |
| System selection | “Restoring system appearance selection” selected System and restored expanded sidebar on Overview. | passed |
| Live OS appearance event | Follow-up on 2026-11-16: macOS Dark → Light → Dark changed the existing Showcase palette and semantic token swatches automatically while System stayed selected. See the follow-up ledger below. | passed |

All seven destination surfaces and all seven layout compositions were observed. No native functional failure was observed. Playground defaults were restored, with the session local custom theme and activation count still present. No application source changed. The existing bundle remains open on Overview with System selected. The separate CLI process started by this run was stopped.

Other platforms, physical multi-monitor DPI, IME and screen reader use were not exercised. Native accessibility state exposed command and input labels, but this is not an accessibility compliance pass. The follow-up below closes the AC-3 appearance event gap. No scope row or verify.md matched the foundation, and no boxes or spec status were changed.

## Live macOS appearance follow-up, 2026-11-16

The user explicitly requested this check and its report update. The Leader performed it inline without additional workers or source edits. Current Git HEAD was 13c021c2f817d64e1a6e7b962e46ce4bfecc33d4. Current target/debug/showcase and the native Showcase bundle both retained SHA256 95ed2edae7263286c84c4d5fe80ef11310d251ce27132efc094330319d1dfc73, matching the previously verified executable. The existing app process remained running throughout.

| Evidence operation in this chat | Action and observed result | Status |
| --- | --- | --- |
| Reading original appearance mode | System Settings → Appearance exposed Dark as selected. | passed |
| Observing dark system tokens | Showcase Themes displayed System selected, background #10141B, surface #171E29, primary #60A5FA, and dark native controls. This established the baseline before any OS change. | passed |
| Switching macOS to Light; Confirming Light selection | Selected the OS Light control. Accessibility state confirmed Light selected. | passed |
| Observing automatic light refresh | Without clicking any Showcase control after the OS change, its existing window changed to light surfaces and controls. System stayed selected. Token swatches displayed background #F8FAFC, surface #FFFFFF, primary #2563EB; footer showed light. The custom input value and three registered theme cards remained present. | passed |
| Restoring macOS Dark appearance; Confirming original mode restored | Selected the OS Dark control. Accessibility state confirmed Dark selected again. | passed |
| Observing automatic dark refresh | Without clicking any Showcase control after the OS change, its existing window changed back to dark surfaces and controls. System stayed selected. Background, surface and primary returned to #10141B, #171E29 and #60A5FA; footer showed dark. | passed |
| Restoring the original app view | System Settings returned to its initial General page. Showcase returned to Overview with expanded navigation and System selected. | passed |

Screenshots and accessibility states were inspected inline through Computer Use; no screenshot files were exported. The evidence proves both live OS transitions on this macOS host, with no app restart or app theme command between each OS action and its resulting observation. It does not measure refresh latency, exercise scheduled Auto appearance transitions, or certify other platforms or screen readers. AC-3 now passes together with the prior named theme, validation and token refresh evidence. Automated checks were not rerun because this follow-up changed documentation and coordination state only.
