# Layouts

`AppShell::new(content)` accepts optional header, sidebar, toolbar, inspector and footer slots. IntoElement values follow normal GPUI ownership; no React state model or custom renderer is introduced. `ShellConfig::default().with_sidebar_width(280.)?` gives a validated preferred width. `ShellConfig::from_json` validates persisted configuration. AppShell::config also validates before use.

At the default 800 logical pixel threshold the sidebar collapses; inspectors appear only with sufficient extra width. The Showcase remains navigable through Ctrl+Tab while collapsed. Consumers should provide an alternative keyboard or compact navigation path in their own shells. Minimum native window dimensions belong in WindowOptions. Header and status regions stay separate from scrollable content.

Functions header, content_container, sidebar_navigation, navigation_group and dashboard compose native regions. Nested groups use normal child composition. Top navigation uses upstream TabBar, Breadcrumb or Buttons; selection remains controlled by the application. TabbedWorkspace composes the tab bar with caller selected content. Settings screens can compose these same sidebar, content and inspector slots without another layout engine.

`SplitWorkspace::master_detail(id, master, detail)` and `SplitWorkspace::inspector(id, main, inspector)` reuse h_resizable and resizable_panel. Add with_inspector to form a three panel workspace. Upstream owns handle painting, drag state, sizing constraints and keyed retention. Split panes are intended for desktop widths that fit their minimums; put a narrow window alternative in the application. The Showcase permits horizontal content scrolling where a three panel preview needs more space.

The Layouts gallery demonstrates seven compositions and live controlled selection. Root shell collapse is adaptive to native window dimensions. Optional docking is already accessible through ui_kit::components::dock, but a docking preset, persistent panel storage and custom fullscreen window commands are deferred. Native OS fullscreen remains the platform window's responsibility.

The workspace includes a temporary GPUI Base 0.9.1 compatibility patch so the first held pointer sample resizes the panel, including a native down/move/up sequence with no second move. The same upstream constraint and redistribution engine remains authoritative. See docs/architecture/compatibility.md for source provenance and external consumer overrides.
