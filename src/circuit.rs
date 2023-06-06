//! A circuit wrapper around a Clifford circuit and a [Tracker].
//!
//! The main content of this module is a wrapper [TrackedCircuit] that provides an
//! methods to track Paulis while building up the circuit or executing it. One can
//! either use it directly by providing an appropriate circuit simulator (should
//! implement [CliffordCircuit]) or use it as template/idea to write a custom wrapper.
//! The module also provides two pseudo circuit simulators that can be used to test the
//! Pauli tracking.

use crate::{
    pauli::Pauli,
    tracker::{
        frames::{
            storage::StackStorage,
            Frames,
        },
        Tracker,
    },
};

/// The interface into a circuit that can handle Clifford gates and (unspecified)
/// measurements. We don't really care what the circuit is actually doing, except for
/// possible measurement outcomes, since we only use this interface to pass the actions
/// through to the implementing circuit.
pub trait CliffordCircuit {
    type Outcome;
    /// Apply the **X** gate
    fn x(&mut self, bit: usize);
    /// Apply the **Y** gate
    fn y(&mut self, bit: usize);
    /// Apply the **Z** gate
    fn z(&mut self, bit: usize);
    /// Apply the **H** gate
    fn h(&mut self, bit: usize);
    /// Apply the **S** gate
    fn s(&mut self, bit: usize);
    /// Apply the **Control X (Control Not)** gate
    fn cx(&mut self, control: usize, target: usize);
    /// Apply the **Control Z** gate
    fn cz(&mut self, bit_a: usize, bit_b: usize);
    /// **Measure** (unspecified)
    fn measure(&mut self, bit: usize) -> Self::Outcome;
}

mod dummy;
pub use dummy::DummyCircuit;
mod random_measurement;
pub use random_measurement::RandomMeasurementCircuit;

/// A Wrapper around a Clifford circuit (simulator) and a Pauli tracker.
///
/// The type can
/// be used to build up the underlining circuit, while keeping track of the Pauli gates
/// that shall be extracted from the (quantum) simulation, e.g., the Pauli corrections
/// in [MBQC](https://doi.org/10.48550/arXiv.0910.1116).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TrackedCircuit<Circuit, Tracker, Storage> {
    /// The underlining circuit (simulator). Should implement [CliffordCircuit]
    pub circuit: Circuit,
    /// The tracker of the Pauli frames. The `ActiveStorage` should implement
    /// [StackStorage].
    pub tracker: Tracker,
    /// An additional storage which the [StackStorage]s of the measure qubits. Should
    /// implement [StackStorage].
    pub storage: Storage,
}

// split impl into multiple blocks with the minimum required bounds, so that it is
// simpler to write generic functions later on

impl<C, T, S> TrackedCircuit<C, T, S>
where
    T: Tracker,
{
    // Safety: storage < 4 is hardcoded
    /// Append a tracked X gate to the tracker.
    pub fn track_x(&mut self, bit: usize) {
        self.tracker.track_pauli(bit, unsafe { Pauli::from_unchecked(2) });
    }
    /// Append a tracked Y gate to the tracker.
    pub fn track_y(&mut self, bit: usize) {
        self.tracker.track_pauli(bit, unsafe { Pauli::from_unchecked(3) });
    }
    /// Append a tracked Z gate to the tracker.
    pub fn track_z(&mut self, bit: usize) {
        self.tracker.track_pauli(bit, unsafe { Pauli::from_unchecked(1) });
    }

    /// Call [Tracker::move_x_to_x] on the underlining tracker
    pub fn move_x_to_x(&mut self, source: usize, destination: usize) {
        self.tracker.move_x_to_x(source, destination);
    }
    /// Call [Tracker::move_x_to_z] on the underlining tracker
    pub fn move_x_to_z(&mut self, source: usize, destination: usize) {
        self.tracker.move_x_to_z(source, destination);
    }
    /// Call [Tracker::move_z_to_x] on the underlining tracker
    pub fn move_z_to_x(&mut self, source: usize, destination: usize) {
        self.tracker.move_z_to_x(source, destination);
    }
    /// Call [Tracker::move_z_to_z] on the underlining tracker
    pub fn move_z_to_z(&mut self, source: usize, destination: usize) {
        self.tracker.move_z_to_z(source, destination);
    }
}

impl<C, T, S> TrackedCircuit<C, T, S>
where
    C: CliffordCircuit,
{
    /// Apply the **X** gate on the circuit (identity on the tracker).
    pub fn x(&mut self, bit: usize) {
        self.circuit.x(bit);
    }
    /// Apply the **Y** gate on the circuit (identity on the tracker).
    pub fn y(&mut self, bit: usize) {
        self.circuit.x(bit);
    }
    /// Apply the **Z** gate on the circuit (identity on the tracker).
    pub fn z(&mut self, bit: usize) {
        self.circuit.x(bit);
    }

    /// Perform a **Measurement** on the circuit, returning the result.
    pub fn measure(&mut self, bit: usize) -> C::Outcome {
        self.circuit.measure(bit)
    }
}

macro_rules! single {
    ($name:ident) => {
        pub fn $name(&mut self, bit: usize) {
            self.circuit.$name(bit);
            self.tracker.$name(bit);
        }
    };
}

macro_rules! double {
    ($name:ident) => {
        pub fn $name(&mut self, bit_a: usize, bit_b: usize) {
            self.circuit.$name(bit_a, bit_b);
            self.tracker.$name(bit_a, bit_b);
        }
    };
    ($name:ident, $bit_a:ident, $bit_b:ident) => {
        pub fn $name(&mut self, $bit_a: usize, $bit_b: usize) {
            self.circuit.$name($bit_a, $bit_b);
            self.tracker.$name($bit_a, $bit_b);
        }
    };
}

impl<C, T, S> TrackedCircuit<C, T, S>
where
    C: CliffordCircuit,
    T: Tracker,
{
    /// Apply the **H** gate on the circuit and update the Pauli tracker accordingly.
    single!(h);
    /// Apply the **S** gate on the circuit and update the Pauli tracker accordingly.
    single!(s);

    /// Apply the **Control X (Control Not)** gate on the circuit and update the Pauli
    /// tracker accordingly.
    double!(cx, control, target);
    /// Apply the **Control Z** gate on the circuit and update the Pauli tracker
    /// accordingly.
    double!(cz);
}

impl<C, A, S> TrackedCircuit<C, Frames<A>, S>
where
    C: CliffordCircuit,
    A: StackStorage,
    S: StackStorage,
{
    /// Perform a **Measurement** and move the according qubit from the tracker into the
    /// additional storage.
    pub fn measure_and_store(&mut self, bit: usize) -> Result<(), String> {
        self.circuit.measure(bit);
        self.tracker.measure_and_store(bit, &mut self.storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        circuit::{
            DummyCircuit,
            RandomMeasurementCircuit,
        },
        tracker::{
            frames::{
                storage::{
                    self,
                    MappedVector,
                    PauliVec,
                    Vector,
                },
                Frames,
            },
            live::LiveVector,
        },
    };

    #[test]
    fn single_rotation_teleportation() {
        let mut circ = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<MappedVector>::init(2),
            storage: MappedVector::default(),
        };

        circ.cz(0, 1);
        circ.measure_and_store(0);
        // this hadamard corrects the hadamard from the rotation, therefore, we put the
        // tracked_z behind it (it is effectively commuted through the identity)
        circ.h(1);
        circ.track_z(1);

        assert_eq!(
            vec![(1, PauliVec::try_from_str("0", "1").unwrap())],
            storage::into_sorted_by_bit(circ.tracker.into_storage())
        );
        assert_eq!(
            vec![(0, PauliVec::new())],
            storage::into_sorted_by_bit(circ.storage)
        );
    }

    #[test]
    fn control_v_dagger() {
        let mut circ = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<MappedVector>::init(5),
            storage: MappedVector::default(),
        };

        circ.cx(0, 2);
        circ.measure_and_store(0);
        circ.track_z(2);
        circ.h(1);
        circ.cx(1, 2);
        circ.cx(2, 3);
        circ.measure_and_store(2);
        circ.track_z(3);
        circ.cx(1, 4);
        circ.measure_and_store(1);
        circ.track_z(4);
        circ.cx(4, 3);
        circ.h(4);

        assert_eq!(
            vec![
                (3, PauliVec::try_from_str("000", "010").unwrap()),
                (4, PauliVec::try_from_str("011", "000").unwrap())
            ],
            storage::into_sorted_by_bit(circ.tracker.into_storage())
        );
        assert_eq!(
            vec![
                (0, PauliVec::new()),
                (1, PauliVec::try_from_str("00", "10").unwrap()),
                (2, PauliVec::try_from_str("0", "1").unwrap())
            ],
            storage::into_sorted_by_bit(circ.storage)
        );
    }

    #[test]
    fn toffoli_time_dependent() {
        let mut circ = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<MappedVector>::init(10),
            storage: MappedVector::default(),
        };

        // wrapping this impl into a trait makes it local to that function (normal impl
        // blocks have the same scope as the type)
        trait TTele {
            fn t_tele(&mut self, origin: usize, new: usize);
        }
        impl TTele for TrackedCircuit<DummyCircuit, Frames<MappedVector>, MappedVector> {
            fn t_tele(&mut self, origin: usize, new: usize) {
                self.cx(origin, new);
                self.measure_and_store(origin);
                self.track_z(new);
            }
        }

        circ.t_tele(0, 3);
        circ.t_tele(1, 4);
        circ.h(2);
        circ.cx(3, 4);
        circ.t_tele(2, 5);
        circ.cx(4, 5);
        circ.t_tele(4, 6);
        circ.t_tele(5, 7);
        circ.cx(3, 6);
        circ.cx(6, 7);
        circ.cx(3, 6);
        circ.t_tele(7, 8);
        circ.cx(6, 8);
        circ.cx(3, 6);
        circ.t_tele(8, 9);
        circ.cx(6, 9);
        circ.h(9);

        assert_eq!(
            vec![
                (3, PauliVec::try_from_str("0000000", "1101010").unwrap()),
                (6, PauliVec::try_from_str("0000000", "0001111").unwrap()),
                (9, PauliVec::try_from_str("0000001", "0000000").unwrap()),
            ],
            storage::into_sorted_by_bit(circ.tracker.into_storage())
        );
        assert_eq!(
            vec![
                (0, PauliVec::try_from_str("", "").unwrap()),
                (1, PauliVec::try_from_str("0", "0").unwrap()),
                (2, PauliVec::try_from_str("00", "00").unwrap()),
                (4, PauliVec::try_from_str("000", "011").unwrap()),
                (5, PauliVec::try_from_str("0000", "0010").unwrap()),
                (7, PauliVec::try_from_str("00000", "00001").unwrap()),
                (8, PauliVec::try_from_str("000000", "000001").unwrap())
            ],
            storage::into_sorted_by_bit(circ.storage)
        );
    }

    #[test]
    fn toffoli_time_independent() {
        let mut circ = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<MappedVector>::init(10),
            storage: MappedVector::default(),
        };

        trait TTele {
            fn t_tele(&mut self, origin: usize, new: usize);
        }
        impl TTele for TrackedCircuit<DummyCircuit, Frames<MappedVector>, MappedVector> {
            fn t_tele(&mut self, origin: usize, new: usize) {
                self.cx(origin, new);
                self.move_z_to_z(origin, new);
                self.measure_and_store(origin);
                self.track_z(new);
            }
        }

        circ.t_tele(0, 3);
        circ.t_tele(1, 4);
        circ.h(2);
        circ.cx(3, 4);
        circ.t_tele(2, 5);
        circ.cx(4, 5);
        circ.t_tele(4, 6);
        circ.t_tele(5, 7);
        circ.cx(3, 6);
        circ.cx(6, 7);
        circ.cx(3, 6);
        circ.t_tele(7, 8);
        circ.cx(6, 8);
        circ.cx(3, 6);
        circ.t_tele(8, 9);
        circ.cx(6, 9);
        circ.h(9);

        assert_eq!(
            vec![
                (3, PauliVec::try_from_str("0000000", "1001110").unwrap()),
                (6, PauliVec::try_from_str("0000000", "0101101").unwrap()),
                (9, PauliVec::try_from_str("0010111", "0000000").unwrap()),
            ],
            storage::into_sorted_by_bit(circ.tracker.into_storage())
        );
        assert_eq!(
            vec![
                (0, PauliVec::try_from_str("", "").unwrap()),
                (1, PauliVec::try_from_str("0", "").unwrap()),
                (2, PauliVec::try_from_str("00", "").unwrap()),
                (4, PauliVec::try_from_str("000", "").unwrap()),
                (5, PauliVec::try_from_str("0000", "").unwrap()),
                (7, PauliVec::try_from_str("00000", "").unwrap()),
                (8, PauliVec::try_from_str("000000", "").unwrap())
            ],
            storage::into_sorted_by_bit(circ.storage)
        );
    }

    #[test]
    fn toffoli_live() {
        let mut circ = TrackedCircuit {
            circuit: RandomMeasurementCircuit {},
            tracker: LiveVector::init(10),
            storage: (),
        };

        trait TTele {
            fn t_tele(&mut self, origin: usize, new: usize) -> bool;
        }
        impl TTele for TrackedCircuit<RandomMeasurementCircuit, LiveVector, ()> {
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

        let mut check = LiveVector::init(10);
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
    fn first_graph_test() {
        let mut circ = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<MappedVector>::init(10),
            storage: MappedVector::default(),
        };

        // wrapping this impl into a trait makes it local to that function (normal impl
        // blocks have the same scope as the type)
        trait TTele {
            fn t_tele(&mut self, origin: usize, new: usize);
        }
        impl TTele for TrackedCircuit<DummyCircuit, Frames<MappedVector>, MappedVector> {
            fn t_tele(&mut self, origin: usize, new: usize) {
                self.cx(origin, new);
                self.measure_and_store(origin);
                self.track_z(new);
            }
        }

        circ.t_tele(0, 3);
        circ.t_tele(1, 4);
        circ.h(2);
        circ.cx(3, 4);
        circ.t_tele(2, 5);
        circ.cx(4, 5);
        circ.t_tele(4, 6);
        circ.t_tele(5, 7);
        circ.cx(3, 6);
        circ.cx(6, 7);
        circ.cx(3, 6);
        circ.t_tele(7, 8);
        circ.cx(6, 8);
        circ.cx(3, 6);
        circ.t_tele(8, 9);
        circ.cx(6, 9);
        circ.h(9);
        circ.measure_and_store(9);
        circ.measure_and_store(6);
        circ.measure_and_store(3);

        let graph = crate::tracker::frames::storage::create_dependency_graph(
            &circ.storage,
            &[0, 1, 2, 4, 5, 7, 8],
        );
        // println!("{:?}", graph);
    }

    #[test]
    fn another_graph_test() {
        let mut circ = TrackedCircuit {
            circuit: DummyCircuit {},
            tracker: Frames::<Vector>::init(10),
            storage: (),
        };

        // wrapping this impl into a trait makes it local to that function (normal impl
        // blocks have the same scope as the type)
        trait TTele {
            fn t_tele(&mut self, origin: usize, new: usize);
        }
        impl TTele for TrackedCircuit<DummyCircuit, Frames<Vector>, ()> {
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

        let graph = crate::tracker::frames::storage::create_dependency_graph(
            &rest,
            &[0, 1, 2, 4, 5, 7, 8],
        );
        // println!("{:?}", graph);
        // println!("{:?}", graph.len());
    }
}
