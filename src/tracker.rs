use crate::pauli::Pauli;

/// A vector describing an encoded Pauli string, for example, one frame of
/// [Frames](frames::Frames) (via [Frames::pop_frame](frames::Frames::pop_frame)). The
/// `usize` element is the qubit index of the `Pauli` However, importantly note, that it
/// is not optimal to build arrays with PauliStrings on the minor access. The library is
/// build to use implementors of [StackStorage], which should have [PauliVec]s on the
/// minor array axis, as workhorses. This vector should be mainly used to analyze single
/// Pauli strings.
///
/// [StackStorage]: frames::storage::StackStorage
/// [PauliVec]: frames::storage::PauliVec
pub type PauliString = Vec<(usize, Pauli)>;

pub trait Tracker {
    type Stack;
    /// Initialize the tracker with qubits from 0 to `num_bits`-1.
    fn init(num_bits: usize) -> Self;

    /// Insert a new qu`bit` into the tracker. If the qu`bit` is already present
    /// [Some](Some)(`bit`) is returned, otherwise [None]
    fn new_qubit(&mut self, bit: usize) -> Option<usize>;

    /// Append the Tracker with one frame consisting of the [Pauli] gate `pauli` at
    /// qu`bit`.
    fn track_pauli(&mut self, bit: usize, pauli: Pauli);

    /// Append a frame including multiple [Pauli] gates, i.e., e [PauliString] to the
    /// Tracker, i.e., do [Tracker::track_pauli] for multiple [Pauli]s but all within
    /// the same frame
    fn track_pauli_string(&mut self, string: PauliString);

    /// Update the tracked frames according to a Hadamard gate on qu`bit`
    fn h(&mut self, bit: usize);
    /// Update the tracked frames according to an S gate on qu`bit`
    fn s(&mut self, bit: usize);

    /// Update the tracked frames according to Control X (Control Not) on the `control`
    /// and `target` bits.
    fn cx(&mut self, control: usize, target: usize);
    /// Update the tracked frames according to Control Z on `bit_a` and `bit_b`.
    fn cz(&mut self, bit_a: usize, bit_b: usize);

    /// "Move" the Z Pauli stack from `origin` to `destination`, transforming it to an X
    /// stack. "Moving" means removing on `origin` and adding (mod 2) on `destination`.
    fn move_z_to_x(&mut self, source: usize, destination: usize);
    /// "Move" the Z Pauli stack from `origin` to `destination`, transforming it to an Z
    /// stack. "Moving" means removing on `origin` and adding (mod 2) on `destination`.
    fn move_z_to_z(&mut self, source: usize, destination: usize);
    /// "Move" the X Pauli stack from `origin` to `destination`, transforming it to an X
    /// stack. "Moving" means removing on `origin` and adding (mod 2) on `destination`.
    fn move_x_to_x(&mut self, source: usize, destination: usize);
    /// "Move" the X Pauli stack from `origin` to `destination`, transforming it to an Z
    /// stack. "Moving" means removing on `origin` and adding (mod 2) on `destination`.
    fn move_x_to_z(&mut self, source: usize, destination: usize);

    /// Remove the Pauli stack on qu`bit`, if it is present
    fn measure(&mut self, bit: usize) -> Option<Self::Stack>;
}

pub mod frames;
pub mod live;
