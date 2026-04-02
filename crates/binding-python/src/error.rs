// This code is part of Cqlib.
//
// (C) Copyright China Telecom Quantum Group 2026
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
//
// Any modifications or derivative works of this code must retain this
// copyright notice, and modified files need to carry a notice indicating
// that they have been altered from the originals.

//! Python exception type for Tianyan platform errors.
//!
//! All [`cqlib_tianyan::TianyanError`] variants are mapped to the single
//! Python exception `TianyanError(RuntimeError)` so callers can use a
//! simple `except TianyanError` clause.

use cqlib_tianyan::TianyanError;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

/// Python exception class: `TianyanError(RuntimeError)`.
///
/// Raised by all Tianyan platform operations when an error occurs.
///
/// # Python Example
///
/// ```python
/// from cqlib_tianyan import TianyanError, TianyanPlatform
///
/// try:
///     platform = TianyanPlatform.login("invalid-key")
/// except TianyanError as e:
///     print(f"Login failed: {e}")
/// ```
#[pyclass(name = "TianyanError", subclass, module = "cqlib_tianyan")]
pub struct PyTianyanError;

/// Convert a [`TianyanError`] into a Python [`PyErr`] (`TianyanError`).
///
/// Registered as the canonical conversion so all `Result<_, TianyanError>`
/// returns work with the `?` operator inside `#[pymethods]`.
pub fn tianyan_error_to_pyerr(py: Python<'_>, e: TianyanError) -> PyErr {
    let err_type = py.get_type::<PyTianyanError>();
    match err_type.call1((e.to_string(),)) {
        Ok(obj) => PyErr::from_value(obj),
        Err(_) => PyRuntimeError::new_err(e.to_string()),
    }
}

/// Convenience trait so `Result<T, TianyanError>` can be converted with `.map_py_err(py)`.
pub trait IntoPyResult<T> {
    fn map_py_err(self, py: Python<'_>) -> PyResult<T>;
}

impl<T> IntoPyResult<T> for Result<T, TianyanError> {
    fn map_py_err(self, py: Python<'_>) -> PyResult<T> {
        self.map_err(|e| tianyan_error_to_pyerr(py, e))
    }
}
