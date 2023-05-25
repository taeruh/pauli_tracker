use crate::pauli::Pauli;

/// A vector describing an encoded Pauli string, for example, one frame of
/// [Frames](frames::Frames) (via [Frames::pop_frame](frames::Frames::pop_frame)). The
/// `usize` element is the qubit index of the `Pauli` However, importantly note, that it
/// is not optimal to build arrays with PauliStrings on the minor access. The library is
/// build to use implementors of [PauliStorage], which should have [PauliVec]s on the
/// minor array axis, as workhorses. This vector should be mainly used to analyze single
/// Pauli strings.
pub type PauliString = Vec<(usize, Pauli)>;

pub trait Tracker {
    type Stack;
    fn init(num_qubits: usize) -> Self;
    fn new_qubit(&mut self, bit: usize) -> Option<usize>;
    fn track_pauli(&mut self, bit: usize, pauli: Pauli);
    fn track_pauli_string(&mut self, string: PauliString);
    fn h(&mut self, bit: usize);
    fn s(&mut self, bit: usize);
    fn cx(&mut self, control: usize, target: usize);
    fn cz(&mut self, bit_a: usize, bit_b: usize);
    fn move_z_to_x(&mut self, source: usize, destination: usize);
    fn move_z_to_z(&mut self, source: usize, destination: usize);
    fn measure(&mut self, bit: usize) -> Option<Self::Stack>;
}

pub mod frames;
pub mod live;
