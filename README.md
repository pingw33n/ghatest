# GitHub Actions experiments for Rust

![Build](https://github.com/pingw33n/ghatest/workflows/Build/badge.svg)

## build.yml

* Runs after master branch push, on pull requests.
* Matrix is configured for multiple OS/archs.
* Runs `cargo clippy`, `cargo test`, `cargo build`.
* `sdl2` with `bundled` feature is built with correctly set `vcvars` for VS 2019.
* Cache is configured for `target/release` directory.
* Artifacts are uploaded as `<NAME>-<GIT_HASH>-<OS>-<ARCH>.zip`, e.g. `ghatest-7ebd4c7-windows-x86_64.zip`.
 
 ## snapshot.yml
 
 * Runs on `snapshot-*` tag push, e.g. `snapshot-2020-02-14`. Intended for doing snapshot (nightly) releases.
 * Matrix is similar to `build.yml` but additionally has `x86` in `matrix.arch`.
 * Creates a new release named after the tag name. Artifacts are uploaded as release assets.
 
 ## nightly.tml
 
 * Triggered on cron schedule.
 * Checks for an existing `snapshot-*` tag on HEAD. Creates one using the current date if missing.
 * The new tag push triggers `snapshot.yml`.
 * Requires personal access token in `PAT` secret in order for the push to trigger the workflow (Github limitation).