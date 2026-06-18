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
use cqlib_core::circuit::{Instruction, StandardGate};

fn standard_native_gates(device: &cqlib_core::device::Device) -> Vec<StandardGate> {
    device
        .native_gates()
        .iter()
        .filter_map(|instruction| match instruction {
            Instruction::Standard(gate) => Some(*gate),
            _ => None,
        })
        .collect()
}

fn sample_config_json() -> serde_json::Value {
    serde_json::json!({
        "calibrationTime": "2025-08-07 15:16:22",
        "computerId": "tianyan24",
        "disabledQubits": "Q2,Q10",
        "disabledCouplers": "G4,G8",
        "overview": {
            "name": "",
            "type": "",
            "qubits": ["Q0", "Q1", "Q2", "Q3"],
            "couplers": ["G0", "G1", "G2"],
            "coupler_map": {
                "G0": ["Q1", "Q0"],
                "G1": ["Q2", "Q1"],
                "G2": ["Q3", "Q2"]
            },
            "qubits_length": 4,
            "couplers_length": 3,
            "readouts_length": 1,
            "T1": 44.65,
            "T2": 14.12,
            "cz_error": 3.2,
            "1q_gate_error": 0.22,
            "readout_error": 7.84
        },
        "qubit": {
            "frequency": {
                "f01": {
                    "param_list": [4.5657, 5.1297, 5.175, 4.6992],
                    "qubit_used": ["Q0", "Q1", "Q2", "Q3"],
                    "unit": "GHz"
                }
            },
            "relatime": {
                "T1": {
                    "param_list": [26.5, 39.4, 43.3, 57.2],
                    "qubit_used": ["Q0", "Q1", "Q2", "Q3"],
                    "unit": "us"
                },
                "T2": {
                    "param_list": [8.6, 4.5, 7.6, 21.7],
                    "qubit_used": ["Q0", "Q1", "Q2", "Q3"],
                    "unit": "us"
                }
            },
            "singleQubit": {
                "gate error": {
                    "param_list": [0.15, 0.48, 0.23, 0.10],
                    "qubit_used": ["Q0", "Q1", "Q2", "Q3"],
                    "unit": "%"
                }
            }
        },
        "readout": {
            "readoutArray": {
                "Readout Error": {
                    "param_list": [11.48, 4.79, 6.75, 7.05],
                    "qubit_used": ["Q0", "Q1", "Q2", "Q3"],
                    "unit": "%"
                },
                "|0> readout fidelity": {
                    "param_list": [0.9525, 0.9689, 0.965, 0.94],
                    "qubit_used": ["Q0", "Q1", "Q2", "Q3"],
                    "unit": ""
                },
                "|1> readout fidelity": {
                    "param_list": [0.818, 0.9352, 0.8999, 0.9191],
                    "qubit_used": ["Q0", "Q1", "Q2", "Q3"],
                    "unit": ""
                }
            }
        },
        "twoQubitGate": {
            "czGate": {
                "gate error": {
                    "param_list": [2.91, 1.59],
                    "qubit_used": ["G0", "G2"],
                    "unit": "%"
                }
            }
        },
        "status": 0
    })
}

#[test]
fn parse_device_config_basic() {
    let json = sample_config_json();
    let device = parse_device_config("tianyan24", &json).unwrap();

    assert_eq!(device.name(), "tianyan24");
    // Q2 is disabled → available qubits are Q0, Q1, Q3 (3 total).
    assert_eq!(device.topology().num_qubits(), 3);

    // Disabled qubits
    let invalid: HashSet<PhysicalQubit> = device.invalid_qubits().collect();
    assert!(invalid.contains(&PhysicalQubit::new(2)));
    assert!(!invalid.contains(&PhysicalQubit::new(10)));

    assert_eq!(device.default_single_qubit_error(), Some(0.0022));
    assert_eq!(device.default_two_qubit_error(), Some(0.032));

    // Per-qubit properties
    let q0_props = device.qubit_properties(PhysicalQubit::new(0)).unwrap();
    assert!((q0_props.readout_error() - 0.1148).abs() < 0.001);
    assert!((q0_props.frequency().unwrap() - 4.5657).abs() < 0.001);
    assert!((q0_props.t1().unwrap() - 26.5).abs() < 0.1);
    assert!((q0_props.t2().unwrap() - 8.6).abs() < 0.1);

    // Readout fidelities
    assert!((q0_props.prob_meas1_prep0().unwrap() - 0.0475).abs() < 0.001);
    assert!((q0_props.prob_meas0_prep1().unwrap() - 0.182).abs() < 0.001);
}

#[test]
fn native_gates_are_set_from_known_tianyan_machine() {
    let json = sample_config_json();
    let device = parse_device_config("tianyan176", &json).unwrap();
    assert_eq!(
        standard_native_gates(&device),
        vec![
            StandardGate::RZ,
            StandardGate::X2P,
            StandardGate::X2M,
            StandardGate::Y2P,
            StandardGate::Y2M,
            StandardGate::XY2P,
            StandardGate::XY2M,
            StandardGate::CZ,
        ]
    );
}

#[test]
fn tianyan504_native_gates_include_fsim() {
    let json = sample_config_json();
    let device = parse_device_config("tianyan504", &json).unwrap();
    assert!(standard_native_gates(&device).contains(&StandardGate::FSIM));
}

#[test]
fn parse_qubit_names() {
    assert_eq!(parse_qubit("Q0"), Some(PhysicalQubit::new(0)));
    assert_eq!(parse_qubit("Q23"), Some(PhysicalQubit::new(23)));
    assert_eq!(parse_qubit("G0"), None);
    assert_eq!(parse_qubit(""), None);
    assert_eq!(parse_qubit("Qx"), None);
}

#[test]
fn parse_calibration_time_valid() {
    let dt = parse_calibration_time("2025-08-07 15:16:22");
    assert!(dt.is_some());
    let dt = dt.unwrap();
    assert_eq!(dt.year(), 2025);
    assert_eq!(dt.month() as u8, 8);
    assert_eq!(dt.day(), 7);
}

#[test]
fn extract_readout_calibration_data() {
    let json = sample_config_json();
    let cal = extract_readout_calibration(&json).unwrap();
    assert_eq!(cal.qubit_names.len(), 4);
    assert!((cal.f00[0] - 0.9525).abs() < 0.001);
    assert!((cal.f11[0] - 0.818).abs() < 0.001);
}

#[test]
fn topology_connectivity() {
    let json = sample_config_json();
    let device = parse_device_config("test", &json).unwrap();
    let topo = device.topology();

    // G0 (Q1–Q0) is enabled and both endpoints are available.
    assert!(topo.supports_directed_coupling(PhysicalQubit::new(1), PhysicalQubit::new(0)));

    // G1 (Q2–Q1) and G2 (Q3–Q2) both involve Q2 which is disabled,
    // so those edges are excluded from the topology.
    // Q2 itself is not a topology node, so Q3–Q2 is not connected.
    assert!(!topo.supports_coupling_either_direction(PhysicalQubit::new(3), PhysicalQubit::new(2)));
    // Q3 is an isolated node (all its edges involved the disabled Q2).
    assert!(!topo.supports_coupling_either_direction(PhysicalQubit::new(3), PhysicalQubit::new(1)));
}

#[test]
fn edge_properties_cz_gate() {
    let json = sample_config_json();
    let device = parse_device_config("test", &json).unwrap();

    let edge = device.edge_properties(PhysicalQubit::new(1), PhysicalQubit::new(0));
    assert!(edge.is_some());
    let edge = edge.unwrap();
    let instructions = edge.native_instructions();
    assert_eq!(instructions.len(), 1);
    assert!((instructions[0].error_rate() - 0.0291).abs() < 0.001);
}

#[test]
fn parse_calibration_time_invalid() {
    assert!(parse_calibration_time("not-a-date").is_none());
    assert!(parse_calibration_time("").is_none());
}

#[test]
fn config_missing_overview_returns_error() {
    let json = serde_json::json!({
        "computerId": "test",
        "qubit": {},
        "status": 0
    });
    let result = parse_device_config("test", &json);
    assert!(result.is_err());
}

#[test]
fn config_empty_coupler_map() {
    let json = serde_json::json!({
        "overview": {
            "name": "",
            "type": "",
            "qubits": ["Q0", "Q1"],
            "couplers": [],
            "coupler_map": {},
            "qubits_length": 2,
            "couplers_length": 0,
            "readouts_length": 0,
            "T1": 10.0,
            "T2": 5.0,
            "cz_error": 1.0,
            "1q_gate_error": 0.1,
            "readout_error": 5.0
        },
        "status": 0
    });
    let device = parse_device_config("test", &json).unwrap();
    assert_eq!(device.topology().num_qubits(), 2);
    assert_eq!(device.topology().num_couplings(), 0);
}

#[test]
fn extract_readout_calibration_missing_fidelities() {
    let json = serde_json::json!({
        "overview": {
            "qubits": ["Q0"],
            "couplers": [],
            "coupler_map": {}
        },
        "readout": {
            "readoutArray": {
                "Readout Error": {
                    "param_list": [5.0],
                    "qubit_used": ["Q0"],
                    "unit": "%"
                }
            }
        }
    });
    // Missing both |0> and |1> fidelity arrays
    let cal = extract_readout_calibration(&json);
    assert!(cal.is_none());
}

#[test]
fn parse_device_config_from_string_encoded_json() {
    // Simulate what the API actually returns: data field is a JSON *string*
    let json_obj = sample_config_json();
    let json_str = serde_json::to_string(&json_obj).unwrap();
    // Wrap as a string Value (double-encoded)
    let string_value = serde_json::Value::String(json_str);

    // parse_device_config works on an already-decoded Value, but
    // download_device_config_full handles the string decoding. Test via
    // the decode logic directly:
    let decoded: serde_json::Value = if let Some(s) = string_value.as_str() {
        serde_json::from_str(s).unwrap()
    } else {
        string_value
    };
    let device = parse_device_config("test-encoded", &decoded).unwrap();
    assert_eq!(device.name(), "test-encoded");
    // Q2 is disabled → available qubits are Q0, Q1, Q3 (3 total).
    assert_eq!(device.topology().num_qubits(), 3);
}

#[test]
fn all_qubit_properties_present() {
    let json = sample_config_json();
    let device = parse_device_config("test", &json).unwrap();

    // Q3 should also have properties
    let q3_props = device.qubit_properties(PhysicalQubit::new(3)).unwrap();
    assert!((q3_props.frequency().unwrap() - 4.6992).abs() < 0.001);
    assert!((q3_props.t1().unwrap() - 57.2).abs() < 0.1);
    assert!((q3_props.t2().unwrap() - 21.7).abs() < 0.1);
    assert!((q3_props.readout_error() - 0.0705).abs() < 0.001);
}
