/*!
This module defines the [Tracker] trait and provides different implementors through the
[frames] and [live] module. The [Tracker] trait provides the core functionality of
tracking Pauli gates through a Clifford circuit.
*/

use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
    },
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use crate::pauli::Pauli;

/// A vector describing an encoded Pauli string, for example, one frame of
/// [Frames](frames::Frames) (via [Frames::pop_frame](frames::Frames::pop_frame)).
///
/// The `usize` element is the qubit index of the `Pauli` However, importantly note,
/// that it is not optimal to build arrays with PauliStrings on the minor access. The
/// library is build to use implementors of [StackStorage], which should have
/// [PauliVec](crate::pauli::PauliVec)s on the minor array axis, as workhorses. This
/// vector should be mainly used to analyze single Pauli strings.
///
/// [StackStorage]: frames::storage::StackStorage
pub type PauliString<T> = Vec<(usize, T)>;

/// The Error when we try to [measure](Tracker::measure) a missing qubit.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MissingStack {
    /// The missing qubit.
    pub bit: usize,
}
impl Display for MissingStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "there's no Pauli stack for qubit {}", self.bit)
    }
}
impl Error for MissingStack {}

macro_rules! single {
    ($(( $name:ident, $gate:literal),)*) => {$(
        /// Update the tracked frames according the
        #[doc=$gate]
        /// gate on qu`bit`.
        fn $name(&mut self, bit: usize);
    )*}
}

macro_rules! double {
    ($name:ident, $gate:literal) => {
        double!($name, $gate, bit_a, bit_b);
    };
    ($name:ident, $gate:literal, $bit_a:ident, $bit_b:ident) => {
        /// Update the tracked frames according to the
        #[doc=$gate]
        /// on the `
        #[doc=stringify!($bit_a)]
        /// ` and `
        #[doc=stringify!($bit_b)]
        /// ` qubits.
        fn $name(&mut self, $bit_a: usize, $bit_b: usize);
    };
}

macro_rules! movements {
    ($((
        $name:ident,
        $from_doc:literal,
        $to_doc:literal
    ),)*) => {$(
        /// "Move" the
        #[doc=$from_doc]
        /// Pauli stack from the `origin` qubit to the `destination` qubit, transforming
        /// it to an
        #[doc=$to_doc]
        /// stack.
        fn $name(&mut self, source: usize, destination: usize);
    )*}
}

macro_rules! track_pauli {
    ($(($name:ident, $gate:literal, $call:ident),)*) => {$(
        /// Track a new frame consisting of the [Pauli]
        #[doc = $gate]
        /// at qu`bit`.
        #[inline]
        fn $name(&mut self, bit: usize) {
            self.track_pauli(bit, Self::Pauli::$call() );
        }
    )*};
}

/// The core API to track Paulis through a clifford circuit.
///
/// The implementors must ensure that they implement the methods correctly according
/// to the conjugation rules of Clifford gates with Pauli gates.
///
/// For extensive examples, please refer to the [library documentation](crate#examples).
///
/// *currently, the set of supported Cliffords is very limited, it will be extended over
/// time*
pub trait Tracker {
    /// The storage type used to store the tracked Paulis for each qubit, e.g.,
    /// [PauliVec](crate::pauli::PauliVec) for the [Frames](frames::Frames) tracker.
    type Stack;

    /// The type of Pauli representation use for operations like
    /// [track_pauli](Self::track_pauli).
    type Pauli: Pauli;

    /// Initialize the tracker with qubits numbered from 0 to `num_bits`-1.
    fn init(num_bits: usize) -> Self;

    /// Insert a new qu`bit` into the tracker. If the qu`bit` is already present
    /// [Some](Some)(`bit`) is returned, otherwise [None]
    fn new_qubit(&mut self, bit: usize) -> Option<usize>;

    /// Track a new frame consisting of the [Pauli] gate `pauli` at qu`bit`.
    fn track_pauli(&mut self, bit: usize, pauli: Self::Pauli);

    /// Track a new frame including multiple [Pauli] gates, i.e., e [PauliString] to the
    /// Tracker, i.e., do [Tracker::track_pauli] for multiple [Pauli]s but all within
    /// the same frame.
    fn track_pauli_string(&mut self, string: PauliString<Self::Pauli>);

    track_pauli!((track_x, "X", new_x), (track_y, "Y", new_y), (track_z, "Z", new_z),);

    single!((h, "Hadamard"), (s, "S"),);

    double!(cx, "Control X (Control Not)", control, target);
    double!(cz, "Control Z");

    movements!(
        (move_x_to_x, "X", "X"),
        (move_x_to_z, "X", "Z"),
        (move_z_to_x, "Z", "X"),
        (move_z_to_z, "Z", "Z"),
    );

    /// Remove the Pauli stack on qu`bit`, if it is present.
    fn measure(&mut self, bit: usize) -> Result<Self::Stack, MissingStack>;
}

// {{ some helpers for simpler gate implementations
macro_rules! unwrap_get_mut {
    ($inner:expr, $bit:expr, $gate:expr) => {
        $inner
            .get_mut($bit)
            .unwrap_or_else(|| panic!("{}: qubit {} does not exist", $gate, $bit))
    };
}
use unwrap_get_mut;

// that's not stable yet (https://github.com/rust-lang/rust/issues/83527), so we have
// to do it manually or try it with a functional macro

// macro_rules! create_single {
//     ($inner:ident) => {
//         macro_rules! single {
//             ($$($$name:ident),*) => {$$(
//                 fn $$name(&mut self, bit: usize) {
//                     unwrap_get_mut!(self.$inner, bit, stringify!($$name)).$name()
//                 }
//             )*};
//         }
//     }
// }
// use create_single;

macro_rules! unwrap_get_two_mut {
    ($inner:expr, $bit_a:expr, $bit_b:expr, $gate:expr) => {
        $inner.get_two_mut($bit_a, $bit_b).unwrap_or_else(|| {
            panic!("{}: qubit {} and/or {} do not exist", $gate, $bit_a, $bit_b)
        })
    };
}
use unwrap_get_two_mut;
// }}

pub mod frames;
pub mod live;

#[cfg(test)]
mod test {
    pub mod impl_utils {
        use super::super::*;
        use crate::{
            pauli::PauliDense,
            tracker::PauliString,
        };

        // when we update the results here and use this module in the test of the tracker
        // implementors, the type system ensures that we test all gates/actions

        //                 name for debugging, expected results
        pub type SingleResults = (&'static str, [u8; 4]);
        pub type DoubleResults = (&'static str, [(u8, u8); 16]);
        pub type SingleAction<T> = fn(&mut T, usize);
        pub type DoubleAction<T> = fn(&mut T, usize, usize);

        // the following expected results are proven in ./docs/conjugation_rules.md

        pub const N_SINGLES: usize = 2;
        const SINGLE_GENERATORS: [(&str, [u8; 2]); N_SINGLES] =
            // (name, [conjugate X, conjugate Z])
            [("H", [1, 2]), ("S", [3, 1])];

        pub const N_DOUBLES: usize = 6;
        const DOUBLE_GENERATORS: [(&str, [(u8, u8); 4]); N_DOUBLES] = [
            // (name, [conjugate X1, conjugate Z1, conjugate 1X, conjugate 1Z])
            ("cx", [(2, 2), (1, 0), (0, 2), (1, 1)]),
            ("cz", [(2, 1), (1, 0), (1, 2), (0, 1)]),
            // these here are not conjugations with unitary operators, however it still
            // works, because the move operation is a homomorphism
            ("move_x_to_x", [(0, 2), (1, 0), (0, 2), (0, 1)]),
            ("move_x_to_z", [(0, 1), (1, 0), (0, 2), (0, 1)]),
            ("move_z_to_x", [(2, 0), (0, 2), (0, 2), (0, 1)]),
            ("move_z_to_z", [(2, 0), (0, 1), (0, 2), (0, 1)]),
        ];

        #[cfg_attr(coverage_nightly, no_coverage)]
        pub fn single_check<T, R>(runner: R, actions: [SingleAction<T>; N_SINGLES])
        where
            T: Tracker,
            R: Fn(SingleAction<T>, SingleResults),
        {
            for (action, result_generator) in actions.into_iter().zip(SINGLE_GENERATORS)
            {
                let mut results = [0; 4];
                for (i, r) in results.iter_mut().enumerate() {
                    *r = (if (i & 2) > 0 { result_generator.1[0] } else { 0 })
                        ^ (if (i & 1) > 0 { result_generator.1[1] } else { 0 })
                }
                (runner)(action, (result_generator.0, results))
            }
        }

        #[cfg_attr(coverage_nightly, no_coverage)]
        pub fn double_check<T, R>(runner: R, actions: [DoubleAction<T>; N_DOUBLES])
        where
            T: Tracker,
            R: Fn(DoubleAction<T>, DoubleResults),
        {
            for (action, result_generator) in actions.into_iter().zip(DOUBLE_GENERATORS)
            {
                let mut results = [(0, 0); 16];
                for (i, r) in (0..).zip(results.iter_mut()) {
                    let a = if (i & 8) > 0 { result_generator.1[0] } else { (0, 0) };
                    let b = if (i & 4) > 0 { result_generator.1[1] } else { (0, 0) };
                    let c = if (i & 2) > 0 { result_generator.1[2] } else { (0, 0) };
                    let d = if (i & 1) > 0 { result_generator.1[3] } else { (0, 0) };
                    *r = (a.0 ^ b.0 ^ c.0 ^ d.0, a.1 ^ b.1 ^ c.1 ^ d.1)
                }
                (runner)(action, (result_generator.0, results))
            }
        }

        #[cfg_attr(coverage_nightly, no_coverage)]
        pub fn single_init<T: From<PauliDense>>(input: u8) -> PauliString<T> {
            vec![(0, PauliDense::try_from(input).unwrap().into())]
        }

        // masks to decode p in 0..16 into two paulis and vice versa
        const FIRST: u8 = 12; // = 1100
        const SECOND: u8 = 3; // = 0011
        const FIRST_SHIFT: u8 = 2;

        #[cfg_attr(coverage_nightly, no_coverage)]
        pub fn double_init<T: From<PauliDense>>(input: u8) -> PauliString<T> {
            vec![
                (
                    0,
                    PauliDense::try_from((input & FIRST) >> FIRST_SHIFT)
                        .unwrap()
                        .into(),
                ),
                (1, PauliDense::try_from(input & SECOND).unwrap().into()),
            ]
        }

        #[cfg_attr(coverage_nightly, no_coverage)]
        pub fn double_output<T: Into<PauliDense>>(
            frame: impl IntoIterator<Item = (usize, T)>,
        ) -> (u8, u8) {
            let mut output = [0, 0];
            for (i, p) in frame {
                output[i] = p.into().storage()
            }
            (output[0], output[1])
        }
    }
}
