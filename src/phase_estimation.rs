use crate::{gates::{unitary_modular::{UnitaryModular}, hadamard::Hadamard, control_unitary_modular::ControlUnitaryModular}, gate_util::Gate, quantum_fourier_transformation::get_qft_gate};

pub fn get_phase_estimation_wire(estimation_wire : usize, unitary_modular_gate : UnitaryModular) -> Vec<Box<dyn Gate>> {
    if estimation_wire < 1 {
        panic!("estimation_wire must be greater than 0");
    }

    let gate_length = unitary_modular_gate.n();
    let wire_length = gate_length + estimation_wire;

    let mut vec : Vec<Box<dyn Gate>> = Vec::new();

    for i in 0..estimation_wire {
        vec.push(Box::new(
            Hadamard::new(wire_length,i)
        ))
    }

    for i in (0..estimation_wire).rev() {
        for _ in 0..(1 << (estimation_wire - i - 1)) {
            vec.push(Box::new(
                ControlUnitaryModular::new(
                    wire_length,
                    unitary_modular_gate.start_wire() + estimation_wire,
                    unitary_modular_gate.end_wire() + estimation_wire,
                    i,
                    unitary_modular_gate.x(),
                    unitary_modular_gate.modular()
                )
            ));
        }
    }

    get_qft_gate(0, estimation_wire, wire_length)
        .iter().rev()
        .for_each(|gate| {
            vec.push(gate.reverse());
        });

    vec
}