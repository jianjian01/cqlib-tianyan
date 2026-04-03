// This code is part of Cqlib.
//
// (C) Copyright China Telecom Quantum Group 2026
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.

//! C API for [`TianyanPlatform`] — the top-level entry point.
//!
//! # Memory ownership
//!
//! Functions returning `*mut TianyanPlatform` transfer ownership to the caller.
//! The caller **must** eventually call `tianyan_platform_free()` on the pointer.
//!
//! # Error handling
//!
//! All functions return `NULL` on failure. Call `tianyan_last_error()` to retrieve
//! the error description, then free it with `tianyan_string_free()`.

use cqlib_tianyan::TianyanPlatform;
use std::os::raw::c_char;

use crate::backend::TianyanBackendC;
use crate::config::{TianyanConfigC, config_from_c};
use crate::error::{clear_last_error, cstr_to_str, set_last_error};
use crate::task::TianyanTaskC;

/// Opaque handle to the Tianyan platform client.
///
/// Obtain via `tianyan_platform_login()` or `tianyan_platform_from_credentials()`.
/// Must be freed with `tianyan_platform_free()`.
pub struct TianyanPlatformC(pub(crate) TianyanPlatform);

/// Authenticate with the Tianyan platform using an API key.
///
/// Credentials are saved to the default path (`~/.cqlib/tianyan/credentials.json`)
/// so subsequent calls can use `tianyan_platform_from_credentials()`.
///
/// Returns a heap-allocated `TianyanPlatform` on success, `NULL` on error.
/// The caller must free the returned pointer with `tianyan_platform_free()`.
#[unsafe(no_mangle)]
pub extern "C" fn tianyan_platform_login(api_key: *const c_char) -> *mut TianyanPlatformC {
    clear_last_error();
    let key = match cstr_to_str(api_key, "api_key") {
        Ok(s) => s,
        Err(e) => {
            set_last_error(e);
            return std::ptr::null_mut();
        }
    };
    match TianyanPlatform::login(key) {
        Ok(p) => Box::into_raw(Box::new(TianyanPlatformC(p))),
        Err(e) => {
            set_last_error(e);
            std::ptr::null_mut()
        }
    }
}

/// Like `tianyan_platform_login()` but with a custom configuration.
///
/// Pass `NULL` for `config` to use the default configuration.
#[unsafe(no_mangle)]
pub extern "C" fn tianyan_platform_login_with_config(
    api_key: *const c_char,
    config: *const TianyanConfigC,
) -> *mut TianyanPlatformC {
    clear_last_error();
    let key = match cstr_to_str(api_key, "api_key") {
        Ok(s) => s,
        Err(e) => {
            set_last_error(e);
            return std::ptr::null_mut();
        }
    };
    let cfg = match config_from_c(config) {
        Ok(c) => c,
        Err(e) => {
            set_last_error(e);
            return std::ptr::null_mut();
        }
    };
    match TianyanPlatform::login_with_config(key, cfg) {
        Ok(p) => Box::into_raw(Box::new(TianyanPlatformC(p))),
        Err(e) => {
            set_last_error(e);
            std::ptr::null_mut()
        }
    }
}

/// Load previously saved credentials from disk.
///
/// If the access token has expired, re-logs in automatically using the saved
/// API key (requires `auto_refresh = true`, the default).
///
/// Returns a heap-allocated `TianyanPlatform` on success, `NULL` on error.
/// The caller must free the returned pointer with `tianyan_platform_free()`.
#[unsafe(no_mangle)]
pub extern "C" fn tianyan_platform_from_credentials() -> *mut TianyanPlatformC {
    clear_last_error();
    match TianyanPlatform::from_credentials() {
        Ok(p) => Box::into_raw(Box::new(TianyanPlatformC(p))),
        Err(e) => {
            set_last_error(e);
            std::ptr::null_mut()
        }
    }
}

/// Like `tianyan_platform_from_credentials()` but with a custom configuration.
///
/// Pass `NULL` for `config` to use the default configuration.
#[unsafe(no_mangle)]
pub extern "C" fn tianyan_platform_from_credentials_with_config(
    config: *const TianyanConfigC,
) -> *mut TianyanPlatformC {
    clear_last_error();
    let cfg = match config_from_c(config) {
        Ok(c) => c,
        Err(e) => {
            set_last_error(e);
            return std::ptr::null_mut();
        }
    };
    match TianyanPlatform::from_credentials_with_config(cfg) {
        Ok(p) => Box::into_raw(Box::new(TianyanPlatformC(p))),
        Err(e) => {
            set_last_error(e);
            std::ptr::null_mut()
        }
    }
}

/// Free a `TianyanPlatform` returned by any `tianyan_platform_*` function.
///
/// Passing `NULL` is safe and does nothing.
#[unsafe(no_mangle)]
pub extern "C" fn tianyan_platform_free(platform: *mut TianyanPlatformC) {
    if !platform.is_null() {
        // SAFETY: `platform` was created via `Box::into_raw` in this crate.
        unsafe { drop(Box::from_raw(platform)) };
    }
}

/// Fetch all available quantum backends from the platform.
///
/// On success writes the backend count to `*out_len` and returns a
/// heap-allocated array of `TianyanBackend` pointers (length `*out_len`).
/// Each element **and** the array itself must be freed by calling
/// `tianyan_backend_list_free(backends, len)`.
///
/// Returns `NULL` on error; call `tianyan_last_error()` for details.
#[unsafe(no_mangle)]
pub extern "C" fn tianyan_platform_list_backends(
    platform: *const TianyanPlatformC,
    out_len: *mut usize,
) -> *mut *mut TianyanBackendC {
    clear_last_error();
    if platform.is_null() {
        set_last_error("platform must not be NULL");
        return std::ptr::null_mut();
    }
    if out_len.is_null() {
        set_last_error("out_len must not be NULL");
        return std::ptr::null_mut();
    }
    // SAFETY: `platform` was created by this crate.
    let p = unsafe { &*platform };
    match p.0.list_backends() {
        Ok(backends) => {
            let mut ptrs: Vec<*mut TianyanBackendC> = backends
                .into_iter()
                .map(|b| Box::into_raw(Box::new(TianyanBackendC(b))))
                .collect();
            // SAFETY: writing through the caller-provided pointer.
            unsafe { *out_len = ptrs.len() };
            let raw = ptrs.as_mut_ptr();
            std::mem::forget(ptrs);
            raw
        }
        Err(e) => {
            set_last_error(e);
            std::ptr::null_mut()
        }
    }
}

/// Return the backend with the given name (case-sensitive).
///
/// Returns a heap-allocated `TianyanBackend` on success, `NULL` if not found.
/// The caller must free the returned pointer with `tianyan_backend_free()`.
#[unsafe(no_mangle)]
pub extern "C" fn tianyan_platform_get_backend(
    platform: *const TianyanPlatformC,
    name: *const c_char,
) -> *mut TianyanBackendC {
    clear_last_error();
    if platform.is_null() {
        set_last_error("platform must not be NULL");
        return std::ptr::null_mut();
    }
    let name_str = match cstr_to_str(name, "name") {
        Ok(s) => s,
        Err(e) => {
            set_last_error(e);
            return std::ptr::null_mut();
        }
    };
    // SAFETY: `platform` was created by this crate.
    let p = unsafe { &*platform };
    match p.0.get_backend(name_str) {
        Ok(b) => Box::into_raw(Box::new(TianyanBackendC(b))),
        Err(e) => {
            set_last_error(e);
            std::ptr::null_mut()
        }
    }
}

/// Submit circuits directly without first fetching a backend handle.
///
/// Equivalent to `tianyan_platform_get_backend(platform, device_name)` followed
/// by `tianyan_backend_run(...)`, but skips the extra device-list network round-trip.
///
/// - `circuits`: array of `n_circuits` null-terminated QCIS strings.
/// - `shots`: number of measurement shots per circuit.
/// - `device_name`: target backend identifier (case-sensitive).
///
/// Returns a heap-allocated `TianyanTask` on success, `NULL` on error.
/// The caller must free the returned pointer with `tianyan_task_free()`.
#[unsafe(no_mangle)]
pub extern "C" fn tianyan_platform_submit(
    platform: *const TianyanPlatformC,
    circuits: *const *const c_char,
    n_circuits: usize,
    shots: usize,
    device_name: *const c_char,
) -> *mut TianyanTaskC {
    clear_last_error();
    if platform.is_null() {
        set_last_error("platform must not be NULL");
        return std::ptr::null_mut();
    }
    let device_str = match cstr_to_str(device_name, "device_name") {
        Ok(s) => s,
        Err(e) => {
            set_last_error(e);
            return std::ptr::null_mut();
        }
    };
    let circuit_inputs = match collect_circuits(circuits, n_circuits) {
        Ok(v) => v,
        Err(e) => {
            set_last_error(e);
            return std::ptr::null_mut();
        }
    };
    // SAFETY: `platform` was created by this crate.
    let p = unsafe { &*platform };
    match p.0.submit(circuit_inputs, shots, device_str) {
        Ok(t) => Box::into_raw(Box::new(TianyanTaskC(t))),
        Err(e) => {
            set_last_error(e);
            std::ptr::null_mut()
        }
    }
}

/// Collect a C array of C strings into a Vec<CircuitInput>.
pub(crate) fn collect_circuits(
    circuits: *const *const c_char,
    n: usize,
) -> Result<Vec<cqlib_tianyan::device::CircuitInput>, String> {
    if circuits.is_null() && n > 0 {
        return Err("circuits must not be NULL when n_circuits > 0".to_string());
    }
    (0..n)
        .map(|i| {
            // SAFETY: `circuits` is a valid C array of length `n`.
            let ptr = unsafe { *circuits.add(i) };
            let s = cstr_to_str(ptr, &format!("circuits[{i}]"))?;
            Ok(cqlib_tianyan::device::CircuitInput::from(s))
        })
        .collect()
}
