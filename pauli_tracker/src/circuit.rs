/*!
A circuit wrapper around a Clifford circuit and a [Tracker].

The main content of this module is a wrapper [TrackedCircuit] that provides an methods
to track Paulis while building up the circuit or executing it. One can either use it
directly by providing an appropriate circuit simulator (should implement
[CliffordCircuit]) or use it as template/idea to write a custom wrapper. The module also
provides two pseudo circuit simulators that can be used to test the Pauli tracking.
*/

use std::mem;

use crate::{
    boolean_vector::BooleanVector,
    clifford_helper,
    collection::{Base, Full},
    pauli::PauliStack,
    tracker::{
        PauliString, Tracker,
        frames::{Frames, MoveError, OverwriteStack},
    },
};

macro_rules! single_doc_standard {
    ($gate:literal) => {
        concat!("Apply the ", $gate, " gate on the qu`bit`.")
    };
}
macro_rules! single_doc_equivalent {
    ($gate:literal, $equiv:literal) => {
        concat!(single_doc_standard!($gate), " Equivalent to the ", $equiv, " gate.")
    };
}

macro_rules! double_doc {
    ($gate:literal) => {
        double_doc!($gate, bit_a, bit_b)
    };
    ($gate:literal, $bit_a:ident, $bit_b:ident) => {
        concat!(
            "Apply the ",
            $gate,
            " on the `",
            stringify!($bit_a),
            "` and `",
            stringify!($bit_b),
            "` qubits."
        )
    };
}

macro_rules! coset {
    ($coset:ident, $coset_name:literal, $(($name:ident, $gate:literal),)*) => {$(
        #[doc = single_doc_equivalent!($gate, $coset_name)]
        fn $name(&mut self, bit: usize) {
            self.$coset(bit);
        }
    )*};
}

/// API for Clifford gates.
///
/// We don't really care what the circuit is actually doing, except for possible
/// measurement outcomes, since we only use this interface to pass the actions through
/// to the implementing circuit.
pub trait CliffordCircuit {
    /// The type of the measurement outcome, e.g., a boolean for
    /// [RandomMeasurementCircuit].
    type Outcome;

    clifford_helper::trait_gates!();

    /// Measure (unspecified)
    fn measure(&mut self, bit: usize) -> Self::Outcome;
}

mod dummies;
pub use dummies::{DummyCircuit, RandomMeasurementCircuit};

/// A Wrapper around a Clifford circuit (simulator) and a Pauli tracker.
///
/// It basically just passes through most function calls directly to its circuit and
/// tracker.
///
/// The type can be used to build up the underlining circuit, while keeping track of the
/// Pauli gates that shall be extracted from the (quantum) simulation, e.g., the Pauli
/// corrections in [MBQC].
///
/// [MBQC]: https://doi.org/10.48550/arXiv.0910.1116
// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrackedCircuit<Circuit, Tracker, Storage> {
    /// The underlining circuit (simulator). Should implement [CliffordCircuit]
    pub circuit: Circuit,
    /// The tracker of the Pauli frames.
    pub tracker: Tracker,
    /// An additional storage to store measurement results.
    pub storage: Storage,
}

// split impl into multiple blocks with the minimum required bounds, so that it is
// simpler to write generic functions later on

// cf create_single comment in tracker.rs -> macro to generate macro when it is stable

macro_rules! track_paulis {
    ($(($name:ident, $gate:literal),)*) => {$(
        /// Append a tracked
        #[doc = $gate]
        /// gate to the tracker.
        pub fn $name(&mut self, bit: usize) {
            self.tracker.$name(bit)
        }
    )*};
}

macro_rules! apply_paulis {
    ($(($name:ident, $gate:literal),)*) => {$(
        /// Apply the
        #[doc = $gate]
        /// gate on the circuit (identity on the tracker).
        pub fn $name(&mut self, bit: usize) {
            self.circuit.$name(bit)
        }
    )*};
}

macro_rules! single_gate {
    ($(($name:ident, $gate:literal),)*) => {$(
        /// Apply the
        #[doc = $gate]
        /// gate on the circuit and update the Pauli tracker accordingly.
        pub fn $name(&mut self, bit: usize) {
            self.circuit.$name(bit);
            self.tracker.$name(bit)
        }
    )*};
}

macro_rules! movements {
    ($((
        $name:ident,
        $from_doc:literal,
        $to_doc:literal
    ),)*) => {$(
        /// On the tracker, "move" the
        #[doc=$from_doc]
        /// Pauli stack from the `origin` qubit to the `destination` qubit,
        /// transforming it to an
        #[doc=$to_doc]
        /// stack.
        pub fn $name(&mut self, source: usize, destination: usize) {
            self.tracker.$name(source, destination)
        }
    )*};
}

macro_rules! remove {
    ($((
        $name:ident,
        $correction:literal
    ),)*) => {$(
        /// On the tracker, "remove" the
        #[doc=$correction]
        /// Pauli stack from the qu`bit`.
        pub fn $name(&mut self, bit: usize) {
            self.tracker.$name(bit)
        }
    )*};
}

macro_rules! double_gate {
    ($name:ident, $gate:literal) => {
        double_gate!($name, $gate, bit_a, bit_b);
    };
    ($name:ident, $gate:literal, $bit_a:ident, $bit_b:ident) => {
        /// Apply the
        #[doc = $gate]
        /// gate on the circuit and update the Pauli tracker accordingly.
        pub fn $name(&mut self, $bit_a: usize, $bit_b: usize) {
            self.circuit.$name($bit_a, $bit_b);
            self.tracker.$name($bit_a, $bit_b)
        }
    };
}

impl<C, T, S> TrackedCircuit<C, T, S>
where
    T: Tracker,
{
    /// Append a [Pauli](crate::pauli::Pauli) gate `pauli` to the tracker.
    pub fn track_pauli(&mut self, bit: usize, pauli: T::Pauli) {
        self.tracker.track_pauli(bit, pauli)
    }
    /// Append a [PauliString] to the tracker.
    pub fn track_pauli_string(&mut self, pauli: PauliString<T::Pauli>) {
        self.tracker.track_pauli_string(pauli)
    }
    track_paulis!((track_x, "X"), (track_y, "Y"), (track_z, "Z"),);
    movements!(
        (move_x_to_x, "X", "X"),
        (move_x_to_z, "X", "Z"),
        (move_z_to_x, "Z", "X"),
        (move_z_to_z, "Z", "Z"),
    );
    remove!((remove_z, "Z"), (remove_x, "X"),);
}

impl<C, T, S> TrackedCircuit<C, T, S>
where
    C: CliffordCircuit,
{
    apply_paulis!((id, "I"), (x, "X"), (y, "Y"), (z, "Z"),);

    /// Perform a Measurement on the circuit, returning the result.
    pub fn measure(&mut self, bit: usize) -> C::Outcome {
        self.circuit.measure(bit)
    }
}

impl<C, T, S> TrackedCircuit<C, T, S>
where
    C: CliffordCircuit,
    T: Tracker,
{
    single_gate!(
        (s, "S"),
        (sdg, "SDG"),
        (sz, "SZ"),
        (szdg, "SZDG"),
        (hxy, "H_xy"),
        (h, "H"),
        (sy, "SY"),
        (sydg, "SYDG"),
        (sh, "SH"),
        (hs, "HS"),
        (shs, "SHS"),
        (sx, "SX"),
        (sxdg, "SXDG"),
        (hyz, "H_yz"),
    );

    double_gate!(cz, "Control Z");
    double_gate!(cx, "Control X (Control Not)", control, target);
    double_gate!(cy, "Control Y", control, target);
    double_gate!(swap, "SWAP");
    double_gate!(zcz, "Z-Control Z", control, target);
    double_gate!(zcx, "Z-Control X");
    double_gate!(zcy, "Z-Control Y", control, target);
    double_gate!(iswap, "iSWAP");
    double_gate!(iswapdg, "iSWAP^dagger");
}

impl<C, A, S, B> TrackedCircuit<C, Frames<A>, S>
where
    C: CliffordCircuit,
    A: Full<T = PauliStack<B>> + Default,
    S: Base<TB = PauliStack<B>>,
    B: BooleanVector,
{
    /// Perform a **Measurement** and move the according qubit with its Pauli stack from
    /// the tracker into the additional storage. Returns the measurement outcome and
    /// the result of [Frames::measure_and_store].
    pub fn measure_and_store(
        &mut self,
        bit: usize,
    ) -> (C::Outcome, Result<(), MoveError<B>>) {
        let outcome = self.circuit.measure(bit);
        match self.tracker.measure_and_store(bit, &mut self.storage) {
            Ok(_) => (outcome, Ok(())),
            Err(e) => (outcome, Err(e)),
        }
    }

    /// Measure all remaining qubits and put the according stack of Paulis into the
    /// additional storage, i.e., do [Self::measure_and_store] for all qubits. Return
    /// the measurement outcomes as tuples (qubit, outcome) and errors if we would
    /// overwrite a PauliStack
    #[allow(clippy::type_complexity)] // cos Result is basically two types
    pub fn measure_and_store_all(
        &mut self,
    ) -> (Vec<(usize, C::Outcome)>, Result<(), OverwriteStack<B>>) {
        let mut outcome = Vec::<(usize, C::Outcome)>::new();
        let num_frames = self.tracker.frames_num();
        let mut storage = mem::take(&mut self.tracker).into_storage().into_iter();
        while let Some((bit, pauli)) = storage.next() {
            outcome.push((bit, self.circuit.measure(bit)));
            if let Some(stack) = self.storage.insert(bit, pauli) {
                self.tracker = Frames::new_unchecked(storage.collect(), num_frames);
                return (outcome, Err(OverwriteStack { bit, stack }));
            }
        }
        (outcome, Ok(()))
    }
}

#[cfg(test)]
mod tests {
    // use bitvec::vec::BitVec;
    use bit_vec::BitVec;
    use coverage_helper::test;

    use super::*;
    use crate::{
        boolean_vector::bitvec_simd::SimdBitVec,
        collection::{BufferedVector, Init, Map, MappedVector, NaiveVector},
        pauli::{PauliDense, PauliEnum},
        tracker::{MissingBit, frames::induced_order, live},
    };

    type PauliBitVec = PauliStack<BitVec>;
    type PauliSimdBitVec = PauliStack<SimdBitVec>;
    // type Live<P> = live::Live<BufferedVector<P>>;
    type Live<P> = live::Live<crate::collection::MappedVector<P>>;

    #[test]
    fn measure_and_store() {
        let mut circ = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<MappedVector<PauliStack<BitVec>>>::init(3),
            storage: Map::<_>::default(),
        };

        circ.measure_and_store(0).1.unwrap();
        circ.track_z(2);
        circ.cx(1, 2);
        circ.h(1);
        circ.measure_and_store(2).1.unwrap();
        circ.tracker.new_qubit(2);
        match circ.measure_and_store(2).1.unwrap_err() {
            MoveError::OverwriteStack(e) => {
                assert_eq!(e, OverwriteStack {
                    bit: 2,
                    stack: PauliBitVec::try_from_str("1", "0").unwrap()
                });
            },
            MoveError::MissingBit(_) => panic!("wrong error"),
        }
        match circ.measure_and_store(2).1.unwrap_err() {
            MoveError::OverwriteStack(_) => panic!("wrong error"),
            MoveError::MissingBit(e) => {
                assert_eq!(e, MissingBit(2));
            },
        }
        circ.tracker.new_qubit(2);
        circ.tracker.new_qubit(3);
        circ.tracker.new_qubit(4);
        // note that the iterator of MappedVector is deterministic (that's not clear
        // from the API, but from the source code); without that, the following wouldn't
        // work
        let (outcome, r) = circ.measure_and_store_all();
        assert_eq!(outcome.len(), 2);
        assert_eq!(r.unwrap_err(), {
            OverwriteStack {
                bit: 2,
                stack: PauliBitVec::try_from_str("0", "0").unwrap(),
            }
        });
        let (outcome, r) = circ.measure_and_store_all();
        assert_eq!(outcome.len(), 2);
        r.unwrap()
    }

    #[test]
    fn move_and_remove() {
        let mut circ = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<Map<PauliStack<Vec<bool>>>>::init(3),
            storage: (),
        };
        let mut live = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: live::Live::<NaiveVector<PauliEnum>>::init(3),
            storage: (),
        };

        circ.track_z(0);
        live.track_z(0);
        circ.track_z(1);
        live.track_z(1);
        circ.track_x(1);
        live.track_x(1);
        circ.track_x(2);
        live.track_x(2);
        assert_eq!(
            vec![
                (0, PauliStack::try_from_str("1000", "0000").unwrap()),
                (1, PauliStack::try_from_str("0100", "0010").unwrap()),
                (2, PauliStack::try_from_str("0000", "0001").unwrap())
            ],
            circ.tracker.as_storage().clone().into_sorted_by_key()
        );
        assert_eq!(live.tracker.as_storage().0, [
            PauliEnum::Z,
            PauliEnum::Y,
            PauliEnum::X
        ]);

        circ.move_z_to_x(0, 1);
        live.move_z_to_x(0, 1);
        assert_eq!(
            vec![
                (0, PauliStack::try_from_str("", "0000").unwrap()),
                (1, PauliStack::try_from_str("0100", "1010").unwrap()),
                (2, PauliStack::try_from_str("0000", "0001").unwrap())
            ],
            circ.tracker.as_storage().clone().into_sorted_by_key()
        );
        assert_eq!(live.tracker.as_storage().0, [
            PauliEnum::I,
            PauliEnum::Z,
            PauliEnum::X
        ]);

        circ.remove_x(2);
        live.remove_x(2);
        assert_eq!(
            vec![
                (0, PauliStack::try_from_str("", "0000").unwrap()),
                (1, PauliStack::try_from_str("0100", "1010").unwrap()),
                (2, PauliStack::try_from_str("0000", "").unwrap())
            ],
            circ.tracker.as_storage().clone().into_sorted_by_key()
        );
        assert_eq!(live.tracker.as_storage().0, [
            PauliEnum::I,
            PauliEnum::Z,
            PauliEnum::I
        ]);

        circ.move_x_to_z(1, 2);
        live.move_x_to_z(1, 2);
        assert_eq!(
            vec![
                (0, PauliStack::try_from_str("", "0000").unwrap()),
                (1, PauliStack::try_from_str("0100", "").unwrap()),
                (2, PauliStack::try_from_str("1010", "").unwrap())
            ],
            circ.tracker.as_storage().clone().into_sorted_by_key()
        );
        assert_eq!(live.tracker.as_storage().0, [
            PauliEnum::I,
            PauliEnum::Z,
            PauliEnum::I
        ]);
    }

    #[test]
    fn single_rotation_teleportation() {
        let mut circ = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<MappedVector<PauliStack<BitVec>>>::init(2),
            storage: MappedVector::<_>::default(),
        };

        circ.cz(0, 1);
        circ.measure_and_store(0).1.unwrap();
        // this hadamard corrects the hadamard from the rotation, therefore, we put the
        // tracked_z behind it (it is effectively commuted through the identity)
        circ.h(1);
        circ.track_z(1);

        assert_eq!(
            vec![(1, PauliBitVec::try_from_str("1", "0").unwrap())],
            circ.tracker.into_storage().into_sorted_by_key()
        );
        assert_eq!(vec![(0, PauliBitVec::new())], circ.storage.into_sorted_by_key());
    }

    #[test]
    fn control_v_dagger() {
        let mut circ = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<MappedVector<PauliSimdBitVec>>::init(5),
            storage: MappedVector::<_>::default(),
        };

        circ.cx(0, 2);
        // -----
        circ.measure_and_store(0).1.unwrap();
        //  ----
        circ.track_z(2);
        //  izii
        circ.h(1);
        //  izii
        circ.cx(1, 2);
        //  zzii
        circ.cx(2, 3);
        //  zzii
        circ.measure_and_store(2).1.unwrap();
        //  z ii
        circ.track_z(3);
        //  z ii
        //  i zi
        circ.cx(1, 4);
        //  z ii
        //  i zi
        circ.measure_and_store(1).1.unwrap();
        //    ii
        //    zi
        circ.track_z(4);
        //    ii
        //    zi
        //    iz
        circ.cx(4, 3);
        //    ii
        //    zz
        //    iz
        circ.h(4);
        //    ii
        //    zx
        //    ix

        assert_eq!(
            vec![
                (3, PauliSimdBitVec::try_from_str("010", "000").unwrap()),
                (4, PauliSimdBitVec::try_from_str("000", "011").unwrap())
            ],
            circ.tracker.into_storage().into_sorted_by_key()
        );

        assert_eq!(
            vec![
                (0, PauliSimdBitVec::new()),
                (1, PauliSimdBitVec::try_from_str("10", "00").unwrap()),
                (2, PauliSimdBitVec::try_from_str("1", "0").unwrap())
            ],
            circ.storage.into_sorted_by_key()
        );
    }

    #[test]
    fn toffoli_live() {
        let mut circ = TrackedCircuit {
            circuit: RandomMeasurementCircuit {},
            tracker: Live::init(10),
            storage: (),
        };

        trait TTele {
            fn t_tele(&mut self, origin: usize, new: usize) -> bool;
        }
        impl TTele for TrackedCircuit<RandomMeasurementCircuit, Live<PauliDense>, ()> {
            #[cfg_attr(coverage_nightly, coverage(off))]
            fn t_tele(&mut self, origin: usize, new: usize) -> bool {
                self.cx(origin, new);
                self.move_z_to_z(origin, new);
                let result = self.circuit.measure(origin);
                if result {
                    self.track_z(new);
                };
                result
            }
        }

        let mut results = Vec::new();

        results.push(circ.t_tele(0, 3) as u8);
        results.push(circ.t_tele(1, 4) as u8);
        circ.h(2);
        circ.cx(3, 4);
        results.push(circ.t_tele(2, 5) as u8);
        circ.cx(4, 5);
        results.push(circ.t_tele(4, 6) as u8);
        results.push(circ.t_tele(5, 7) as u8);
        circ.cx(3, 6);
        circ.cx(6, 7);
        circ.cx(3, 6);
        results.push(circ.t_tele(7, 8) as u8);
        circ.cx(6, 8);
        circ.cx(3, 6);
        results.push(circ.t_tele(8, 9) as u8);
        circ.cx(6, 9);
        circ.h(9);

        let mut check = Live::<PauliDense>::init(10);
        // compare toffoli tests with frame tracker
        // (3, PauliVec::try_from("0000000", "1001110").unwrap()),
        // (6, PauliVec::try_from("0000000", "0101101").unwrap()),
        // (9, PauliVec::try_from("0010111", "0000000").unwrap()),
        check
            .get_mut(3)
            .unwrap()
            .set_storage((results[0] + results[3] + results[4] + results[5]) % 2);
        check
            .get_mut(6)
            .unwrap()
            .set_storage((results[1] + results[3] + results[4] + results[6]) % 2);
        check
            .get_mut(9)
            .unwrap()
            .set_storage(((results[2] + results[4] + results[5] + results[6]) % 2) * 2);

        // println!("{:?}", circ.tracker);
        assert_eq!(circ.tracker, check);
    }

    #[test]
    fn another_graph_test() {
        let mut circ = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<BufferedVector<PauliStack<BitVec>>>::init(10),
            storage: (),
        };

        // wrapping this impl into a trait makes it local to that function (normal impl
        // blocks have the same scope as the type)
        trait TTele {
            fn t_tele(&mut self, origin: usize, new: usize);
        }
        impl TTele
            for TrackedCircuit<
                DummyCircuit,
                Frames<BufferedVector<PauliStack<BitVec>>>,
                (),
            >
        {
            #[cfg_attr(coverage_nightly, coverage(off))]
            fn t_tele(&mut self, origin: usize, new: usize) {
                self.cx(origin, new);
                self.move_z_to_z(origin, new);
                self.measure(origin);
                self.track_z(new);
            }
        }

        circ.t_tele(0, 3);
        circ.t_tele(1, 4);
        circ.h(4);
        circ.cz(4, 3);
        circ.cx(3, 4);
        circ.t_tele(2, 5);
        circ.cx(4, 5);
        circ.t_tele(4, 6);
        circ.h(6);
        circ.t_tele(5, 7);
        circ.cx(3, 6);
        circ.h(3);
        circ.cx(3, 7);
        circ.s(3);
        circ.cz(3, 6);
        circ.t_tele(7, 8);
        circ.cx(6, 8);
        circ.s(8);
        circ.cx(3, 6);
        circ.cx(8, 6);
        circ.t_tele(8, 9);
        circ.cx(6, 9);
        circ.h(9);

        let rest = circ.tracker.into_storage();

        // println!("{:#?}", rest);

        let _graph = induced_order::get_order(&rest, &[0, 1, 2, 4, 5, 7, 8]);
        // println!("{:?}", graph);
        // println!("{:?}", graph.len());
    }
}
