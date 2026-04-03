// This code is part of Cqlib.
//
// (C) Copyright China Telecom Quantum Group 2026
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.

//! C-compatible configuration struct for the Tianyan platform client.
//!
//! Pass a pointer to `TianyanConfigC` to `tianyan_platform_login_with_config()`
//! or `tianyan_platform_from_credentials_with_config()` for custom settings.
//! Pass `NULL` to use the default production configuration.
//!
//! All string fields may be `NULL` to select the built-in defaults.

use cqlib_tianyan::config::TianyanConfig;
use std::os::raw::c_char;

use crate::error::cstr_to_str;

/// Runtime configuration for the Tianyan platform client.
///
/// All pointer fields may be `NULL` to use built-in defaults.
#[repr(C)]
pub struct TianyanConfigC {
    /// Platform hostname, e.g. `"qc.zdxlz.com"`. `NULL` → default.
    pub domain: *const c_char,
    /// File path for the JSON credentials file. `NULL` → default platform path.
    pub credentials_path: *const c_char,
    /// Whether to persist credentials to disk after login/refresh (default: `true`).
    pub save_credentials: bool,
    /// Whether to re-login automatically when the stored token expires (default: `true`).
    pub auto_refresh: bool,
}

/// Build a [`TianyanConfig`] from a nullable C config pointer.
///
/// `NULL` → `TianyanConfig::default()`.
pub(crate) fn config_from_c(ptr: *const TianyanConfigC) -> Result<TianyanConfig, String> {
    if ptr.is_null() {
        return Ok(TianyanConfig::default());
    }
    // SAFETY: caller guarantees `ptr` is a valid, non-null `TianyanConfigC`.
    let c = unsafe { &*ptr };

    let mut cfg = if c.domain.is_null() {
        TianyanConfig::default()
    } else {
        let domain = cstr_to_str(c.domain, "domain")?;
        TianyanConfig::custom(domain)
    };

    cfg = cfg.with_save_credentials(c.save_credentials);
    cfg = cfg.with_auto_refresh(c.auto_refresh);

    if !c.credentials_path.is_null() {
        let path = cstr_to_str(c.credentials_path, "credentials_path")?;
        cfg = cfg.with_credentials_path(std::path::PathBuf::from(path));
    }

    Ok(cfg)
}
