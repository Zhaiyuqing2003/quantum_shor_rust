use num::complex::Complex32;
use crate::gate_util::*;
use crate::quantum_state::*;

pub struct Rz {
    n : usize,
    wire : usize,
    angle : f32,
}

impl Rz {
    pub fn new(n : usize, wire : usize, angle : f32) -> Rz {
        panic_on_out_of_bounds(n, wire);
        Rz {
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

impl Gate for Rz {
    fn get(&self) -> GateFunction {
        let n = self.n;
        let wire = self.wire;
        let angle = self.angle;

        let rz = move |state : &QuantumState| {
            panic_on_length_mismatch(n, state.get_bit_length());

            let mut new_state = QuantumState::from_length(n);

            for (state, value) in state.iter() {
                new_state.increment_state(
                    *state,
                    value * Complex32::from_polar(
                        1.0, 
                        if state & (1 << (n - 1 - wire)) == 1 {
                            -angle / 2.0
                        } else {
                            angle / 2.0
                        }
                    )
                );
            }
            
            new_state
        };
        Box::new(rz)
    }
}

impl Reversible for Rz {
    fn reverse(&self) -> Box<dyn Gate> {
        Box::new(Rz {
            n : self.n,
            wire : self.wire,
            angle : -self.angle,
        })
    }
}