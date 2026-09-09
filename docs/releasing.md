# CI and Manual Releases

The workflow layout follows [cqlib](https://github.com/cq-lib/cqlib): entry-point
workflows, reusable workflows, and `tools/ci.py`. Workflows do not publish to
PyPI or crates.io.

| File | Responsibility |
| --- | --- |
| `.github/workflows/ci.yml` | PR entry point: lint plus development tests on three operating systems |
| `.github/workflows/lint.yml` | Reusable Rust fmt / Clippy, Python Ruff, and C formatting checks |
| `.github/workflows/tests.yml` | Reusable Rust workspace, offline pytest, and C static/shared linking tests |
| `.github/workflows/wheels.yml` | Tag entry point: validates versions, runs five-platform tests, then builds |
| `.github/workflows/wheels-build.yml` | Builds wheels and C SDKs for five platforms, plus sdist and `.crate` |
| `tools/ci.py` | Version checks, C SDK packaging/tests, installed-wheel tests, source packages |
| `tools/c-sdk/` | CMake test configuration and README distributed with each C SDK |

Rust 1.97.1 is pinned only in CI. The crate MSRV remains `1.85`. Ruff `0.15.9`
and clang-format `22.1.2` match this repository's pre-commit configuration.

## Trigger Behavior

Pull requests run `cargo test --workspace --locked`, pytest with
`-m "not integration"` against a development install, and C linking tests on
Linux x86_64, Windows x86_64, and macOS arm64. Integration tests that need
`TIANYAN_API_KEY` are not run in CI.

Tags first check versions, then run the same development tests on all five
platforms. Distribution packages are built only after those checks pass.
Workflows have `contents: read` only. They create no GitHub Releases and publish
nothing.

## Tags and Versions

This repository has not been published before. The first prerelease uses the
same number for Python and Rust/C:

- Tag: `v0.1.0-beta.1`
- Python source version: `0.1.0-beta.1` (PEP 440 / wheel: `0.1.0b1`)
- Rust/C workspace version: `0.1.0-beta.1`

Later releases may let Python and Rust/C versions diverge, as in cqlib. Tags
always match the Python version:

- Stable: `vX.Y.Z`
- Prerelease: `vX.Y.Z-alpha.N`, `vX.Y.Z-beta.N`, or `vX.Y.Z-rc.N` (`N` starts at 1)

CI checks the tag shape, Python `>=3.10`, `abi3-py310`, and that both bindings
pin `cqlib-tianyan` to the workspace version.

## Platforms and Artifacts

| Platform | Runner / Build Environment |
| --- | --- |
| Windows x86_64 | windows-2022 / MSVC |
| Linux glibc 2.28+ x86_64 | ubuntu-24.04 / manylinux_2_28_x86_64 |
| Linux glibc 2.28+ aarch64 | ubuntu-24.04-arm / manylinux_2_28_aarch64 |
| macOS 11+ x86_64 | macos-15-intel / deployment target 11.0 |
| macOS 11+ arm64 | macos-15 / deployment target 11.0 |

Each platform produces one `cp310-abi3` wheel. The same wheel is installed into
CPython 3.10–3.14 for import checks, `pip check`, and offline pytest. Linux
wheels are built with `maturin --auditwheel repair` (and `maturin[patchelf]`)
so OpenSSL (`libssl`, `libcrypto`) is vendored into the wheel; `--auditwheel
check` rejects those external shared libraries. Each
platform also produces one C SDK archive (ZIP on Windows, tar.gz elsewhere)
containing `cqlib_tianyan.h`, static and shared libraries, README, LICENSE.txt,
and `tests/test_api.c`.

The source job generates the Python sdist only. `cqlib-tianyan` depends on
`cqlib-core` via git and does not declare a crates.io version, so
`cargo package` exits before producing a `.crate`. Do not add a placeholder
registry version: Cargo would rewrite the git source to crates.io and fail
verification. Publish the Rust crate only after `cqlib-core` is on crates.io
and the dependency is switched to a registry version.

The Python package does not hard-depend on PyPI `cqlib>=0.1.0` (that range
installs classic `cqlib 1.3.11`, which lacks `cqlib.device` and
`cqlib.circuit`). Install the companion extra `cqlib-tianyan[cqlib]` when the
Rust-backed `cqlib>=1.4.0b1` is published. `device_config()` and result
conversion call those modules at runtime.

## Downloading and Publishing

Wait for the entire tag workflow to succeed. Download the five
`release-<tag>-<platform>` artifacts and `release-<tag>-sources`. Together they
contain 11 files: 5 wheels, 1 sdist, and 5 C SDK archives. There is no
`.crate` until `cqlib-core` is a registry dependency.

- PyPI: manually review and upload the 5 wheels and sdist with Twine.
- crates.io: only after the `cqlib-core` git dependency is replaced by a
  registry version. Publish `cqlib-tianyan` only. `binding-python` and
  `binding-c` are not published.
- GitHub Releases: maintainers create a Release at the same tag and attach the
  C SDK archives.
