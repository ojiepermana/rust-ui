# Versioning and release preparation

All reusable crates start at 0.1.0 and use matching registry requirements plus local paths. Follow semantic versioning. Breaking API changes during 0.x advance the minor version. Keep the unified release version synchronized across manifests and internal requirements. Update CHANGELOG.md and the compatibility record with evidence.

Proposed publication order: ui-core, ui-components, ui-theme and ui-icons (after their dependencies), ui-layout (after ui-components), then ui-kit. These generic package names have not been reserved or checked for ownership on crates.io. A future registry release must first verify name ownership, choose a real repository URL and add it to the package metadata. Rename coherently if names are unavailable.

Before publication, move to an upstream GPUI Base release containing the coalesced native divider fix and remove the temporary vendor override. Workspace Cargo patches do not propagate through published library dependencies. The override and sunset criteria are documented in vendor/gpui-base/PATCHES.md.

Before publication, run all validation and `cargo package --list -p NAME` to inspect files. The theme and icon assets are inside their owning packages so no external include path breaks packaging. A complete `cargo publish --dry-run` of dependent crates requires the lower level versions to exist in the target registry; versioned paths alone cannot make unpublished registry packages available. Do not interpret an offline file list as a verified publish.

The manual release workflow validates the requested vMAJOR.MINOR.PATCH against metadata, builds/tests, and prepares a source archive and Showcase executable as workflow artifacts. It does not push tags, publish crates or create a GitHub release. Workflow artifacts are unsigned development binaries, not notarized or signed installers.

Publication and release creation require a separate explicit authorization and registry credentials. No credentials or publishing tokens are configured here. Once authorized, publish in dependency order, confirm registry indexing between dependent packages, then tag and create a reviewed release. Git consumers can use a version tag before registry publication.
