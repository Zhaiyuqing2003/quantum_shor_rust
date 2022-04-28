use num::complex::Complex32;
use crate::gate_util::*;
use crate::quantum_state::*;

#[derive(Debug, Clone)]
pub struct Phase {
    n : usize,
    wire : usize,
    angle : f32,
}

impl Phase {
    pub fn new(n : usize, wire : usize, angle : f32) -> Phase {
        panic_on_out_of_bounds(n, wire);
        Phase {
            n,
            wire,
            angle,
        }
    }
    pub fn n(&self) -> usize {
        self.n
    }
    pub fn wire(&self) -> usize {
        self.wire
    }
    pub fn angle(&self) -> f32 {
        self.angle
    }
}

impl Gate for Phase {
    fn get(&self) -> GateFunction {
        let n = self.n;
        let wire = self.wire;
        let angle = self.angle;

        let phase = move |state : &QuantumState| {
            panic_on_length_mismatch(n, state.get_bit_length());

            let mut new_state = QuantumState::from_length(n);

            for (state, value) in state.iter() {
                new_state.increment_state(
                    *state,
                    if state & (n - 1 - wire) == 0 {
                        value.clone()
                    } else {
                        value * Complex32::from_polar(1.0, angle)
                    }
                );
            }
            
            new_state
        };
        Box::new(phase)
    }
}

impl Reversible for Phase {
    fn reverse(&self) -> Self {
        Phase {
            n : self.n,
            wire : self.wire,
            angle : -self.angle,
        }
    }
}
