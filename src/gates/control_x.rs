use crate::gate_util::*;
use crate::quantum_state::*;

#[derive(Debug, Clone)]
pub struct ControlX {
    n : usize,
    wire : usize,
    control : usize,
}

impl ControlX {
    pub fn new(n : usize, wire : usize, control : usize) -> ControlX {
        panic_on_out_of_bounds(n, wire);
        panic_on_out_of_bounds(n, control);
        panic_if_equal(wire, control);
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

impl Gate for ControlX { 
    fn get(&self) -> GateFunction {
        let n = self.n;
        let wire = self.wire;
        let checker_number = 1 << (n - 1 - self.control);

        let control_x = move |state : &QuantumState| {
            panic_on_length_mismatch(n, state.get_bit_length());

            let mut new_state = QuantumState::from_length(n);

            for (state, value) in state.iter() {
                if checker_number & state == checker_number {
                    new_state.increment_state(
                        state ^ (1 << (n - 1 - wire)),
                        value.clone()
                    )
                } else {
                    new_state.increment_state(
                        *state,
                        value.clone()
                    )
                }
            }
            
            new_state
        };
        Box::new(control_x)
    }
}

impl Reversible for ControlX {
    fn reverse(&self) -> Box<dyn Gate> {
        Box::new(ControlX {
            n : self.n,
            wire : self.wire,
            control : self.control,
        })
    }
}