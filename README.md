# cqlib-tianyan

![Tianyan Quantum Computing](https://jiangsu-10.zos.ctyun.cn/qccp1/uiUpdate/img/logo.png)
> **天衍量子云平台** 的 Rust 同步客户端库——认证、设备管理、线路提交与读取误差矫正，一站式封装。

[English README](README.en.md) | [Rust 教程](docs/rust.cn.md) | [English Tutorial](docs/rust.en.md)

---

## ✨ 特性

| 功能 | 说明 |
|------|------|
| 🔐 **认证** | API Key 一键登录，凭据自动持久化与刷新 |
| 🖥️ **后端管理** | 列举所有量子计算机，获取拓扑与校准配置 |
| ⚛️ **任务提交** | 支持 QCIS 字符串和 `cqlib-core` Circuit 对象，单次最多50条线路 |
| 📊 **结果轮询** | 阻塞等待或非阻塞状态查询，内置超时与重试 |
| 🎯 **读取误差矫正** | Kronecker 积逆混淆矩阵，默认自动启用 |
| 🔌 **FFI 友好** | 同步阻塞 API，便于 Python（PyO3）、C（cbindgen）等语言绑定 |

---

## 🚀 快速开始

### 安装

```sh
cargo add cqlib-tianyan
```

### 认证

```rust
use cqlib_tianyan::TianyanPlatform;

// 首次登录（凭据保存至 ~/.cqlib/tianyan/credentials.json）
let api_key = std::env::var("TIANYAN_API_KEY").expect("set TIANYAN_API_KEY");
let platform = TianyanPlatform::login(&api_key)?;

// 后续运行（自动检测是否过期并刷新）
// let platform = TianyanPlatform::from_credentials()?;
```

### 提交线路并获取结果

```rust
use std::time::Duration;

// 选择后端
let backend = platform.get_backend("tianyan-287")?;

// 提交 Bell 态线路: CNOT(Q1→Q8) = H(Q8)·CZ(Q1,Q8)·H(Q8)
let task = backend.run(
    vec!["H Q1\nH Q8\nCZ Q1 Q8\nH Q8\nM Q1\nM Q8".into()],
    1000, // 采样次数
)?;

// 等待结果（默认自动应用读取误差矫正）
let results = task.wait(Duration::from_secs(120), Duration::from_secs(5))?;

for r in &results {
    println!("计数: {:?}", r.counts());
    println!("概率: {:?}", r.probabilities());
}
```

### 查看设备校准数据

```rust
let backend = platform.get_backend("tianyan-287")?;

// 查看拓扑
let device = backend.device_config()?;
println!("比特数: {}", device.topology().num_qubits());

// 查看读取保真度
if let Some(cal) = backend.readout_calibration_data()? {
    for (i, q) in cal.qubit_names.iter().enumerate() {
        println!("{}: f00={:.4}  f11={:.4}", q, cal.f00[i], cal.f11[i]);
    }
}
```

### 运行 E2E 示例

```sh
cargo run -p cqlib-tianyan --example e2e
```

---

## 📐 API 概览

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
├── wait(timeout, interval)              → Vec<ExecutionResult>  [尊重 calibration_mode]
├── wait_raw(timeout, interval)          → Vec<ExecutionResult>  [始终原始]
├── wait_calibrated(timeout, interval)   → Vec<ExecutionResult>  [始终矫正]
├── wait_with_calibration(timeout, interval, &cal) → Vec<ExecutionResult>  [手动指定校准数据]
└── status()                             → Vec<ExecutionResult>  [单次快照]
```

---

## 🎯 读取误差矫正

测量误差矫正使用**逆混淆矩阵**方法：

1. 从校准配置中读取每个量子比特的读取保真度 `f00[k]`（P(0|0)）和 `f11[k]`（P(1|1)）。
2. 为每个量子比特构建 2×2 混淆矩阵并求逆。
3. 以 Kronecker 积组合为 2ᴺ × 2ᴺ 完整逆矩阵。
4. 对观测概率向量施加逆矩阵，截断负值并重新归一化。

> **内存限制：** Kronecker 积矩阵大小为 O(4ᴺ)，N=14 时约 2 GiB，N=15 时超 8 GiB。  
> 因此 `CalibrationMode::Auto` 仅对 ≤14 个被测量子比特自动启用矫正；更大的线路需要显式指定 `CalibrationMode::Enabled`（由调用方承担内存责任）或使用 `CalibrationMode::Disabled` 禁用。

详细算法说明见 [docs/rust.cn.md](docs/rust.cn.md)。

---

## ⚙️ 认证配置选项

```rust
use cqlib_tianyan::config::TianyanConfig;

let cfg = TianyanConfig::default()
    .with_save_credentials(false) // 不写磁盘
    .with_auto_refresh(false)     // 不自动刷新
    .with_credentials_path("/custom/path/creds.json");
```

---

## 🐍 Python 绑定

`cqlib-tianyan` 提供了基于 [PyO3](https://pyo3.rs) 的 Python 绑定（`crates/binding-python`），可在 Python 中以原生风格调用所有核心功能。

```python
from cqlib_tianyan import TianyanPlatform, CalibrationMode
import os

# 登录
platform = TianyanPlatform.login(os.environ["TIANYAN_API_KEY"])

# 选择后端并提交线路
backend = platform.get_backend("tianyan-287")
task = backend.run(["H Q1\nH Q8\nCZ Q1 Q8\nH Q8\nM Q1\nM Q8"], shots=1000)

# 等待结果（Auto 模式下，≤14 比特自动矫正）
results = task.wait(timeout=120, poll_interval=5)
print(results[0].counts())

# 手动控制模式
task2 = backend.run_with_mode(["..."], shots=1000, mode=CalibrationMode.Enabled)
# 或使用字符串简写
task3 = backend.run_with_mode(["..."], shots=1000, mode="disabled")
```

详细文档见 [docs/python.cn.md](docs/python.cn.md) | [docs/python.en.md](docs/python.en.md)

---

## 📦 项目结构

```
crates/cqlib-tianyan/
├── src/
│   ├── lib.rs           # 公共 API 导出
│   ├── platform.rs      # TianyanPlatform 入口
│   ├── auth.rs          # 认证与凭据持久化
│   ├── client.rs        # HTTP 客户端（含重试）
│   ├── device.rs        # TianyanBackend
│   ├── device_config.rs # 校准配置解析
│   ├── task.rs          # TaskHandle + CalibrationMode
│   ├── calibration.rs   # 读取误差矫正算法
│   ├── config.rs        # TianyanConfig
│   └── error.rs         # TianyanError
├── examples/
│   └── e2e.rs           # 完整端到端示例
└── docs/
    ├── rust.cn.md       # 中文详细教程
    └── rust.en.md       # 英文详细教程
```

---

## 📄 许可证

Apache License 2.0 — 详见 [LICENSE.txt](LICENSE.txt)。

Copyright © China Telecom Quantum Group 2026



