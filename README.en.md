# cqlib-tianyan

<div align="center">

![Tianyan Quantum Computing](https://jiangsu-10.zos.ctyun.cn/qccp1/uiUpdate/img/logo.png)

Client library for the **Tianyan Quantum Cloud Platform** — Rust, Python, and C

[中文](README.md) · [Rust Tutorial](docs/rust.en.md) · [Python Tutorial](docs/python.en.md) · [C Tutorial](docs/C.en.md)

</div>

---

## Overview

`cqlib-tianyan` is a client library for the [Tianyan Quantum Cloud Platform](https://qc.zdxlz.com) (`qc.zdxlz.com`), providing:

- 🔐 **Authentication** — API-key login with automatic credential persistence and token refresh
- 🖥️ **Backend discovery** — list quantum computers, inspect topology and calibration data
- ⚛️ **Circuit submission** — submit QCIS circuits in bulk (auto-batched, up to 50 per request)
- 📊 **Result polling** — blocking wait or non-blocking status snapshot
- 🎯 **Readout error mitigation** — inverse confusion matrix method, enabled by default (≤ 14 qubits)

The core library is implemented in Rust and ships with **Python** (PyO3) and **C** (cbindgen) bindings.

---

## Quick Start

### Rust

```sh
cargo add cqlib-tianyan
```

```rust
use cqlib_tianyan::TianyanPlatform;
use std::time::Duration;

let platform = TianyanPlatform::login(&std::env::var("TIANYAN_API_KEY")?)?;
let backend  = platform.get_backend("tianyan-287")?;
let task     = backend.run(vec!["H Q1\nH Q8\nCZ Q1 Q8\nH Q8\nM Q1\nM Q8".into()], 1000)?;
let results  = task.wait(Duration::from_secs(120), Duration::from_secs(5))?;

for r in &results {
    println!("counts: {:?}", r.counts());
}
```

→ [Full Rust Tutorial](docs/rust.en.md)

### Python

```sh
pip install cqlib-tianyan
```

```python
import os
from cqlib_tianyan import TianyanPlatform

platform = TianyanPlatform.login(os.environ["TIANYAN_API_KEY"])
backend  = platform.get_backend("tianyan-287")
task     = backend.run(["H Q1\nH Q8\nCZ Q1 Q8\nH Q8\nM Q1\nM Q8"], shots=1000)
results  = task.wait(timeout=120, poll_interval=5)

print(results[0].counts())
```

→ [Full Python Tutorial](docs/python.en.md)

### C

```bash
# Build the library (from the workspace root)
cargo build -p binding-c --release
```

```c
#include "cqlib_tianyan.h"

TianyanPlatformC *p = tianyan_platform_login(getenv("TIANYAN_API_KEY"));
TianyanBackendC  *b = tianyan_platform_get_backend(p, "tianyan-287");

const char *circuits[] = {"H Q1\nH Q8\nCZ Q1 Q8\nH Q8\nM Q1\nM Q8"};
TianyanTaskC     *t = tianyan_backend_run(b, circuits, 1, 1000);
TianyanResultList *r = tianyan_task_wait(t, 120.0, 5.0);

printf("counts: %s\n", tianyan_result_counts_json(r, 0));

tianyan_result_list_free(r);
tianyan_task_free(t);
tianyan_backend_free(b);
tianyan_platform_free(p);
```

→ [Full C Tutorial](docs/C.en.md)

---

## Language Bindings

| Language | Crate / Module | Documentation |
|----------|---------------|---------------|
| **Rust** | `crates/cqlib-tianyan` | [中文](docs/rust.cn.md) · [English](docs/rust.en.md) |
| **Python** | `crates/binding-python` | [中文](docs/python.cn.md) · [English](docs/python.en.md) |
| **C** | `crates/binding-c` | [中文](docs/C.cn.md) · [English](docs/C.en.md) |

---

## Repository Layout

```
cqlib-tianyan/
├── crates/
│   ├── cqlib-tianyan/     # Core Rust library
│   ├── binding-python/    # Python bindings (PyO3)
│   └── binding-c/         # C bindings (cbindgen)
└── docs/                  # Per-language detailed tutorials
```

---

## Building

This project uses **Make** to orchestrate multi-language builds. You can also use **Cargo** directly for the Rust parts.

### Using Make (recommended for macOS/Linux)

```bash
make all       # Build everything (Rust + C + Python wheel)
make rust      # Build Rust only (excludes Python bindings)
make python    # Build Python wheel (requires maturin)
make c         # Build C bindings and run tests
make test      # Run all tests
make clean     # Clean all build artifacts
```

### Using Cargo

```bash
# Build Rust crates (automatically excludes Python bindings which need maturin)
cargo build --workspace --exclude binding-python

# Run tests
cargo test --workspace --exclude binding-python
```

### Windows

Windows does not include Make by default. Use Cargo + Maturin directly:

```powershell
# Build Rust parts
cargo build --workspace --exclude binding-python

# Build Python bindings (install maturin first: pip install maturin)
cd crates/binding-python
maturin build --release
```

Or use Git Bash / WSL for Make support.

---

## License

[Apache License 2.0](LICENSE.txt) · Copyright © China Telecom Quantum Group 2026
