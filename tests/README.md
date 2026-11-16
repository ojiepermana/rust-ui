# Validation map

Pure contracts live in crates/ui-theme, crates/ui-icons and crates/ui-layout. The external consumer contract is crates/ui-kit/tests/public_api.rs. Native event integration tests use the real Showcase in apps/showcase/tests/workbench.rs; app owned story rendering checks live beside its implementation. scripts/validate-dependencies.py validates workspace source and runtime boundaries. No duplicate root test runner is needed in a virtual Cargo workspace.
