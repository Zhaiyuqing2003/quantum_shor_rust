use crate::{
    gate_util::*,
};

fn flip(start_wire : usize, end_wire : usize, n: usize) {
    panic_on_out_of_bounds(n, start_wire);
    panic_on_out_of_bounds(n, end_wire);
    panic_if_bigger_than(start_wire, end_wire);

    let middle_wire = (start_wire + end_wire) / 2;
    // 0, 1, 2, 3, 4, 5
    for wire in start_wire..middle_wire {
        
    }

}


pub fn get_qft_gate(start_wire : usize, end_wire : usize, n : usize) {
    // let vec  = Vec::new()
    let vec : Vec<Box<dyn Gate>> = Vec::new();

}