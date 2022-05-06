use std::f32::consts::SQRT_2;
use crate::gate_util::*;
use crate::quantum_state::*;

#[derive(Debug, Clone)]
pub struct Hadamard {
    n : usize,
    wire : usize,
}

impl Hadamard {
    pub fn new(n : usize, wire : usize) -> Hadamard {
        panic_on_out_of_bounds(n, wire);
        Hadamard {
            n,
            wire,
        }
    }
    pub fn n(&self) -> usize {
        self.n
    }
    pub fn wire(&self) -> usize {
        self.wire
    }
}

impl Gate for Hadamard {
    fn get(&self) -> GateFunction {
        let n = self.n;
        let wire = self.wire;
        let signature = 1 << (n - 1 - wire);

        let hadamard = move |state : &QuantumState| {
            panic_on_length_mismatch(n, state.get_bit_length());

            let mut new_state = QuantumState::from_length(n);

            for (state, value) in state.iter() {
                new_state.increment_state(
                    state & !signature,
                    value / SQRT_2
                );
                new_state.increment_state(
                    state | signature,
                    if state & signature == 0 {
                        value / SQRT_2
                    } else {
                        value / -SQRT_2
                    }
                );
            }
            
            new_state
        };
        Box::new(hadamard)
    }
}

impl Reversible for Hadamard {
    fn reverse(&self) -> Box<dyn Gate> {
        Box::new(self.clone())
    }
}
