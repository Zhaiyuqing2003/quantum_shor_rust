use std::f32::consts::PI;
use num::{complex::Complex32, One};

use quantum_shor_rust::{
    gates::{phase::*, unitary_modular::UnitaryModular, control_unitary_modular::ControlUnitaryModular, control_phase::ControlPhase},
    quantum_state::*, classical_shor::{self, quantum_shor}, gate_util::Gate
};

use classical_shor::classical_shor;


fn main() {
    // let mut state = QuantumState::from_length(2);
    // // insert state |01> with prob 0.6 + 0.8i
    // state.increment_state(1, Complex32::new(0.6, 0.8));
    // // create a gate
    // let mut gate = Phase::new(2, 1, PI);
    println!("updated");
    let vec = quantum_shor(15.0);

    // let mut state = QuantumState::from_length(2);
    // state.increment_state(1, Complex32::one());
    // state.increment_state(1, -Complex32::one());

    println!("{:?}", vec);
    // let a = 4.0;
    // println!("{}", a % 3.0 == 1.0)
}
