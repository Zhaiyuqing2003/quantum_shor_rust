use num::complex::Complex32;
use crate::gate_util::*;
use crate::quantum_state::*;

pub struct ControlRz {
    n : usize,
    wire : usize,
    control : usize,
    angle : f32,
}


impl ControlRz {
    pub fn new(n : usize, wire : usize, control : usize, angle : f32) -> ControlRz {
        panic_on_out_of_bounds(n, wire);
        panic_on_out_of_bounds(n, control);
        panic_if_equal(wire, control);
        ControlRz {
            n,
            wire,
            control,
            angle,
        }
    }
    pub fn n(&self) -> usize {
        self.n
    }
    pub fn wire(&self) -> usize {
        self.wire
    }
    pub fn control(&self) -> usize {
        self.control
    }
    pub fn angle(&self) -> f32 {
        self.angle
    }
}


impl Gate for ControlRz {
    fn get(&self) -> GateFunction {
        let n = self.n;
        let wire = self.wire;
        let control = self.control;
        let angle = self.angle;

        let control_rz = move |state : &QuantumState| {
            panic_on_length_mismatch(n, state.get_bit_length());

            let mut new_state = QuantumState::from_length(n);

            for (state, value) in state.iter() {
                if control & state == control {
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
                } else {
                    new_state.increment_state(
                        *state,
                        value.clone()
                    );
                }
            }
            
            new_state
        };
        Box::new(control_rz)
    }
}

impl Reversible for ControlRz {
    fn reverse(&self) -> Box<dyn Gate> {
        Box::new(ControlRz {
            n : self.n,
            wire : self.wire,
            control : self.control,
            angle : -self.angle,
        })
    }
}