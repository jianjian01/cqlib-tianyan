# cqlib-tianyan C Bindings

C ABI bindings for [cqlib-tianyan](../cqlib-tianyan), the Rust client for the **Tianyan Quantum Cloud Platform**. The C header `include/cqlib_tianyan.h` is generated automatically by [cbindgen](https://github.com/mozilla/cbindgen) via `build.rs` on every `cargo build`.

For a user-facing tutorial see [docs/C.en.md](../../docs/C.en.md) | [docs/C.cn.md](../../docs/C.cn.md).

---

## Project Layout

```
crates/binding-c/
├── Cargo.toml          # crate metadata and dependencies
├── build.rs            # invokes cbindgen → generates include/cqlib_tianyan.h
├── cbindgen.toml       # cbindgen configuration
├── Makefile            # convenience targets: make test, make example
├── include/
│   └── cqlib_tianyan.h # auto-generated C header (do not edit manually)
├── src/
│   ├── lib.rs          # module declarations
│   ├── error.rs        # thread-local last-error + helper functions
│   ├── config.rs       # TianyanConfigC (#[repr(C)] struct)
│   ├── platform.rs     # TianyanPlatformC + platform functions
│   ├── backend.rs      # TianyanBackendC + backend functions
│   └── task.rs         # TianyanTaskC, TianyanResultList + task/result functions
├── example/
│   └── basic_usage.c   # full workflow demo (login → submit → wait → print)
└── tests/
    └── test_api.c      # offline tests (NULL-safety, error paths, string lifetime)
```

---

## Building

Run from the **workspace root**:

```bash
# Debug (for development and testing)
cargo build -p binding-c

# Release
cargo build -p binding-c --release
```

`build.rs` calls cbindgen automatically, regenerating `include/cqlib_tianyan.h` whenever source files change. **No manual cbindgen invocation is needed.**

Output artifacts (macOS example):

| File | Description |
|------|-------------|
| `target/debug/libbinding_c.a` | static library (debug) |
| `target/debug/libbinding_c.dylib` | dynamic library (debug) |
| `target/release/libbinding_c.a` | static library (release) |
| `crates/binding-c/include/cqlib_tianyan.h` | generated C header |

> On Linux the dynamic library is `.so`; on Windows it is `.dll` with a `.dll.lib` import library.

---

## Using the Makefile

Linking a Rust static library from C requires several platform-specific system libraries (Security, CoreFoundation, and SystemConfiguration frameworks on macOS; pthread/dl/m on Linux). The `Makefile` captures these flags so you don't have to remember them.

```bash
# From crates/binding-c/
make          # build debug library (cargo build -p binding-c)
make release  # build release library
make test     # compile tests/test_api.c and run it (38/38 offline tests)
make example  # compile example/basic_usage.c and run it (requires TIANYAN_API_KEY)
make clean    # remove compiled C binaries
```

If you prefer not to use Make, the equivalent manual commands are shown in the sections below.

---

## Running the Offline Test Suite

`tests/test_api.c` requires no network and no API key. It covers NULL-safety for all 38 API functions, error-path contracts, and string lifecycle.

```bash
make test
# Expected last line: === Results: 38/38 passed ===
```

Manual equivalent (macOS):

```bash
cargo build -p binding-c
cc -Iinclude tests/test_api.c \
   ../../target/debug/libbinding_c.a \
   -framework Security -framework CoreFoundation \
   -framework SystemConfiguration -lc++ \
   -o tests/test_api
tests/test_api
```

---

## Running the Example

`example/basic_usage.c` demonstrates the full workflow. Requires a valid API key and network access.

```bash
export TIANYAN_API_KEY="your_api_key_here"
make example
```

Manual equivalent (macOS):

```bash
cargo build -p binding-c --release
cc -Iinclude example/basic_usage.c \
   ../../target/release/libbinding_c.a \
   -framework Security -framework CoreFoundation \
   -framework SystemConfiguration -lc++ \
   -o example/basic_usage
example/basic_usage
```

---

## API Reference

### Opaque handle types

| Type | Created by | Freed by |
|------|-----------|----------|
| `TianyanPlatformC *` | `tianyan_platform_login()` / `from_credentials()` | `tianyan_platform_free()` |
| `TianyanBackendC *` | `tianyan_platform_get_backend()` | `tianyan_backend_free()` |
| `TianyanBackendC **` | `tianyan_platform_list_backends()` | `tianyan_backend_list_free(ptr, len)` |
| `TianyanTaskC *` | `tianyan_backend_run*()` / `tianyan_platform_submit()` | `tianyan_task_free()` |
| `TianyanResultList *` | `tianyan_task_wait*()` / `status_snapshot()` | `tianyan_result_list_free()` |

All `_free()` functions are **NULL-safe**.

### Config struct

```c
typedef struct TianyanConfigC {
    const char *domain;            // NULL → "qc.zdxlz.com"
    const char *credentials_path;  // NULL → ~/.cqlib/tianyan/credentials.json
    bool save_credentials;         // persist credentials to disk (default true)
    bool auto_refresh;             // re-login on token expiry (default true)
} TianyanConfigC;
```

### Error handling pattern

```c
TianyanPlatformC *p = tianyan_platform_login(api_key);
if (!p) {
    char *err = tianyan_last_error();
    fprintf(stderr, "Login failed: %s\n", err ? err : "(unknown)");
    tianyan_string_free(err);
    return 1;
}
```

### CalibrationMode (for `tianyan_backend_run_with_mode`)

| Value | Meaning |
|-------|---------|
| `0` | Auto — apply correction when ≤14 measured qubits and data available (default) |
| `1` | Enabled — force correction (caller ensures memory) |
| `2` | Disabled — always return raw counts |

### Memory ownership

| Return type | Ownership | Free with |
|-------------|-----------|-----------|
| `char *` (mutable) | Caller owns | `tianyan_string_free()` |
| `const char *` | Borrowed (lifetime = parent object) | **Do not free** |
| `char **` from `task_ids` | Caller owns | `tianyan_task_ids_free(ptr, len)` |
| All opaque handles | Caller owns | Respective `_free()` |

---

## Generating the Header Manually (optional)

To preview what cbindgen will generate without a full `cargo build`:

```bash
# Install cbindgen >= 0.29
cargo install cbindgen

# From crates/binding-c/
cbindgen --config cbindgen.toml --lang C
```
