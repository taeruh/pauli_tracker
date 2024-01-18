/*!
This module defines the [Tracker] trait and provides the [Frames] and
[Live] implementors.

The [Tracker] trait provides the core functionality of tracking Pauli gates through a
Clifford circuit.

[Frames] is a tracker that is useful for anlyzing the dependency flow, for example, in
MBQC, or in general when gates are injected or teleported and have non-deterministic
side effects.

[Live] can be used to track Pauli gates during the actual execution of a circuit to
adopt measurements correctly.

[Frames]: frames::Frames
[Live]: live::Live
[MBQC]: https://doi.org/10.48550/arXiv.0910.1116
*/

use thiserror::Error;

use crate::{
    clifford_helper,
    pauli::Pauli,
};

/// A vector describing an encoded Pauli string.
///
/// For example, one frame of [Frames](frames::Frames) (via
/// [Frames::pop_frame](frames::Frames::pop_frame)). The `usize` element is the qubit
/// index of the `Pauli`.
pub type PauliString<T> = Vec<(usize, T)>;

/// The Error when one tries to [measure](Tracker::measure) a missing bit.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("there's no Pauli stack for qubit {0}")]
pub struct MissingBit(pub usize);

macro_rules! single_doc_standard {
    ($gate:literal) => {
        concat!(
            "Update the tracked frames according the ",
            $gate,
            " gate on the qu`bit`."
        )
    };
}
macro_rules! single_doc_equivalent {
    ($gate:literal, $equiv:literal) => {
        concat!(
            single_doc_standard!($gate),
            " Equivalent to the ",
            $equiv,
            " gate."
        )
    };
}

macro_rules! double_doc {
    ($gate:literal) => {
        double_doc!($gate, bit_a, bit_b)
    };
    ($gate:literal, $bit_a:ident, $bit_b:ident) => {
        concat!(
            "Update the tracked frames according to the ",
            $gate,
            " on the `",
            stringify!($bit_a),
            "` and `",
            stringify!($bit_b),
            "` qubits."
        )
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
    ($(($name:ident, $gate:ident),)*) => {$(
        /// Track a new frame consisting of the Pauli
        #[doc = stringify!($gate)]
        /// at qu`bit`.
        fn $name(&mut self, bit: usize) {
            self.track_pauli(bit, Self::Pauli::$gate );
        }
    )*};
}

macro_rules! coset {
    ($coset:ident, $coset_name:literal, $(($name:ident, $gate:literal),)*) => {$(
        #[doc = single_doc_equivalent!($gate, $coset_name)]
        fn $name(&mut self, bit: usize) {
            self.$coset(bit);
        }
    )*};
}

/// The core API to track Paulis through a Clifford circuit.
///
/// The implementors must ensure that they implement the methods correctly according
/// to the conjugation rules of Clifford gates with Pauli gates
/// [^rust_analyzer_impl_members]. While many gates have default implementations, one
/// might want to implement them directly for performance reasons[^generators].
///
/// For extensive examples, please refer to the [library documentation](crate#examples).
///
/// The supported gates are also described in [conjugation-rules]. Note that all
/// Clifford gates can be generated from [S](Tracker::s), [H](Tracker::h) and
/// [CZ](Tracker::cz) (cf [conjugation-rules]).
///
/// [^rust_analyzer_impl_members]: Using rust-analyzer's "implement members" feature
/// inserts some weird looking docs, which may not compile. This is because we generate
/// a bunch of the methods with macros. You should delete these docs.
///
/// [^generators]: The default implementations are implemented using the generators
/// [Tracker::h], [Tracker::s] and [Tracker::cz]. For example, [Tracker::cx] is
/// implement as `self.h(target); self.cz(control, target); self.h(target);`. This can
/// probably be done more efficiently by directly implementing this method (it's for
/// example trivial for [Tracker::swap]: three cnots vs one mem::swap). On the other
/// hand, the default implementations are making use of the fact that the update rules
/// are the same per coset with respect to the Pauli group, e.g., [Tracker::sdg]
/// is just `self.s(target);`, which is hopefully inlined (at least after lto). There's
/// probably no need to implement them directly, but only to implement the canonical
/// coset repepresentatives directly. For the single qubit gates, these are: I, S, H,
/// SH, HS, SHS (these are all single qubit Cliffords up to Paulis and phases). For the
/// double-qubit gates, the standard representatives are: CZ, CX, CY, SWAP, iSWAP (these
/// are NOT all two-qubits cosets (there are 720); we don't write our standard
/// representatives in  S, H and CZ, because that would really get out of hand, instead
/// we try to choose the most common gates). The [conjugation-rules] document contains
/// some useful operator identities.
///
/// [conjugation-rules]:
/// https://github.com/taeruh/pauli_tracker/blob/main/docs/conjugation_rules.pdf
pub trait Tracker {
    /// The storage type used to store the tracked Paulis for each qubit, e.g.,
    /// [PauliStack](crate::pauli::PauliStack) for the [Frames](frames::Frames) tracker or
    /// just a simple [Pauli] for the [Live](live::Live) tracker (in this case it's a
    /// stack with one element ...).
    type Stack;

    /// The type of Pauli representation used for operations like
    /// [track_pauli](Self::track_pauli). It is usally the type that is the most
    /// compatible with [Self::Stack].
    type Pauli: Pauli;

    /// Insert a new qu`bit` into the tracker. If the qu`bit`, the old value is
    /// overwritten and returned.
    fn new_qubit(&mut self, bit: usize) -> Option<Self::Stack>;

    /// Track a new frame consisting of the Pauli gate `pauli` at qu`bit`.
    ///
    /// If qu`bit` is not tracked, the method does not error, but simply tracks an empty
    /// frame.
    fn track_pauli(&mut self, bit: usize, pauli: Self::Pauli);

    /// Track a new frame including multiple Pauli gates, i.e., i.e., do
    /// [Tracker::track_pauli] for multiple Paulis but all within the same frame.
    fn track_pauli_string(&mut self, string: PauliString<Self::Pauli>);

    track_pauli!((track_x, X), (track_y, Y), (track_z, Z),);

    clifford_helper::trait_gates!();

    movements!(
        (move_x_to_x, "X", "X"),
        (move_x_to_z, "X", "Z"),
        (move_z_to_x, "Z", "X"),
        (move_z_to_z, "Z", "Z"),
    );

    /// Remove the Pauli stack on qu`bit`, if it is present.
    fn measure(&mut self, bit: usize) -> Result<Self::Stack, MissingBit>;
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
//         macro_rules! single_gate {
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
mod tests {
    use super::*;
    pub mod utils {
        use super::*;
        use crate::pauli::PauliDense;

        // when we update the results here and use this module in the test of the tracker
        // implementors, the type system ensures that we test all gates/actions

        //                 name for debugging, expected results
        pub type SingleResults = (&'static str, [u8; 4]);
        pub type DoubleResults = (&'static str, [(u8, u8); 16]);
        pub type SingleAction<T> = fn(&mut T, usize);
        pub type DoubleAction<T> = fn(&mut T, usize, usize);

        // the following expected results are proven in ./docs/conjugation_rules.pdf
        //
        // instead of writing out all the SingleResults and DoubleResults, we make use
        // of homomorphy and just define the results on a basis
        //
        // the encoding is according to crate::pauli::tableau_encoding, i.e., 0=I, 2=X,
        // 3=Y, 1=Z

        pub const N_SINGLES: usize = 18;
        #[rustfmt::skip]
        const SINGLE_GENERATORS: [(&str, [u8; 2]); N_SINGLES] =
            // (name, result: [conjugate Z, conjugate X])
            [
                ("I",    [1, 2]),
                ("X",    [1, 2]),
                ("Y",    [1, 2]),
                ("Z",    [1, 2]),
                ("S",    [1, 3]),
                ("SDG",  [1, 3]),
                ("SZ",   [1, 3]),
                ("SZDG", [1, 3]),
                ("H_xy", [1, 3]),
                ("H",    [2, 1]),
                ("SY",   [2, 1]),
                ("SYDG", [2, 1]),
                ("SH",   [3, 1]),
                ("HS",   [2, 3]),
                ("SHS",  [3, 2]),
                ("SX",   [3, 2]),
                ("SXDG", [3, 2]),
                ("H_yz", [3, 2]),
            ];

        macro_rules! single_actions {
            ($tracker:ty) => {
                [
                    <$tracker>::id,
                    <$tracker>::x,
                    <$tracker>::y,
                    <$tracker>::z,
                    <$tracker>::s,
                    <$tracker>::sdg,
                    <$tracker>::sz,
                    <$tracker>::szdg,
                    <$tracker>::hxy,
                    <$tracker>::h,
                    <$tracker>::sy,
                    <$tracker>::sydg,
                    <$tracker>::sh,
                    <$tracker>::hs,
                    <$tracker>::shs,
                    <$tracker>::sx,
                    <$tracker>::sxdg,
                    <$tracker>::hyz,
                ]
            };
        }
        pub(crate) use single_actions;

        pub const N_DOUBLES: usize = 10;
        #[rustfmt::skip]
        const DOUBLE_GENERATORS: [(&str, [(u8, u8); 4]); N_DOUBLES] = [
            //+ (name, result: [conjugate Z1, conjugate Z2, conjugate X1, conjugate X2])
            // the left tuple entry of the results belongs to the second qubit (q0) in the
            // function call and the right entry to the first one (q1), i.e., q1 controls
            // q0
            ("cz",          [(1, 0), (0, 1), (2, 1), (1, 2)]),
            ("cx",          [(1, 1), (0, 1), (2, 0), (2, 2)]),
            ("cy",          [(1, 1), (0, 1), (2, 1), (3, 2)]),
            ("swap",        [(0, 1), (1, 0), (0, 2), (2, 0)]),
            ("iswap",       [(0, 1), (1, 0), (1, 3), (3, 1)]),
            ("iswapdg",     [(0, 1), (1, 0), (1, 3), (3, 1)]),
            // these here are not conjugations with unitary operators, however it still
            // works, because the move operation is a homomorphism
            ("move_x_to_x", [(1, 0), (0, 1), (2, 0), (2, 0)]),
            ("move_x_to_z", [(1, 0), (0, 1), (2, 0), (1, 0)]),
            ("move_z_to_x", [(1, 0), (2, 0), (2, 0), (0, 2)]),
            ("move_z_to_z", [(1, 0), (1, 0), (2, 0), (0, 2)]),
        ];

        macro_rules! double_actions {
            ($tracker:ty) => {
                [
                    <$tracker>::cz,
                    <$tracker>::cx,
                    <$tracker>::cy,
                    <$tracker>::swap,
                    <$tracker>::iswap,
                    <$tracker>::iswapdg,
                    <$tracker>::move_x_to_x,
                    <$tracker>::move_x_to_z,
                    <$tracker>::move_z_to_x,
                    <$tracker>::move_z_to_z,
                ]
            };
        }
        pub(crate) use double_actions;

        #[cfg_attr(coverage_nightly, coverage(off))]
        pub fn single_check<T, R>(runner: R, actions: [SingleAction<T>; N_SINGLES])
        where
            T: Tracker,
            R: Fn(SingleAction<T>, SingleResults),
        {
            for (action, result_generator) in actions.into_iter().zip(SINGLE_GENERATORS) {
                let mut results = [0; 4];
                for (i, r) in results.iter_mut().enumerate() {
                    *r = (if (i & 1) > 0 { result_generator.1[0] } else { 0 })
                        ^ (if (i & 2) > 0 { result_generator.1[1] } else { 0 })
                }
                (runner)(action, (result_generator.0, results))
            }
        }

        #[cfg_attr(coverage_nightly, coverage(off))]
        pub fn double_check<T, R>(runner: R, actions: [DoubleAction<T>; N_DOUBLES])
        where
            T: Tracker,
            R: Fn(DoubleAction<T>, DoubleResults),
        {
            for (action, result_generator) in actions.into_iter().zip(DOUBLE_GENERATORS) {
                let mut results = [(0, 0); 16];
                for (i, r) in (0..).zip(results.iter_mut()) {
                    // cf. the masks below in double_init
                    let a = if (i & 1) > 0 { result_generator.1[0] } else { (0, 0) };
                    let b = if (i & 2) > 0 { result_generator.1[2] } else { (0, 0) };
                    let c = if (i & 4) > 0 { result_generator.1[1] } else { (0, 0) };
                    let d = if (i & 8) > 0 { result_generator.1[3] } else { (0, 0) };
                    *r = (a.0 ^ b.0 ^ c.0 ^ d.0, a.1 ^ b.1 ^ c.1 ^ d.1)
                }
                (runner)(action, (result_generator.0, results))
            }
        }

        #[cfg_attr(coverage_nightly, coverage(off))]
        pub fn single_init<T: From<PauliDense>>(input: u8) -> PauliString<T> {
            vec![(0, PauliDense::try_from(input).unwrap().into())]
        }

        #[cfg_attr(coverage_nightly, coverage(off))]
        pub fn double_init<T: From<PauliDense>>(input: u8) -> PauliString<T> {
            // masks to decode p in 0..16 into two paulis and vice versa
            const SECOND: u8 = 12; // = 1100
            const FIRST: u8 = 3; // = 0011
            const SECOND_SHIFT: u8 = 2;
            vec![
                (
                    1,
                    PauliDense::try_from((input & SECOND) >> SECOND_SHIFT)
                        .unwrap()
                        .into(),
                ),
                (0, PauliDense::try_from(input & FIRST).unwrap().into()),
            ]
        }

        #[cfg_attr(coverage_nightly, coverage(off))]
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

    mod defaults {
        use coverage_helper::test;

        use super::{
            super::*,
            utils::{
                DoubleAction,
                DoubleResults,
                N_DOUBLES,
            },
        };
        use crate::{
            collection::{
                Base,
                Map,
            },
            pauli::{
                Pauli,
                PauliDense,
            },
        };

        #[derive(Debug)]
        struct DefaultTester {
            paulis: Map<PauliDense>,
            skip_it: bool,
        }

        impl DefaultTester {
            fn init(n: usize) -> Self {
                Self {
                    paulis: Map::from_iter((0..n).map(|i| (i, PauliDense::I))),
                    skip_it: false,
                }
            }
        }

        impl Tracker for DefaultTester {
            type Stack = PauliDense;
            type Pauli = PauliDense;

            fn new_qubit(&mut self, bit: usize) -> Option<Self::Stack> {
                self.paulis.insert(bit, PauliDense::I)
            }
            fn track_pauli(&mut self, _: usize, _: Self::Pauli) {
                todo!()
            }
            fn track_pauli_string(&mut self, string: PauliString<Self::Pauli>) {
                for (bit, pauli) in string {
                    if let Some(p) = self.paulis.get_mut(&bit) {
                        p.add(pauli)
                    }
                }
            }

            fn h(&mut self, bit: usize) {
                self.paulis.get_mut(&bit).unwrap().h()
            }
            fn s(&mut self, bit: usize) {
                self.paulis.get_mut(&bit).unwrap().s()
            }
            fn cz(&mut self, bit_a: usize, bit_b: usize) {
                let (a, b) = self.paulis.get_two_mut(bit_a, bit_b).unwrap();
                a.zpx(b);
                b.zpx(a);
            }

            fn move_x_to_x(&mut self, _: usize, _: usize) {
                self.skip_it = true
            }
            fn move_x_to_z(&mut self, _: usize, _: usize) {
                self.skip_it = true
            }
            fn move_z_to_x(&mut self, _: usize, _: usize) {
                self.skip_it = true
            }
            fn move_z_to_z(&mut self, _: usize, _: usize) {
                self.skip_it = true
            }

            fn measure(&mut self, _: usize) -> Result<Self::Stack, MissingBit> {
                todo!()
            }
        }

        use super::*;
        use crate::tracker::tests::utils::{
            self,
            SingleAction,
            SingleResults,
            N_SINGLES,
        };

        type ActionS = SingleAction<DefaultTester>;
        type ActionD = DoubleAction<DefaultTester>;

        #[cfg_attr(coverage_nightly, coverage(off))]
        fn single_runner(action: ActionS, result: SingleResults) {
            for (input, check) in (0u8..).zip(result.1) {
                let mut tracker = DefaultTester::init(2);
                tracker.track_pauli_string(utils::single_init(input));
                (action)(&mut tracker, 0);
                let computed = tracker.paulis.get(&0).unwrap().storage();
                assert_eq!(
                    computed, check,
                    "gate: {}, input: {}, expected: {}, computed: {}",
                    result.0, input, check, computed
                );
            }
        }

        #[test]
        fn single_actions() {
            let actions: [ActionS; N_SINGLES] = utils::single_actions!(DefaultTester);
            utils::single_check(single_runner, actions);
        }

        #[cfg_attr(coverage_nightly, coverage(off))]
        fn double_runner(action: ActionD, result: DoubleResults) {
            for (input, check) in (0u8..).zip(result.1) {
                let mut tracker = DefaultTester::init(2);
                tracker.track_pauli_string(utils::double_init(input));
                (action)(&mut tracker, 1, 0);
                if tracker.skip_it {
                    tracker.skip_it = false;
                    return;
                }
                let computed = utils::double_output(tracker.paulis);
                assert_eq!(
                    computed, check,
                    "{}, {}, {:?}, {:?}",
                    result.0, input, check, computed
                );
            }
        }

        #[test]
        fn double_actions() {
            let actions: [ActionD; N_DOUBLES] = utils::double_actions!(DefaultTester);
            utils::double_check(double_runner, actions);
        }
    }
}
