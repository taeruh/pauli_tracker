//! This module defines the [Tracker] trait and provides different implementors through
//! the [frames] and [live] module. The [Tracker] trait provides the core functionality
//! of tracking Pauli gates through a Clifford circuit.

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

/// This trait provides the core API to track Paulis through a clifford circuit. The
/// implementors must ensure that they implement the functions correctly according to
/// the conjugation rules of Clifford gates with Pauli Gates
///
/// *currently, the set of supported Cliffords is very limited, it will be extended over
/// time*
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

#[cfg(test)]
mod test {
    use super::*;

    // when we update the results here and use this module in the test of the tracker
    // implementors, the type system ensures that we test all gates/actions

    //           name for debugging, expected result
    pub type SingleResult = (&'static str, [u8; 4]);
    pub type DoubleResult = (&'static str, [u8; 16]);
    pub type SingleAction<T> = fn(&mut T, usize);
    pub type DoubleAction<T> = fn(&mut T, usize, usize);

    pub const N_SINGLES: usize = 2;
    const SINGLES: [SingleResult; N_SINGLES] =
        // pauli p = ab in binary; encoding: x = a, z = b; input: p = 0 1 2 3
        [("H", [0, 2, 1, 3]), ("S", [0, 1, 3, 2])];

    pub const N_DOUBLES: usize = 6;
    const DOUBLES: [DoubleResult; N_DOUBLES] = [
        // double-pauli p = abcd in binary;
        // encoding: x_0 = a, z_0 = b, x_1 = c, z_1 = d;
        // input: p = 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
        ("cx", [0, 5, 2, 7, 4, 1, 6, 3, 10, 15, 8, 13, 14, 11, 12, 9]),
        ("cz", [0, 1, 6, 7, 4, 5, 2, 3, 9, 8, 15, 14, 13, 12, 11, 10]),
        ("move_x_to_x", [0, 1, 2, 3, 4, 5, 6, 7, 2, 3, 0, 1, 6, 7, 4, 5]),
        ("move_x_to_z", [0, 1, 2, 3, 4, 5, 6, 7, 1, 0, 3, 2, 5, 4, 7, 6]),
        ("move_z_to_x", [0, 1, 2, 3, 2, 3, 0, 1, 8, 9, 10, 11, 10, 11, 8, 9]),
        ("move_z_to_z", [0, 1, 2, 3, 1, 0, 3, 2, 8, 9, 10, 11, 9, 8, 11, 10]),
    ];

    pub fn single_check<T, R>(runner: R, actions: [SingleAction<T>; N_SINGLES])
    where
        T: Tracker,
        R: Fn(SingleAction<T>, SingleResult),
    {
        for (action, result) in actions.into_iter().zip(SINGLES) {
            (runner)(action, result)
        }
    }

    pub fn double_check<T, R>(runner: R, actions: [DoubleAction<T>; N_DOUBLES])
    where
        T: Tracker,
        R: Fn(DoubleAction<T>, DoubleResult),
    {
        for (action, result) in actions.into_iter().zip(DOUBLES) {
            (runner)(action, result)
        }
    }

    pub mod utils {
        use crate::{
            pauli::Pauli,
            tracker::PauliString,
        };

        pub fn single_init(input: u8) -> PauliString {
            vec![(0, Pauli::try_from(input).unwrap())]
        }

        // masks to decode p in 0..16 into two paulis and vice versa
        const FIRST: u8 = 12;
        const FIRST_SHIFT: u8 = 2;
        const SECOND: u8 = 3;

        pub fn double_init(input: u8) -> PauliString {
            vec![
                (0, Pauli::try_from((input & FIRST) >> FIRST_SHIFT).unwrap()),
                (1, Pauli::try_from(input & SECOND).unwrap()),
            ]
        }

        pub fn double_output(frame: impl IntoIterator<Item = (usize, Pauli)>) -> u8 {
            let mut output = 0;
            for (i, p) in frame {
                if i == 0 {
                    output += p.storage() << FIRST_SHIFT
                } else if i == 1 {
                    output += p.storage()
                }
            }
            output
        }
    }
}
