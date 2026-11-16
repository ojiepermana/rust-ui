# Local development

Run all commands from the workspace root. `cargo run -p showcase --locked` uses local workspace paths; publishing is unnecessary. The three example consumers are built with `cargo examples --locked`.

For automatic rebuild and restart, install the tested watcher with `cargo install watchexec-cli --version 2.8.0 --locked`, then run `scripts/dev.sh`. It watches library/app sources, embedded assets, bundled documentation and workspace dependency files. A change terminates the current process, runs Cargo and opens a fresh Showcase. Session state resets on restart.

Source changes modify Rust or embedded assets and require compilation. Cargo caches unchanged crates, so subsequent builds reuse them. Fine grained rustc incremental code generation is disabled in the baseline dev/test profiles to bound GPUI build storage. You can opt in locally with `CARGO_INCREMENTAL=1 scripts/dev.sh` for a different build time/storage tradeoff. This compiler cache is distinct from GPUI's runtime view updates.

Runtime property changes update retained GPUI state and call notify, without compiling or restarting. True Rust hot reload would replace code in a running process and is not implemented. The watcher is never described as hot module replacement.

Useful commands: cargo fmt --all --check, cargo clippy --workspace --all-targets --locked -- -D warnings, cargo test --workspace --locked, cargo build --workspace --locked, cargo doc --workspace --no-deps --locked, python3 scripts/validate-dependencies.py.

On macOS, scripts/bundle-macos.sh creates an unsigned local Showcase.app under target/native. It copies the debug executable and writes native bundle metadata. Signing, notarization, installers and distribution are separate release work.
