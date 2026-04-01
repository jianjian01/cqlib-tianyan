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
fn build_execution_result_basic() {
    let mut prob_map = HashMap::new();
    prob_map.insert("00".to_string(), 0.5);
    prob_map.insert("11".to_string(), 0.5);

    let er = build_er_from_prob_map("task-1", 1000, &prob_map).unwrap();
    assert_eq!(er.task_id(), "task-1");
    assert_eq!(er.shots(), 1000);
    assert_eq!(er.num_qubits(), 2);
    assert!(er.status().is_success());
    let total: usize = er.counts().values().sum();
    assert_eq!(total, 1000);
}

#[test]
fn build_execution_result_empty_map_returns_none() {
    let er = build_er_from_prob_map("t", 100, &HashMap::new());
    assert!(er.is_none());
}

#[test]
fn max_batch_size_is_fifty() {
    assert_eq!(MAX_BATCH_SIZE, 50);
}

#[test]
fn build_execution_result_single_qubit() {
    let mut prob_map = HashMap::new();
    prob_map.insert("0".to_string(), 0.7);
    prob_map.insert("1".to_string(), 0.3);

    let er = build_er_from_prob_map("task-single", 100, &prob_map).unwrap();
    assert_eq!(er.num_qubits(), 1);
    assert_eq!(er.shots(), 100);
    let total: usize = er.counts().values().sum();
    assert_eq!(total, 100);
}

#[test]
fn build_execution_result_rounds_counts_correctly() {
    let mut prob_map = HashMap::new();
    // 0.333 * 1000 = 333, 0.667 * 1000 = 667 → total = 1000
    prob_map.insert("0".to_string(), 0.333);
    prob_map.insert("1".to_string(), 0.667);

    let er = build_er_from_prob_map("round-test", 1000, &prob_map).unwrap();
    let total: usize = er.counts().values().sum();
    assert_eq!(total, 1000);
}

#[test]
fn build_execution_result_has_probabilities() {
    let mut prob_map = HashMap::new();
    prob_map.insert("00".to_string(), 0.5);
    prob_map.insert("11".to_string(), 0.5);

    let er = build_er_from_prob_map("task-prob", 1000, &prob_map).unwrap();
    let probs = er.probabilities();
    assert!(probs.is_some());
    let probs = probs.as_ref().unwrap();
    assert_eq!(probs.len(), 2);
}

#[test]
fn build_execution_result_three_qubit() {
    let mut prob_map = HashMap::new();
    prob_map.insert("000".to_string(), 0.5);
    prob_map.insert("111".to_string(), 0.5);

    let er = build_er_from_prob_map("task-3q", 200, &prob_map).unwrap();
    assert_eq!(er.num_qubits(), 3);
    let total: usize = er.counts().values().sum();
    assert_eq!(total, 200);
}

#[test]
fn build_execution_result_zero_probability_excluded() {
    let mut prob_map = HashMap::new();
    prob_map.insert("00".to_string(), 1.0);
    prob_map.insert("01".to_string(), 0.0);
    prob_map.insert("10".to_string(), 0.0);
    prob_map.insert("11".to_string(), 0.0);

    let er = build_er_from_prob_map("zero-test", 500, &prob_map).unwrap();
    // Only "00" should have a non-zero count
    let counts = er.counts();
    let total: usize = counts.values().sum();
    assert_eq!(total, 500);
    // Outcomes with 0 probability should have 0 or no count
    assert_eq!(counts.len(), 1);
}
