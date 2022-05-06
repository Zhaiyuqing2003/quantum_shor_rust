use std::f32::consts::PI;
use num::{complex::Complex32, One};

use quantum_shor_rust::{
    gates::{phase::*, unitary_modular::UnitaryModular, control_unitary_modular::ControlUnitaryModular, control_phase::ControlPhase},
    quantum_state::*, classical_shor::{self, quantum_shor}, gate_util::Gate
};

use classical_shor::classical_shor;


fn main() {
    println!("updated");
    // factoring 15.0 could be done in reasonable time
    // while other number, when it's greater than 32.0, are starting to take tremendous amount of time.
    let vec = quantum_shor(15.0);
    println!("{:?}", vec);

    let vec = classical_shor(33.0);
    println!("{:?}", vec);
}
