use crate::quantum_state::QuantumState;

pub fn panic_on_out_of_bounds(n : usize, wire : usize) {
    if wire >= n {
        panic!("Wire {} is out of bounds for {} qubits", wire, n);
    }
}

pub fn panic_if_equal(wire1 : usize, wire2 : usize) {
    if wire1 == wire2 {
        panic!("Wire {} is equal to wire {}", wire1, wire2);
    }
}

pub fn panic_if_bigger_than(wire1 : usize, wire2 : usize) {
    if wire1 > wire2 {
        panic!("Wire {} is bigger than wire {}", wire1, wire2);
    }
}

pub fn panic_on_length_mismatch(n : usize, bit_length : usize) {
    if n != bit_length {
        panic!("Quantum Gate and State Mismatch!")
    }
}

pub trait Gate {
    fn get(&self) -> GateFunction;
}
pub trait Reversible {
    fn reverse(&self) -> Box<dyn Gate>;
}

pub type GateFunction = Box<dyn Fn(&QuantumState) -> QuantumState>;
