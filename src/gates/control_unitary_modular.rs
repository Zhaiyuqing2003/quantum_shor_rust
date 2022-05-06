use crate::{gate_util::*, quantum_state::QuantumState};

pub struct ControlUnitaryModular {
    n : usize,
    start_wire : usize,
    end_wire : usize,
    control : usize,
    x : usize,
    modular : usize,
}

impl ControlUnitaryModular {
    pub fn new(n : usize, start_wire : usize, end_wire : usize, control : usize, x : usize, modular : usize) -> ControlUnitaryModular {
        panic_on_out_of_bounds(n, start_wire);
        panic_on_out_of_bounds(n, end_wire - 1);
        panic_if_bigger_than(start_wire, end_wire);
        panic_if_equal(start_wire, end_wire);
        panic_if_equal(control, start_wire);

        ControlUnitaryModular {
            n,
            start_wire,
            end_wire,
            control,
            x,
            modular,
        }
    }
    pub fn n(&self) -> usize {
        self.n
    }
    pub fn start_wire(&self) -> usize {
        self.start_wire
    }
    pub fn end_wire(&self) -> usize {
        self.end_wire
    }
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn modular(&self) -> usize {
        self.modular
    }
    pub fn control(&self) -> usize {
        self.control
    }
}

impl Gate for ControlUnitaryModular {
    fn get(&self) -> GateFunction {
        let n = self.n;
        let start_wire = self.start_wire;
        let end_wire = self.end_wire;
        let x = self.x;
        let modular = self.modular;
        let checker_number = 1 << (n - 1 - self.control);

        let unitary_modular = move |state : &QuantumState| {
            panic_on_length_mismatch(n, state.get_bit_length());

            let mut new_state = QuantumState::from_length(n);

            for (state, value) in state.iter() {
                if checker_number & state == checker_number {
                    let y = (state >> (n - end_wire)) & ((1 << (end_wire - start_wire)) - 1);
                    let new_y = if y >= modular {
                        y
                    } else {
                        (y * x) % modular
                    };
    
                    new_state.increment_state(
                        ((state >> (n - start_wire) << (end_wire - start_wire) | new_y) << (n - end_wire)) |
                            (state & ((1 << (n - end_wire)) - 1)), 
                        value.clone()
                    )
                } else {
                    new_state.increment_state(
                        *state,
                        value.clone()
                    );
                }
            }
            new_state
        };

        Box::new(unitary_modular)
    }
}