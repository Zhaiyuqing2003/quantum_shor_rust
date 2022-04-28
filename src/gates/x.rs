use crate::gate_util::*;
use crate::quantum_state::*;
pub struct X {
    n : usize,
    wire : usize,
}

impl X {
    pub fn new(n : usize, wire : usize) -> X {
        panic_on_out_of_bounds(n, wire);
        X {
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

impl Gate for X {
    fn get(&self) -> GateFunction {
        let n = self.n;
        let wire = self.wire;

        let x = move |state : &QuantumState| {
            panic_on_length_mismatch(n, state.get_bit_length());

            let mut new_state = QuantumState::from_length(n);

            for (state, value) in state.iter() {
                new_state.increment_state(
                    state ^ (1 << (n - 1 - wire)),
                    value.clone()
                );
            }
            
            new_state
        };
        Box::new(x)
    }
}

impl Reversible for X {
    fn reverse(&self) -> Self {
        X {
            n : self.n,
            wire : self.wire,
        }
    }
}