use crate::pauli::Pauli;

use super::Tracker;

pub struct Frame {
    inner: Vec<Pauli>,
}

impl Tracker for Frame {
    type Stack = Pauli;

    fn init(num_qubits: usize) -> Self {
        todo!()
    }

    fn new_qubit(&mut self, bit: usize) -> Option<usize> {
        todo!()
    }

    fn track_pauli(&mut self, bit: usize, pauli: Pauli) {
        todo!()
    }

    fn track_pauli_string(&mut self, string: super::PauliString) {
        todo!()
    }

    fn h(&mut self, bit: usize) {
        todo!()
    }

    fn s(&mut self, bit: usize) {
        todo!()
    }

    fn cx(&mut self, control: usize, target: usize) {
        todo!()
    }

    fn cz(&mut self, bit_a: usize, bit_b: usize) {
        todo!()
    }

    fn move_z_to_x(&mut self, source: usize, destination: usize) {
        todo!()
    }

    fn move_z_to_z(&mut self, source: usize, destination: usize) {
        todo!()
    }

    fn measure(&mut self, bit: usize) -> Option<Self::Stack> {
        todo!()
    }
}
