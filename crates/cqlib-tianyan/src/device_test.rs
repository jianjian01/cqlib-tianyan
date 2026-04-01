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
fn device_status_mapping() {
    assert_eq!(DeviceStatus::from_code(0), DeviceStatus::Running);
    assert_eq!(DeviceStatus::from_code(1), DeviceStatus::Calibration);
    assert_eq!(DeviceStatus::from_code(2), DeviceStatus::UnderMaintenance);
    assert_eq!(DeviceStatus::from_code(3), DeviceStatus::OffLine);
    assert!(matches!(
        DeviceStatus::from_code(99),
        DeviceStatus::Unknown(99)
    ));
}

#[test]
fn device_toll_mapping() {
    assert_eq!(DeviceToll::from_code(1), DeviceToll::Free);
    assert_eq!(DeviceToll::from_code(2), DeviceToll::Paid);
    assert!(matches!(DeviceToll::from_code(0), DeviceToll::Unknown(0)));
}

#[test]
fn circuit_input_from_str() {
    let input: CircuitInput = "H Q0\nM Q0".into();
    let qcis = input.into_qcis().unwrap();
    assert_eq!(qcis, "H Q0\nM Q0");
}

#[test]
fn circuit_input_from_string() {
    let input: CircuitInput = "H Q0".to_string().into();
    let qcis = input.into_qcis().unwrap();
    assert_eq!(qcis, "H Q0");
}

#[test]
fn device_status_all_known_codes() {
    // Exhaustively check that known codes 0-3 map correctly
    let codes_and_expected = [
        (0, DeviceStatus::Running),
        (1, DeviceStatus::Calibration),
        (2, DeviceStatus::UnderMaintenance),
        (3, DeviceStatus::OffLine),
    ];
    for (code, expected) in codes_and_expected {
        assert_eq!(DeviceStatus::from_code(code), expected, "code={}", code);
    }
}

#[test]
fn device_status_negative_code() {
    let status = DeviceStatus::from_code(-1);
    assert!(matches!(status, DeviceStatus::Unknown(-1)));
}

#[test]
fn device_toll_negative_code() {
    let toll = DeviceToll::from_code(-1);
    assert!(matches!(toll, DeviceToll::Unknown(-1)));
}

#[test]
fn circuit_input_multiline_qcis() {
    let qcis = "H Q0\nCZ Q0 Q1\nM Q0\nM Q1";
    let input: CircuitInput = qcis.into();
    assert_eq!(input.into_qcis().unwrap(), qcis);
}

#[test]
fn circuit_input_empty_string() {
    let input: CircuitInput = "".into();
    assert_eq!(input.into_qcis().unwrap(), "");
}
