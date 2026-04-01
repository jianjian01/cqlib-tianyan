# cqlib-tianyan

> Synchronous Rust client for the **Tianyan Quantum Cloud Platform** — authentication, device management, circuit submission, and readout error mitigation in one clean API.

[中文 README](README.md) | [English Tutorial](docs/rust.en.md) | [中文教程](docs/rust.cn.md)

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🔐 **Authentication** | Single-call API-key login with automatic credential persistence and refresh |
| 🖥️ **Backend discovery** | List quantum computers, inspect topology and calibration data |
| ⚛️ **Task submission** | QCIS strings or `cqlib-core` `Circuit` objects; up to 50 circuits per batch |
| 📊 **Result polling** | Blocking wait or non-blocking status snapshot, with timeout and retry |
| 🎯 **Readout calibration** | Kronecker-product inverse confusion matrix applied automatically by default |
| 🔌 **FFI-friendly** | Synchronous blocking API — easy to bind from Python (PyO3), C (cbindgen), Java (JNI) |

---

## 🚀 Quick Start

### Install

```sh
cargo add cqlib-tianyan
```

### Authenticate

```rust
use cqlib_tianyan::TianyanPlatform;

// First-time login (saves credentials to ~/.cqlib/tianyan/credentials.json)
let api_key = std::env::var("TIANYAN_API_KEY").expect("set TIANYAN_API_KEY");
let platform = TianyanPlatform::login(&api_key)?;

// Subsequent runs (auto-refreshes if expired)
// let platform = TianyanPlatform::from_credentials()?;
```

### Submit a circuit and get results

```rust
use std::time::Duration;

// Select a backend
let backend = platform.get_backend("tianyan-287")?;

// Bell state: CNOT(ctrl=Q1, tgt=Q8) = H(Q8) · CZ(Q1,Q8) · H(Q8)
let task = backend.run(
    vec!["H Q1\nH Q8\nCZ Q1 Q8\nH Q8\nM Q1\nM Q8".into()],
    1000, // shots
)?;

// Wait for results (readout calibration applied automatically by default)
let results = task.wait(Duration::from_secs(120), Duration::from_secs(5))?;

for r in &results {
    println!("Counts : {:?}", r.counts());
    println!("Probs  : {:?}", r.probabilities());
}
```

### Inspect calibration data

```rust
let backend = platform.get_backend("tianyan-287")?;

// Topology
let device = backend.device_config()?;
println!("Qubits   : {}", device.topology().num_qubits());
println!("Couplings: {}", device.topology().num_couplings());

// Per-qubit readout fidelities
if let Some(cal) = backend.readout_calibration_data()? {
    for (i, q) in cal.qubit_names.iter().enumerate() {
        println!("{}: f00={:.4}  f11={:.4}", q, cal.f00[i], cal.f11[i]);
    }
}
```

### Run the E2E example

```sh
cargo run -p cqlib-tianyan --example e2e
```

---

## 📐 API Overview

```
TianyanPlatform
├── login(api_key)                        → TianyanPlatform
├── login_with_config(api_key, config)    → TianyanPlatform
├── from_credentials()                    → TianyanPlatform
├── from_credentials_with_config(config)  → TianyanPlatform
├── list_backends()                       → Vec<TianyanBackend>
└── get_backend(name)                     → TianyanBackend

TianyanBackend
├── device_config()                       → Device (cqlib-core)
├── readout_calibration_data()            → Option<ReadoutCalibrationData>
├── run(circuits, shots)                  → TaskHandle  [CalibrationMode::Auto]
├── run_raw(circuits, shots)              → TaskHandle  [CalibrationMode::Disabled]
└── run_with_mode(circuits, shots, mode)  → TaskHandle

TaskHandle
├── wait(timeout, interval)              → Vec<ExecutionResult>  [respects calibration_mode]
├── wait_raw(timeout, interval)          → Vec<ExecutionResult>  [always raw]
├── wait_calibrated(timeout, interval)   → Vec<ExecutionResult>  [always calibrated]
├── wait_with_calibration(…, f00, f11)   → Vec<ExecutionResult>  [explicit fidelities]
└── status()                             → Vec<ExecutionResult>  [single snapshot]
```

---

## 🎯 Readout Error Calibration

Readout correction uses the **inverse confusion matrix** method:

1. Download per-qubit readout fidelities `f00[k]` (P(0|0)) and `f11[k]` (P(1|1)) from calibration config.
2. Build and invert a 2×2 confusion matrix for each qubit.
3. Combine into a 2ᴺ × 2ᴺ full inverse via Kronecker product.
4. Apply the inverse to the measured probability vector, clamp negatives to 0, and renormalise.

The default `CalibrationMode::Auto` applies correction when data is available and silently falls back to raw counts otherwise. See [docs/rust.en.md](docs/rust.en.md) for the full algorithm derivation.

---

## ⚙️ Configuration Options

```rust
use cqlib_tianyan::config::TianyanConfig;

let cfg = TianyanConfig::default()
    .with_save_credentials(false)          // in-memory only
    .with_auto_refresh(false)              // fail on expiry instead of re-logging in
    .with_credentials_path("/vault/creds.json"); // custom path
```

### CalibrationMode

```rust
use cqlib_tianyan::task::CalibrationMode;

let task = backend.run_with_mode(circuits, shots, CalibrationMode::Enabled)?;  // require calibration
let task = backend.run_raw(circuits, shots)?;                                   // skip calibration
```

---

## 📦 Project Layout

```
crates/cqlib-tianyan/
├── src/
│   ├── lib.rs           # Public API re-exports
│   ├── platform.rs      # TianyanPlatform entry point
│   ├── auth.rs          # Authentication & credential persistence
│   ├── client.rs        # HTTP client (with retry)
│   ├── device.rs        # TianyanBackend
│   ├── device_config.rs # Calibration config parser
│   ├── task.rs          # TaskHandle + CalibrationMode
│   ├── calibration.rs   # Readout error mitigation
│   ├── config.rs        # TianyanConfig
│   └── error.rs         # TianyanError
├── examples/
│   └── e2e.rs           # Full end-to-end demo
docs/
├── rust.en.md           # Detailed English tutorial
└── rust.cn.md           # 详细中文教程
```

---

## 🧪 Running Tests

```sh
cargo test -p cqlib-tianyan
```

---

## 📄 License

Apache License 2.0 — see [LICENSE.txt](LICENSE.txt).

Copyright © China Telecom Quantum Group 2026
