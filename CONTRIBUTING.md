# Contributing

Use the pinned toolchain and native dependencies in docs/architecture/compatibility.md. Start Showcase with cargo run -p showcase. Keep standard controls in GPUI Kit and reusable application presentation in its owning crate.

Before submitting a change, run formatting, Clippy with warnings denied, workspace tests/build, docs and the dependency validation script from the README. Add tests for behavioral contracts rather than reproducing builder chains. Theme and icon validation need negative cases. UI behavior should be exercised with the production view and gpui-kit test-support helpers.

Preserve local path dependencies and registry alternatives. Document every public API. Record any native platform or accessibility checks and their limits in docs/architecture/validation.md. Check installed signatures when adding a component; examples from a different snapshot are not proof that a method exists.

For a new component story, add its catalog metadata and render branch, then exercise its real interaction. For a layout, reuse upstream resizing/docking behavior. For new assets, record their source and license. Source edit auto restart instructions are in scripts/dev.sh. Release preparation is described in docs/contributing/releases.md.
