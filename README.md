# GitHub Actions experiments for Rust

![Build](https://github.com/pingw33n/ghatest/workflows/Build/badge.svg)

## build.yml

For verifying master branch, pull requests.

Matrix is configured for multiple OS/archs. Runs `cargo clippy`, `cargo test`, `cargo build`.

sdl2 with "bundled" feature is built with correctly set vcvars for VS 2019.

Cache is configured for `target/release` directory.

Artifacts are uploaded as `<NAME>-<GIT_HASH>-<OS>-<ARCH>.zip`: `ghatest-7ebd4c7-windows-x86_64.zip`.
 
 ## snapshot.yml
 
 For doing snapshot (nightly) releases.
 
 Triggers on `snapshot-*` tag, e.g. `snapshot-2020-02-14`.
 
 Build process is similar to `build.yml` but has `x86` in `matrix.arch`.
 
 Creates a new release named after the tag name. Artifacts are uploaded as release assets.