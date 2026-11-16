# Third party notices

The repository's 13 semantic SVGs are original and covered by Apache-2.0. No external icon collection or font files are copied into this repository.

The application links GPUI Kit 0.9.1, GPUI Component, GPUI Base and GPUI snapshots from crates.io under Apache-2.0. GPUI Kit's optional default asset bundle includes upstream icon assets. The default asset subset uses Lucide icons under the ISC license; that dependency includes LICENSE-LUCIDE, also retained at assets/icons/LICENSE-LUCIDE. The web documentation also mentions Isocons, but this repository does not select or copy that collection. Retain the dependency notices when distributing a packaged application. Cargo.lock enumerates the full dependency graph; dependency licenses must be reviewed for the specific packaged target before a public binary release.

GPUI Base 0.9.1 is temporarily vendored under vendor/gpui-base with one functional divider drag-start patch. Its original Apache-2.0 license, source provenance, published archive checksum and complete code delta are retained there. This does not introduce another renderer or component engine.
