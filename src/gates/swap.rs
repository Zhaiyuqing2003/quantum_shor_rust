use crate::{gate_util::*, quantum_state::QuantumState};

#[derive(Debug, Clone)]
pub struct Swap {
    n : usize,
    wire_one : usize,
    wire_two : usize,
}

impl Swap {
    pub fn new(n : usize, wire_one : usize, wire_two : usize) -> Swap {
        panic_on_out_of_bounds(n, wire_one);
        panic_on_out_of_bounds(n, wire_two);
        panic_if_equal(wire_one, wire_two);
        Swap {
            n,
            wire_one,
            wire_two,
        }
    }
    pub fn n(&self) -> usize {
        self.n
    }
    pub fn wire_one(&self) -> usize {
        self.wire_one
    }
    pub fn wire_two(&self) -> usize {
        self.wire_two
    }
}

impl Gate for Swap {
    fn get(&self) -> GateFunction {
        let n = self.n;
        let wire_one = self.wire_one;
        let wire_two = self.wire_two;

        let swap = move |state : &QuantumState| {
            panic_on_length_mismatch(n, state.get_bit_length());

            let mut new_state = QuantumState::from_length(n);

            for (state, value) in state.iter() {
                let bit_one = (state >> wire_one) & 1;
                let bit_two = (state >> wire_two) & 1;

                let x = bit_one ^ bit_two;
                let x = (x << wire_one) | (x << wire_two);

                new_state.increment_state(
                    x ^ state,
                    value.clone()
                )
            }
            
            new_state
        };
        Box::new(swap)
    }
}

impl Reversible for Swap {
    fn reverse(&self) -> Box<dyn Gate> {
        Box::new(self.clone())
    }
}


