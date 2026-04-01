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

#[test]
fn default_config_has_correct_domain() {
    let cfg = TianyanConfig::default();
    assert_eq!(cfg.domain, "qc.zdxlz.com");
    assert_eq!(cfg.scheme, "https");
    assert_eq!(cfg.base_url(), "https://qc.zdxlz.com");
}

#[test]
fn custom_config_overrides_domain() {
    let cfg = TianyanConfig::custom("test.example.com");
    assert_eq!(cfg.domain, "test.example.com");
    assert_eq!(cfg.base_url(), "https://test.example.com");
}

#[test]
fn credentials_path_is_non_empty() {
    let cfg = TianyanConfig::default();
    assert!(!cfg.credentials_path.as_os_str().is_empty());
    assert_eq!(
        cfg.credentials_path.file_name().and_then(|n| n.to_str()),
        Some("credentials.json")
    );
}

#[test]
fn api_path_constants_start_with_slash() {
    assert!(DEVICE_LIST_PATH.starts_with('/'));
    assert!(SUBMIT_PATH.starts_with('/'));
    assert!(QUERY_RESULT_PATH.starts_with('/'));
    assert!(LOGIN_PATH.starts_with('/'));
    assert!(DOWNLOAD_CONFIG_PATH.starts_with('/'));
}

#[test]
fn credentials_path_contains_tianyan_dir() {
    let cfg = TianyanConfig::default();
    let path_str = cfg.credentials_path.to_string_lossy();
    assert!(
        path_str.contains("tianyan"),
        "credentials path should contain 'tianyan': {}",
        path_str
    );
}
