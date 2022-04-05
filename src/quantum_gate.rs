use std::{f32::consts::{SQRT_2}};

use crate::quantum_state::QuantumState;

mod gates {
    pub struct Hadamard {
        n : usize,
        wire : usize,
    }
    
    pub struct ControlX {
        n : usize,
        wire : usize,
        control : usize,
    }
    
    pub struct Phase {
        n : usize,
        wire : usize,
        angle : f64,
    }

    fn panic_on_out_of_bounds(n : usize, wire : usize) {
        if wire >= n {
            panic!("Wire {} is out of bounds for {} qubits", wire, n);
        }
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

    impl ControlX {
        pub fn new(n : usize, wire : usize, control : usize) -> ControlX {
            panic_on_out_of_bounds(n, wire);
            panic_on_out_of_bounds(n, control);
            ControlX {
                n,
                wire,
                control,
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
    }

    impl Phase {
        pub fn new(n : usize, wire : usize, angle : f64) -> Phase {
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
        pub fn angle(&self) -> f64 {
            self.angle
        }
    }
}

use gates::{Hadamard, ControlX, Phase};

trait Gate {
    fn get(&self) -> Box<dyn Fn(&QuantumState) -> QuantumState>;
}

fn panic_on_length_mismatch(n : usize, bit_length : usize) {
    if n != bit_length {
        panic!("Quantum Gate and State Mismatch!")
    }
}

impl Gate for Hadamard {
    fn get(&self) -> Box<dyn Fn(&QuantumState) -> QuantumState> {
        let n = self.n();
        let wire = self.wire();
        let signature = 1 << (n - 1 - wire);

        let hadamard = move |state : &QuantumState| {
            panic_on_length_mismatch(n, state.get_bit_length());

            let mut new_state = QuantumState::from_length(n);

            for (state, value) in state.iter() {
                new_state.increment_state(
                    state & signature,
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