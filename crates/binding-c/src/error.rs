// This code is part of Cqlib.
//
// (C) Copyright China Telecom Quantum Group 2026
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.

//! Thread-local last-error storage and string-free helpers.
//!
//! All C API functions that can fail return `NULL` (for pointer-returning functions)
//! or `-1` / `false` on error. Call `tianyan_last_error()` immediately after to
//! retrieve a human-readable description. The returned string must be freed with
//! `tianyan_string_free()`.

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

thread_local! {
    static LAST_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Store an error message in the thread-local slot.
pub(crate) fn set_last_error(e: impl std::fmt::Display) {
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = Some(e.to_string());
    });
}

/// Clear any previously stored error.
pub(crate) fn clear_last_error() {
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = None;
    });
}

/// Retrieve the last error message set on this thread.
///
/// Returns `NULL` if no error has occurred since the last successful call.
/// The returned string is heap-allocated and **must** be freed with
/// `tianyan_string_free()`.
#[unsafe(no_mangle)]
pub extern "C" fn tianyan_last_error() -> *mut c_char {
    LAST_ERROR.with(|slot| {
        match slot.borrow().as_deref() {
            Some(msg) => CString::new(msg).map(|s| s.into_raw()).unwrap_or(std::ptr::null_mut()),
            None => std::ptr::null_mut(),
        }
    })
}

/// Clear the last error stored for this thread.
#[unsafe(no_mangle)]
pub extern "C" fn tianyan_error_clear() {
    clear_last_error();
}

/// Free a `char*` string that was returned by any `cqlib_tianyan` API function.
///
/// Passing `NULL` is safe and does nothing.
#[unsafe(no_mangle)]
pub extern "C" fn tianyan_string_free(s: *mut c_char) {
    if !s.is_null() {
        // SAFETY: `s` was created via `CString::into_raw()` in this crate.
        unsafe { drop(CString::from_raw(s)) };
    }
}

/// Helper: convert a nullable `*const c_char` to `&str`, returning an error on failure.
pub(crate) fn cstr_to_str<'a>(ptr: *const c_char, name: &str) -> Result<&'a str, String> {
    if ptr.is_null() {
        return Err(format!("{name} must not be NULL"));
    }
    // SAFETY: caller guarantees `ptr` is a valid, null-terminated C string.
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|_| format!("{name} is not valid UTF-8"))
}

/// Helper: convert a `&str` to a heap-allocated `*mut c_char` (caller must free).
pub(crate) fn str_to_cstring(s: &str) -> *mut c_char {
    CString::new(s).map(|cs| cs.into_raw()).unwrap_or(std::ptr::null_mut())
}
