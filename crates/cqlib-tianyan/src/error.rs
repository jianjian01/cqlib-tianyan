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

//! Error types for the Tianyan quantum cloud platform client.

use thiserror::Error;

/// All errors that can occur when interacting with the Tianyan cloud platform.
#[derive(Debug, Error)]
pub enum TianyanError {
    /// An HTTP-level transport error.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// The server returned a non-zero business error code.
    #[error("API error (code={code}): {message}")]
    Api { code: i64, message: String },

    /// Authentication or credential-related failure.
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// File system I/O error (reading/writing credential files, etc.).
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// A polling or wait operation exceeded the allowed duration.
    #[error("Operation timed out after {0:?}")]
    Timeout(std::time::Duration),

    /// The requested device name was not found in the platform's device list.
    #[error("Device not found: '{0}'")]
    DeviceNotFound(String),

    /// The caller supplied invalid parameters (e.g. empty circuit list, zero shots).
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Failed to convert a `Circuit` object into a QCIS string for submission.
    #[error("Circuit-to-QCIS conversion error: {0}")]
    CircuitConversion(String),
}
