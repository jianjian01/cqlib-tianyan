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

use super::*;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::tempdir;

fn fresh_creds() -> Credentials {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    Credentials {
        api_key: "test-key".to_string(),
        access_token: "tok-abc".to_string(),
        token_obtained_at: now,
    }
}

#[test]
fn fresh_token_is_not_expired() {
    assert!(!fresh_creds().is_token_expired());
}

#[test]
fn old_token_is_expired() {
    let mut c = fresh_creds();
    c.token_obtained_at -= TOKEN_VALID_SECS + 1;
    assert!(c.is_token_expired());
}

#[test]
fn save_and_load_round_trip() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("creds.json");
    let creds = fresh_creds();
    save_credentials(&creds, &path).unwrap();
    let loaded = load_credentials(&path).unwrap();
    assert_eq!(loaded.api_key, creds.api_key);
    assert_eq!(loaded.access_token, creds.access_token);
}

#[test]
fn load_nonexistent_file_returns_error() {
    let result = load_credentials(std::path::Path::new("/nonexistent/path/creds.json"));
    assert!(matches!(result, Err(TianyanError::Auth(_))));
}

#[test]
fn empty_api_key_creates_valid_credentials_struct() {
    let c = Credentials {
        api_key: String::new(),
        access_token: "tok".to_string(),
        token_obtained_at: 0,
    };
    // An empty key is structurally valid (server will reject it)
    assert!(c.is_token_expired()); // epoch-0 is always expired
}

#[test]
fn save_creates_parent_directories() {
    let dir = tempdir().unwrap();
    let nested = dir.path().join("a").join("b").join("creds.json");
    let creds = fresh_creds();
    save_credentials(&creds, &nested).unwrap();
    assert!(nested.exists());
}

#[test]
fn credentials_serialisation_is_stable() {
    let creds = Credentials {
        api_key: "k".to_string(),
        access_token: "t".to_string(),
        token_obtained_at: 1234567890,
    };
    let json = serde_json::to_string(&creds).unwrap();
    let back: Credentials = serde_json::from_str(&json).unwrap();
    assert_eq!(back.api_key, "k");
    assert_eq!(back.access_token, "t");
    assert_eq!(back.token_obtained_at, 1234567890);
}
