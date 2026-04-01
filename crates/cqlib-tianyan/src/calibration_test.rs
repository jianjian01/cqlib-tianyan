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
fn identity_calibration_no_error() {
    let mut prob = HashMap::new();
    prob.insert("00".to_string(), 0.5);
    prob.insert("11".to_string(), 0.5);

    let corrected = calibrate_probabilities(&prob, &[1.0, 1.0], &[1.0, 1.0]);

    assert!((corrected["00"] - 0.5).abs() < 1e-10);
    assert!((corrected["11"] - 0.5).abs() < 1e-10);
}

#[test]
fn single_qubit_calibration() {
    let mut prob = HashMap::new();
    prob.insert("0".to_string(), 0.7);
    prob.insert("1".to_string(), 0.3);

    let corrected = calibrate_probabilities(&prob, &[0.95], &[0.90]);

    let total: f64 = corrected.values().sum();
    assert!((total - 1.0).abs() < 1e-10);
    assert!((corrected["0"] - 0.7059).abs() < 0.01);
    assert!((corrected["1"] - 0.2941).abs() < 0.01);
}

#[test]
fn two_qubit_calibration_preserves_normalization() {
    let mut prob = HashMap::new();
    prob.insert("00".to_string(), 0.25);
    prob.insert("01".to_string(), 0.25);
    prob.insert("10".to_string(), 0.25);
    prob.insert("11".to_string(), 0.25);

    let corrected = calibrate_probabilities(&prob, &[0.95, 0.90], &[0.85, 0.92]);

    let total: f64 = corrected.values().sum();
    assert!((total - 1.0).abs() < 1e-10, "total = {}", total);
}

#[test]
fn empty_map_returns_empty() {
    let prob = HashMap::new();
    let corrected = calibrate_probabilities(&prob, &[0.95], &[0.90]);
    assert!(corrected.is_empty());
}

#[test]
fn negative_clipping_works() {
    let mut prob = HashMap::new();
    prob.insert("0".to_string(), 0.99);
    prob.insert("1".to_string(), 0.01);

    let corrected = calibrate_probabilities(&prob, &[0.80], &[0.80]);

    for &v in corrected.values() {
        assert!(v >= 0.0);
    }
    let total: f64 = corrected.values().sum();
    assert!((total - 1.0).abs() < 1e-10);
}

#[test]
fn kronecker_2x2_basic() {
    let a = vec![1.0];
    let b = [2.0, 3.0, 4.0, 5.0];
    let result = kronecker_2x2(&a, &b);
    assert_eq!(result, vec![2.0, 3.0, 4.0, 5.0]);
}

#[test]
fn kronecker_2x2_two_step() {
    let a = vec![1.0];
    let m1 = [1.0, 0.0, 0.0, 1.0];
    let step1 = kronecker_2x2(&a, &m1);
    assert_eq!(step1.len(), 4);

    let m2 = [2.0, 0.0, 0.0, 2.0];
    let step2 = kronecker_2x2(&step1, &m2);
    assert_eq!(step2.len(), 16);
    assert_eq!(step2[0], 2.0);
    assert_eq!(step2[5], 2.0);
    assert_eq!(step2[10], 2.0);
    assert_eq!(step2[15], 2.0);
    assert_eq!(step2[1], 0.0);
}

#[test]
fn three_qubit_calibration_normalization() {
    let mut prob = HashMap::new();
    prob.insert("000".to_string(), 0.125);
    prob.insert("001".to_string(), 0.125);
    prob.insert("010".to_string(), 0.125);
    prob.insert("011".to_string(), 0.125);
    prob.insert("100".to_string(), 0.125);
    prob.insert("101".to_string(), 0.125);
    prob.insert("110".to_string(), 0.125);
    prob.insert("111".to_string(), 0.125);

    let corrected = calibrate_probabilities(&prob, &[0.95, 0.90, 0.85], &[0.85, 0.92, 0.88]);

    let total: f64 = corrected.values().sum();
    assert!((total - 1.0).abs() < 1e-10, "3-qubit total = {}", total);
    for &v in corrected.values() {
        assert!(v >= 0.0);
    }
}

#[test]
fn all_zeros_measured_stays_normalized() {
    let mut prob = HashMap::new();
    prob.insert("00".to_string(), 1.0);

    let corrected = calibrate_probabilities(&prob, &[0.9, 0.9], &[0.9, 0.9]);

    let total: f64 = corrected.values().sum();
    assert!((total - 1.0).abs() < 1e-10);
}

#[test]
fn perfect_fidelity_preserves_input() {
    let mut prob = HashMap::new();
    prob.insert("01".to_string(), 0.3);
    prob.insert("10".to_string(), 0.7);

    let corrected = calibrate_probabilities(&prob, &[1.0, 1.0], &[1.0, 1.0]);

    assert!((corrected["01"] - 0.3).abs() < 1e-10);
    assert!((corrected["10"] - 0.7).abs() < 1e-10);
}
