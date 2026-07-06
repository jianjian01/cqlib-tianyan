# cqlib-tianyan

Python bindings for the Tianyan quantum cloud platform client.

## Overview

This package provides synchronous Python access to the Tianyan quantum computing cloud platform, allowing users to:

- Authenticate with API keys
- Discover available quantum backends
- Submit quantum circuits (QCIS format)
- Retrieve execution results with readout error mitigation

All blocking operations release the GIL, making the library safe for use in multi-threaded Python applications.

## Installation

### From Source (Development)

```bash
# Install maturin if not already installed
pip install maturin

# Build and install the package
maturin develop
```

### Production Build

```bash
maturin build --release
pip install target/wheels/*.whl
```

## Quick Start

```python
from cqlib_tianyan import TianyanPlatform

# Authenticate with your API key
platform = TianyanPlatform.login("your_api_key")

# List available backends
backends = platform.list_backends()
for backend in backends:
    print(f"{backend.name}: {backend.status}")

# Get a specific backend
backend = platform.get_backend("tianyan-287")
print(f"Physical qubits: {backend.num_qubits()}")

# Submit a circuit
task = backend.run(["H Q1\nM Q1"], shots=1000)

# Wait for results
results = task.wait(timeout_secs=120.0)
for result in results:
    print(f"Task ID: {result.task_id}")
    print(f"Counts: {result.counts}")
    print(f"Probabilities: {result.probabilities}")
```

## API Reference

### Core Classes

#### `TianyanPlatform`

The main entry point for interacting with the Tianyan platform.

```python
# Login with API key
platform = TianyanPlatform.login(
    api_key="your_key",
    domain="qc.zdxlz.com",  # Optional
    save_credentials=True,   # Optional, default: True
    auto_refresh=True,       # Optional, default: True
)

# Load from saved credentials
platform = TianyanPlatform.from_credentials()

# List all backends
backends = platform.list_backends()

# Get specific backend
backend = platform.get_backend("device_name")

# Submit circuits directly
 task = platform.submit(
    circuits=["H Q1\nM Q1"],
    shots=1000,
    device_name="tianyan-287"
)
```

#### `TianyanBackend`

Represents a quantum computing backend.

```python
# Backend properties
backend.name           # Device identifier
backend.display_name   # Human-readable name
backend.status         # DeviceStatus enum
backend.toll           # DeviceToll enum (free/paid)
backend.num_qubits()   # Physical qubit count (loads config on first use)

# Check availability
if backend.is_available():
    # Submit circuits
    task = backend.run(circuits=["H Q1\nM Q1"], shots=1000)

    # Raw execution (no calibration)
    task = backend.run_raw(circuits=["..."], shots=1000)

    # With explicit calibration mode
    task = backend.run_with_mode(
        circuits=["..."],
        shots=1000,
        mode="disabled"  # "auto", "enabled", "disabled"
    )

    # Get device configuration
    device = backend.device_config()  # Returns cqlib.device.Device
```

#### `TaskHandle`

Represents a batch of submitted circuits.

```python
# Task properties
task.task_ids       # List of platform-assigned task IDs
task.device_name    # Backend name
task.shots          # Number of shots
task.submitted_at   # ISO 8601 timestamp

# Non-blocking status check
results = task.status()  # Returns completed results only

# Blocking wait for all results
results = task.wait(
    timeout_secs=120.0,
    poll_interval_secs=5.0
)

# Wait for raw results
results = task.wait_raw(timeout_secs=120.0)
```

#### `TianyanConfig`

Configuration for the platform client.

```python
config = TianyanConfig(
    domain="qc.zdxlz.com",
    save_credentials=True,
    auto_refresh=True,
    credentials_path="~/.cqlib/tianyan/credentials.json"
)
```

#### Enums

```python
from cqlib_tianyan import DeviceStatus, DeviceToll, CalibrationMode

# DeviceStatus values: "running", "calibration", "under_maintenance", "offline", "unknown"
# DeviceToll values: "free", "paid", "unknown"
# CalibrationMode values: "auto", "enabled", "disabled"
```

## Exception Handling

```python
from cqlib_tianyan import TianyanError

try:
    platform = TianyanPlatform.login("invalid_key")
except Exception as e:
    # Handle authentication failure
    print(f"Login failed: {e}")
```

**Note:** In abi3 mode, `TianyanError` cannot be caught by type. Catch `Exception` instead.

## Testing

### Test Structure

```
tests/
├── conftest.py              # Pytest configuration and fixtures
└── test_cqlib_tianyan.py    # Main test file
```

### Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `TIANYAN_API_KEY` | For integration tests | Your Tianyan platform API key |
| `TIANYAN_DOMAIN` | Optional | Custom domain (defaults to qc.zdxlz.com) |
| `TIANYAN_DEVICE` | Optional | Target device name (e.g., "tianyan-287") |

### Running Tests

```bash
# Run all tests (requires API key)
TIANYAN_API_KEY="your_key" pytest tests/ -v

# Run with specific device
TIANYAN_API_KEY="your_key" TIANYAN_DEVICE="tianyan-287" pytest tests/ -v

# Run only unit tests (no API key required)
pytest tests/ -v -m "not integration"

# Run integration tests only
TIANYAN_API_KEY="your_key" pytest tests/ -v -m integration

# Include slow tests (long timeouts)
TIANYAN_API_KEY="your_key" pytest tests/ -v --run-slow

# Run specific test class
TIANYAN_API_KEY="your_key" pytest tests/test_cqlib_tianyan.py::TestCircuitSubmission -v
```

### Test Markers

| Marker | Description |
|--------|-------------|
| `integration` | Requires API access and network connectivity |
| `slow` | Long-running tests (waits for circuit execution) |

## Technical Details

### Architecture

This package is built with:

- **PyO3**: Rust-Python bindings
- **Maturin**: Build tool for Python-Rust hybrid projects
- **abi3**: Stable Python ABI (supports Python 3.10+)

### Module Structure

```
crates/binding-python/
├── Cargo.toml           # Rust package manifest
├── pyproject.toml       # Python package configuration
├── src/
│   ├── lib.rs          # Module initialization
│   ├── error.rs        # Exception types
│   ├── config.rs       # TianyanConfig bindings
│   ├── platform.rs     # TianyanPlatform bindings
│   ├── backend.rs      # TianyanBackend bindings
│   └── task.rs         # TaskHandle bindings
├── cqlib_tianyan/
│   ├── __init__.py     # Python package entry
│   ├── __init__.pyi    # Type stubs
│   └── py.typed        # PEP 561 marker
└── tests/
    ├── conftest.py     # Pytest configuration
    └── test_cqlib_tianyan.py  # Test suite
```

### Known Limitations

1. **ABI3 Mode**: Due to `abi3-py310` feature:
   - `TianyanError` cannot inherit from `RuntimeError`
   - `TianyanError` cannot be instantiated directly in Python
   - Exception catching must use `except Exception:` instead of `except TianyanError:`

2. **GIL Release**: Long-running operations (`wait()`, `wait_raw()`) release the GIL to allow other Python threads to run.

## Dependencies

- Python >= 3.10
- `cqlib` >= 0.1.0 (Python package)
- `cqlib-core` (Rust workspace dependency)
- `cqlib-tianyan` (Rust workspace dependency)

## License

Licensed under the Apache License, Version 2.0.

Copyright (c) 2026 China Telecom Quantum Group
