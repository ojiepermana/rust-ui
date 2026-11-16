#!/usr/bin/env bash
# Optional local development bundle, unsigned and not a distribution installer.
set -euo pipefail
cd "$(dirname "$0")/.."
if [[ "$(uname -s)" != Darwin ]]; then
  echo 'This development bundle helper requires macOS.' >&2
  exit 1
fi
cargo build -p showcase --locked
bundle_path="$PWD/target/native/Showcase.app"
mkdir -p "$bundle_path/Contents/MacOS"
cp target/debug/showcase "$bundle_path/Contents/MacOS/showcase"
cat > "$bundle_path/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleName</key><string>Showcase</string>
<key>CFBundleDisplayName</key><string>Rust UI Kit Showcase</string>
<key>CFBundleIdentifier</key><string>dev.rust-ui-kit.showcase</string>
<key>CFBundleExecutable</key><string>showcase</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>0.1.0</string>
<key>CFBundleVersion</key><string>1</string>
<key>LSMinimumSystemVersion</key><string>15.0</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>
PLIST
printf '%s\n' "$bundle_path"
