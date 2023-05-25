//! The main content of this module is a wrapper [TrackedCircuit], around a Clifford
//! circuit simulator, that provides tools to track Pauli frames while building up the
//! circuit. One can either use it directly by providing an appropriate circuit
//! simulator or us as template/idea to write a custom wrapper.

use crate::pauli_frame::{
    Frames,
    Pauli,
    PauliStorage,
};

/// The interface into a circuit that can handle Clifford gates and (unspecified)
/// measurements. We don't care what the circuit is actually doing, we only use this
/// interface to pass the actions through to the implementing circuit.
pub trait CliffordCircuit {
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
    fn measure(&mut self, bit: usize);
}

/// A dummy Clifford circuit that does nothing.
pub struct DummyCircuit {}
impl CliffordCircuit for DummyCircuit {
    #[inline(always)]
    fn x(&mut self, _: usize) {}
    #[inline(always)]
    fn y(&mut self, _: usize) {}
    #[inline(always)]
    fn z(&mut self, _: usize) {}
    #[inline(always)]
    fn h(&mut self, _: usize) {}
    #[inline(always)]
    fn s(&mut self, _: usize) {}
    #[inline(always)]
    fn cx(&mut self, _: usize, _: usize) {}
    #[inline(always)]
    fn cz(&mut self, _: usize, _: usize) {}
    #[inline(always)]
    fn measure(&mut self, _: usize) {}
}

pub mod simple;

/// A Combination of a Clifford circuit (simulator) with a Pauli tracker. The type can
/// be used to build up the underlining circuit, while keeping track of the Pauli gates
/// that shall be extracted from the (quantum) simulation, e.g., the Pauli corrections
/// in [MBQC](https://doi.org/10.48550/arXiv.0910.1116).
pub struct TrackedCircuit<Circuit, ActiveStorage, Storage> {
    /// The underlining circuit (simulator). Should implement [CliffordCircuit]
    pub circuit: Circuit,
    /// The tracker of the Pauli frames. The `ActiveStorage` should implement
    /// [PauliStorage].
    pub tracker: Frames<ActiveStorage>,
    /// An additional storage which the [PauliVec](crate::pauli_frame::PauliVec)s of
    /// the measure qubits. Should implement [PauliStorage].
    pub storage: Storage,
}

// split impl into multiple blocks with the minimum required bounds, so that it is
// simpler to write generic functions later on

impl<C, A, S> TrackedCircuit<C, A, S>
where
    A: PauliStorage,
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

    /// In the Pauli tracker, move the **Z corrections** from the `source` qubit to the
    /// `destination` qubit, transforming them to **X corrections**.
    pub fn move_z_to_x(&mut self, source: usize, destination: usize) {
        self.tracker.move_z_to_x(source, destination);
    }

    /// In the Pauli tracker, move the **Z corrections** from the `source` qubit to the
    /// `destination` qubit.
    pub fn move_z_to_z(&mut self, source: usize, destination: usize) {
        self.tracker.move_z_to_z(source, destination);
    }
}

impl<C, A, S> TrackedCircuit<C, A, S>
where
    C: CliffordCircuit,
{
    /// Apply the **X** gate on the circuit.
    pub fn x(&mut self, bit: usize) {
        self.circuit.x(bit);
    }
    /// Apply the **Y** gate on the circuit.
    pub fn y(&mut self, bit: usize) {
        self.circuit.x(bit);
    }
    /// Apply the **Z** gate on the circuit.
    pub fn z(&mut self, bit: usize) {
        self.circuit.x(bit);
    }
}

impl<C, A, S> TrackedCircuit<C, A, S>
where
    C: CliffordCircuit,
    A: PauliStorage,
{
    /// Apply the **H** gate on the circuit and update the Pauli tracker.
    pub fn h(&mut self, bit: usize) {
        self.circuit.h(bit);
        self.tracker.h(bit);
    }
    /// Apply the **S** gate on the circuit and update the Pauli tracker.
    pub fn s(&mut self, bit: usize) {
        self.circuit.s(bit);
        self.tracker.s(bit);
    }

    /// Apply the **Control X (Control Not)** gate on the circuit and update the Pauli
    /// tracker.
    pub fn cx(&mut self, control: usize, target: usize) {
        self.circuit.cx(control, target);
        self.tracker.cx(control, target);
    }
    /// Apply the **Control Z** gate on the circuit and update the Pauli tracker.
    pub fn cz(&mut self, control: usize, target: usize) {
        self.circuit.cz(control, target);
        self.tracker.cz(control, target);
    }
}

impl<C, A, S> TrackedCircuit<C, A, S>
where
    C: CliffordCircuit,
    A: PauliStorage,
    S: PauliStorage,
{
    /// Perform a **Measurement** and move the according qubit from the tracker into the
    /// additional storage.
    pub fn measure(&mut self, bit: usize) {
        self.circuit.measure(bit);
        self.tracker.measure_and_store(bit, &mut self.storage);
    }
}

// TODO finish; maybe with some derive macros to reduce some boilerplate
// pub mod dense;

#[cfg(test)]
mod tests {

    use super::{
        simple::SimpleCircuit,
        *,
    };
    use crate::pauli_frame::{
        self,
        storage::MappedVector,
        Frames,
        PauliVec,
    };

    #[test]
    fn single_rotation_teleportation() {
        let mut circ = TrackedCircuit {
            circuit: SimpleCircuit::new(),
            tracker: Frames::<MappedVector>::init(2),
            storage: MappedVector::default(),
        };

        circ.cz(0, 1);
        circ.measure(0);
        // this hadamard corrects the hadamard from the rotation, therefore, we put the
        // tracked_z behind it (it is effectively commuted through the identity)
        circ.h(1);
        circ.track_z(1);

        assert_eq!(
            vec![(1, PauliVec::try_from(("0", "1")).unwrap())],
            pauli_frame::into_sorted_pauli_storage(circ.tracker.into_storage())
        );
        assert_eq!(
            vec![(0, PauliVec::new())],
            pauli_frame::into_sorted_pauli_storage(circ.storage)
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
        circ.measure(0);
        circ.track_z(2);
        circ.h(1);
        circ.cx(1, 2);
        circ.cx(2, 3);
        circ.measure(2);
        circ.track_z(3);
        circ.cx(1, 4);
        circ.measure(1);
        circ.track_z(4);
        circ.cx(4, 3);
        circ.h(4);

        assert_eq!(
            vec![
                (3, PauliVec::try_from(("000", "010")).unwrap()),
                (4, PauliVec::try_from(("011", "000")).unwrap())
            ],
            pauli_frame::into_sorted_pauli_storage(circ.tracker.into_storage())
        );
        assert_eq!(
            vec![
                (0, PauliVec::new()),
                (1, PauliVec::try_from(("00", "10")).unwrap()),
                (2, PauliVec::try_from(("0", "1")).unwrap())
            ],
            pauli_frame::into_sorted_pauli_storage(circ.storage)
        );
    }

    #[test]
    fn toffoli_time_dependent() {
        let mut circ = TrackedCircuit {
            circuit: SimpleCircuit::new(),
            tracker: Frames::<MappedVector>::init(10),
            storage: MappedVector::default(),
        };

        // wrapping this impl into a trait makes it local to that function (normal impl
        // blocks have the same scope as the type)
        trait TTele {
            fn t_tele(&mut self, origin: usize, new: usize);
        }
        impl TTele for TrackedCircuit<SimpleCircuit, MappedVector, MappedVector> {
            fn t_tele(&mut self, origin: usize, new: usize) {
                self.cx(origin, new);
                self.measure(origin);
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
                (3, PauliVec::try_from(("0000000", "1101010")).unwrap()),
                (6, PauliVec::try_from(("0000000", "0001111")).unwrap()),
                (9, PauliVec::try_from(("0000001", "0000000")).unwrap()),
            ],
            pauli_frame::into_sorted_pauli_storage(circ.tracker.into_storage())
        );
        assert_eq!(
            vec![
                (0, PauliVec::try_from(("", "")).unwrap()),
                (1, PauliVec::try_from(("0", "0")).unwrap()),
                (2, PauliVec::try_from(("00", "00")).unwrap()),
                (4, PauliVec::try_from(("000", "011")).unwrap()),
                (5, PauliVec::try_from(("0000", "0010")).unwrap()),
                (7, PauliVec::try_from(("00000", "00001")).unwrap()),
                (8, PauliVec::try_from(("000000", "000001")).unwrap())
            ],
            pauli_frame::into_sorted_pauli_storage(circ.storage)
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
        impl TTele for TrackedCircuit<DummyCircuit, MappedVector, MappedVector> {
            fn t_tele(&mut self, origin: usize, new: usize) {
                self.cx(origin, new);
                self.move_z_to_z(origin, new);
                self.measure(origin);
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
                (3, PauliVec::try_from(("0000000", "1001110")).unwrap()),
                (6, PauliVec::try_from(("0000000", "0101101")).unwrap()),
                (9, PauliVec::try_from(("0010111", "0000000")).unwrap()),
            ],
            pauli_frame::into_sorted_pauli_storage(circ.tracker.into_storage())
        );
        assert_eq!(
            vec![
                (0, PauliVec::try_from(("", "")).unwrap()),
                (1, PauliVec::try_from(("0", "")).unwrap()),
                (2, PauliVec::try_from(("00", "")).unwrap()),
                (4, PauliVec::try_from(("000", "")).unwrap()),
                (5, PauliVec::try_from(("0000", "")).unwrap()),
                (7, PauliVec::try_from(("00000", "")).unwrap()),
                (8, PauliVec::try_from(("000000", "")).unwrap())
            ],
            pauli_frame::into_sorted_pauli_storage(circ.storage)
        );
    }
}
