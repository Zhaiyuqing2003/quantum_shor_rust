use std::f32::consts::PI;

use crate::gates::control_phase::ControlPhase;
use crate::gates::hadamard::Hadamard;
use crate::{
    gate_util::*,
};
use crate::gates::swap::Swap;

fn get_flip_gate(start_wire : usize, end_wire : usize, n: usize) -> Vec<Box<dyn Reversible>> {
    panic_on_out_of_bounds(n, start_wire);
    panic_on_out_of_bounds(n, end_wire);
    panic_if_bigger_than(start_wire, end_wire);

    let middle_wire = (start_wire + end_wire) / 2;
    // let vec : Vec<Box<dyn ReversibleGate>> = Vec::new();
    // 0, 1, 2, 3, 4, 5
    let mut vec : Vec<Box<dyn Reversible>> = Vec::new(); 
    for current_wire in start_wire..middle_wire {
        // swap current_wire and end_wire - current_wire
        vec.push(Box::new(
            Swap::new(n, current_wire, end_wire - current_wire)
        ));
    }
    vec
}

pub fn get_qft_gate(start_wire : usize, end_wire : usize, n : usize) -> Vec<Box<dyn Reversible>>{
    // let vec  = Vec::new()
    let mut vec = get_flip_gate(start_wire, end_wire, n);

    // for-loop starting from end_wire - 1 to start_wire, end_wire is bigger than start_wire
    for i in (start_wire..end_wire).rev() {
        for j in (end_wire..i+1).rev() {
            vec.push(Box::new(
                ControlPhase::new(
                    n, j, i, 
                    PI / (2.0f32).powi((j - i) as i32)
                )
            ))
        }


        vec.push(Box::new(
            Hadamard::new(n, i)
        ));
    }

    vec
}

