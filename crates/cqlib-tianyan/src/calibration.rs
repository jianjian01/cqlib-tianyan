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

//! Readout error mitigation via inverse confusion-matrix calibration.
//!
//! When a quantum computer measures qubits, readout errors can flip the
//! observed bit values.  Given the per-qubit readout fidelities `F00` (= P(0|0))
//! and `F11` (= P(1|1)), this module constructs the inverse of the
//! *tensor-product confusion matrix* and applies it to measured probability
//! distributions to produce corrected probabilities.
//!
//! # Algorithm
//!
//! For a single qubit *k* with fidelities F00_k and F11_k, the 2×2 confusion
//! matrix is:
//!
//! ```text
//!     CM_k = | F00_k     1-F11_k |
//!            | 1-F00_k   F11_k   |
//! ```
//!
//! Its inverse (which exists when F00_k + F11_k ≠ 1) is:
//!
//! ```text
//!     CM_k⁻¹ = 1/(F00_k + F11_k - 1) * | F11_k      F11_k - 1 |
//!                                        | F00_k - 1  F00_k     |
//! ```
//!
//! The full inverse confusion matrix is the Kronecker (tensor) product
//! `CM⁻¹ = CM_n⁻¹ ⊗ CM_(n-1)⁻¹ ⊗ … ⊗ CM_0⁻¹`.  We apply `P_corrected = CM⁻¹ × P_measured`
//! and then clip negative values to 0, cap values > 1 to 1, and renormalise.
//!
//! This is the same algorithm used in the Python `laboratory_utils.probability_calibration`.

use std::collections::HashMap;

/// Apply readout error mitigation to a probability distribution.
///
/// # Arguments
/// * `prob_map` — measured probability distribution keyed by bit-string (e.g. `"01"` → 0.45).
/// * `f00` — per-qubit P(measure 0 | prepared 0) fidelities, ordered from qubit 0 … n−1.
/// * `f11` — per-qubit P(measure 1 | prepared 1) fidelities, same order.
///
/// # Returns
/// A corrected probability map with the same keys.
///
/// # Panics
/// Panics if `f00` and `f11` have different lengths, or if their length does not
/// match the bit-string width.
pub fn calibrate_probabilities(
    prob_map: &HashMap<String, f64>,
    f00: &[f64],
    f11: &[f64],
) -> HashMap<String, f64> {
    assert_eq!(
        f00.len(),
        f11.len(),
        "f00 and f11 must have the same length"
    );

    if prob_map.is_empty() {
        return HashMap::new();
    }

    let n = f00.len();

    // Build the inverse confusion matrix via Kronecker products.
    // Start with a 1×1 identity [1.0] and iteratively Kronecker with each qubit's inverse CM.
    let mut inv_cm = vec![1.0_f64]; // 2^0 × 2^0 = 1×1

    for k in 0..n {
        let det = f00[k] + f11[k] - 1.0;
        if det.abs() < 1e-12 {
            // Degenerate case: F00+F11≈1 means the measurement is essentially random.
            // Fall through with an identity matrix for this qubit.
            inv_cm = kronecker_2x2(&inv_cm, &[1.0, 0.0, 0.0, 1.0]);
        } else {
            let inv_det = 1.0 / det;
            // CM_k⁻¹ stored row-major: [a00, a01, a10, a11]
            let local_inv = [
                f11[k] * inv_det,
                (f11[k] - 1.0) * inv_det,
                (f00[k] - 1.0) * inv_det,
                f00[k] * inv_det,
            ];
            inv_cm = kronecker_2x2(&inv_cm, &local_inv);
        }
    }

    // Build the probability vector ordered by integer bit-string value.
    let dim = 1 << n;
    let mut p_vec = vec![0.0; dim];
    for (bitstr, &prob) in prob_map {
        if let Ok(idx) = usize::from_str_radix(bitstr, 2) {
            if idx < dim {
                p_vec[idx] = prob;
            }
        }
    }

    // Matrix-vector multiply: p_corrected = inv_cm × p_vec
    let mut corrected = vec![0.0; dim];
    for i in 0..dim {
        let mut sum = 0.0;
        for j in 0..dim {
            sum += inv_cm[i * dim + j] * p_vec[j];
        }
        corrected[i] = sum;
    }

    // Post-process: clip to [0, 1] and renormalise.
    for v in corrected.iter_mut() {
        *v = v.clamp(0.0, 1.0);
    }
    let total: f64 = corrected.iter().sum();
    if total > 1e-15 {
        for v in corrected.iter_mut() {
            *v /= total;
        }
    }

    // Convert back to bit-string map.
    let mut result = HashMap::new();
    for (i, &prob) in corrected.iter().enumerate() {
        if prob > 0.0 {
            let bitstr = format!("{:0>width$b}", i, width = n);
            result.insert(bitstr, prob);
        }
    }
    result
}

/// Kronecker product of matrix `a` (size m×m, stored row-major) with a 2×2 matrix `b`.
///
/// Returns a (2m)×(2m) matrix stored row-major.
fn kronecker_2x2(a: &[f64], b: &[f64]) -> Vec<f64> {
    debug_assert_eq!(b.len(), 4);
    let m = (a.len() as f64).sqrt() as usize;
    debug_assert_eq!(m * m, a.len());

    let new_m = m * 2;
    let mut result = vec![0.0; new_m * new_m];

    for i in 0..m {
        for j in 0..m {
            let a_val = a[i * m + j];
            // Place the 2×2 block a_val * b at position (2i, 2j)
            for bi in 0..2 {
                for bj in 0..2 {
                    result[(2 * i + bi) * new_m + (2 * j + bj)] = a_val * b[bi * 2 + bj];
                }
            }
        }
    }
    result
}

#[cfg(test)]
#[path = "calibration_test.rs"]
mod tests;
