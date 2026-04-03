# cqlib-tianyan

<div align="center">

![Tianyan Quantum Computing](https://jiangsu-10.zos.ctyun.cn/qccp1/uiUpdate/img/logo.png)

**天衍量子云平台**客户端库——支持 Rust、Python 和 C

[English](README.en.md) · [Rust 教程](docs/rust.cn.md) · [Python 教程](docs/python.cn.md) · [C 教程](docs/C.cn.md)

</div>

---

## 简介

`cqlib-tianyan` 是访问[天衍量子云平台](https://qc.zdxlz.com)（`qc.zdxlz.com`）的客户端库，提供：

- 🔐 **认证**：API Key 一键登录，凭据自动持久化与刷新
- 🖥️ **后端管理**：列举量子计算机，查看拓扑与校准配置
- ⚛️ **任务提交**：提交 QCIS 线路，支持批量（自动拆分，单批最多 50 条）
- 📊 **结果获取**：阻塞等待或非阻塞状态查询
- 🎯 **读取误差矫正**：逆混淆矩阵方法，默认自动启用（≤ 14 比特）

核心库用 Rust 实现，并提供 **Python**（PyO3）和 **C**（cbindgen）两种语言绑定。

---

## 快速开始

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
    println!("计数: {:?}", r.counts());
}
```

→ [完整 Rust 教程](docs/rust.cn.md)

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

→ [完整 Python 教程](docs/python.cn.md)

### C

```bash
# 构建库（在 workspace 根目录执行）
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

→ [完整 C 教程](docs/C.cn.md)

---

## 语言绑定

| 语言 | crate/模块 | 文档 |
|------|-----------|------|
| **Rust** | `crates/cqlib-tianyan` | [中文](docs/rust.cn.md) · [English](docs/rust.en.md) |
| **Python** | `crates/binding-python` | [中文](docs/python.cn.md) · [English](docs/python.en.md) |
| **C** | `crates/binding-c` | [中文](docs/C.cn.md) · [English](docs/C.en.md) |

---

## 项目结构

```
cqlib-tianyan/
├── crates/
│   ├── cqlib-tianyan/     # 核心 Rust 库
│   ├── binding-python/    # Python 绑定（PyO3）
│   └── binding-c/         # C 绑定（cbindgen）
└── docs/                  # 各语言详细教程
```

---

## 许可证

[Apache License 2.0](LICENSE.txt) · Copyright © China Telecom Quantum Group 2026
