use std::f32::consts::PI;
use num::complex::Complex32;

use quantum_shor_rust::{
    gates::phase::*,
    quantum_state::*
};


fn main() {
    let mut state = QuantumState::from_length(2);
    // insert state |01> with prob 0.6 + 0.8i
    state.increment_state(1, Complex32::new(0.6, 0.8));
    // create a gate
    let mut gate = Phase::new(2, 1, PI);
    
}
