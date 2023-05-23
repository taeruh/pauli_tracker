//! An intuitive, canonical description of a circuit consisting only of certain Clifford
//! gates and (unspecified) measurements.

use std::ops::{
    Deref,
    DerefMut,
};

use crate::pauli_frame::{
    Frames,
    Pauli,
    PauliStorage,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
/// A subset of the Clifford gates + (unspecified) measurements. Each operation stores
/// the qubit position it acts on.
pub enum Gate {
    /// Pauli X
    X(usize),
    /// Pauli Y
    Y(usize),
    /// Pauli Z
    Z(usize),
    /// Pauli X that shall be tracked
    TrackedX(usize),
    /// Pauli Y that shall be tracked
    TrackedY(usize),
    /// Pauli Z that shall be tracked
    TrackedZ(usize),
    /// Hadamard
    H(usize),
    /// Phase
    S(usize),
    /// Control X (Control Not)
    CX(
        /// Control
        usize,
        /// Target
        usize,
    ),
    /// Control Z
    CZ(usize, usize),
    /// Unspecified measurement
    Measure(usize),
}

impl Gate {
    pub fn apply_on_pauli_tracker(
        &self,
        tracker: &mut Frames<impl PauliStorage>,
        storage: &mut impl PauliStorage,
    ) {
        match *self {
            Gate::X(_) => (),
            Gate::Y(_) => (),
            Gate::Z(_) => (),
            // Safety: storage < 4 is hardcoded
            Gate::TrackedX(b) => {
                tracker.track_pauli(b, unsafe { Pauli::from_unchecked(2) })
            }
            Gate::TrackedY(b) => {
                tracker.track_pauli(b, unsafe { Pauli::from_unchecked(3) })
            }
            Gate::TrackedZ(b) => {
                tracker.track_pauli(b, unsafe { Pauli::from_unchecked(1) })
            }
            Gate::H(b) => tracker.h(b),
            Gate::S(b) => tracker.s(b),
            Gate::CX(c, t) => tracker.cx(c, t),
            Gate::CZ(a, b) => tracker.cz(a, b),
            Gate::Measure(b) => tracker.measure_and_store(b, storage),
        }
    }
}

/// A circuit consisting of Clifford gates and measurements.
// it is just a newtype wrapper around a Vec, so it makes sense to implement Deref and
// DerefMut since Vec is a smart pointer
#[derive(Debug, Default)]
pub struct Circuit {
    /// The circuit instructions
    pub gates: Vec<Gate>,
}

impl Circuit {
    /// Create a new empty [Circuit]
    pub fn new() -> Self {
        Self { gates: Vec::new() }
    }

    // convenience methods to build the circuit (could also be down directly since
    // DerefMut is implemented to point to self.gates)

    /// Push [Gate::X]\(`bit`\)
    pub fn x(&mut self, bit: usize) {
        self.gates.push(Gate::X(bit));
    }

    /// Push [Gate::Z]\(`bit`\)
    pub fn z(&mut self, bit: usize) {
        self.gates.push(Gate::Z(bit));
    }

    /// Push [Gate::Y]\(`bit`\)
    pub fn y(&mut self, bit: usize) {
        self.gates.push(Gate::Y(bit));
    }

    /// Push [Gate::TrackedX]\(`bit`\)
    pub fn tracked_x(&mut self, bit: usize) {
        self.gates.push(Gate::TrackedX(bit));
    }

    /// Push [Gate::TrackedZ]\(`bit`\)
    pub fn tracked_z(&mut self, bit: usize) {
        self.gates.push(Gate::TrackedZ(bit));
    }

    /// Push [Gate::TrackedY]\(`bit`\)
    pub fn tracked_y(&mut self, bit: usize) {
        self.gates.push(Gate::TrackedY(bit));
    }

    /// Push [Gate::H]\(`bit`\)
    pub fn h(&mut self, bit: usize) {
        self.gates.push(Gate::H(bit));
    }

    /// Push [Gate::S]\(`bit`\)
    pub fn s(&mut self, bit: usize) {
        self.gates.push(Gate::S(bit));
    }

    /// Push [Gate::CX]\(`control`, `target`\)
    pub fn cx(&mut self, control: usize, target: usize) {
        self.gates.push(Gate::CX(control, target));
    }

    /// Push [Gate::CX]\(`bit_a`, `bit_b`\)
    pub fn cz(&mut self, bit_a: usize, bit_b: usize) {
        self.gates.push(Gate::CX(bit_a, bit_b));
    }

    /// Push [Gate::Measure]\(`bit`\)
    pub fn measure(&mut self, bit: usize) {
        self.gates.push(Gate::Measure(bit));
    }
}

impl Deref for Circuit {
    type Target = Vec<Gate>;
    fn deref(&self) -> &Self::Target {
        &self.gates
    }
}

impl DerefMut for Circuit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gates
    }
}

// TODO finish; maybe with some derive macros to reduce some boilerplate
// pub mod dense;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pauli_frame::{
        self,
        storage::{
            FullMap,
            MappedVector,
        },
        Frames,
        PauliVec,
    };

    #[test]
    fn single_rotation_teleportation() {
        let mut circ = Circuit::new();
        let mut tracker = Frames::<MappedVector>::init(2);
        let mut storage = FullMap::new();

        circ.cz(0, 1);
        circ.measure(0);
        // this hadamard corrects the hadamard from the rotation, therefore, we put the
        // tracked_z behind it (it is effectively commuted through the identity)
        circ.h(1);
        circ.tracked_z(1);

        for gate in circ.iter() {
            gate.apply_on_pauli_tracker(&mut tracker, &mut storage);
        }

        assert_eq!(
            vec![(1, PauliVec::try_from(("0", "1")).unwrap())],
            pauli_frame::into_sorted_pauli_storage(tracker.into_storage())
        );
        assert_eq!(
            vec![(0, PauliVec::new())],
            pauli_frame::into_sorted_pauli_storage(storage)
        );
    }

    #[test]
    fn control_v_dagger() {
        let mut circ = Circuit::new();
        let mut tracker = Frames::<MappedVector>::init(5);
        let mut storage = FullMap::new();

        circ.cx(0, 2);
        circ.measure(0);
        circ.tracked_z(2);
        circ.h(1);
        circ.cx(1, 2);
        circ.cx(2, 3);
        circ.measure(2);
        circ.tracked_z(3);
        circ.cx(1, 4);
        circ.measure(1);
        circ.tracked_z(4);
        circ.cx(4, 3);
        circ.h(4);

        for gate in circ.iter() {
            gate.apply_on_pauli_tracker(&mut tracker, &mut storage);
        }

        assert_eq!(
            vec![
                (3, PauliVec::try_from(("000", "010")).unwrap()),
                (4, PauliVec::try_from(("011", "000")).unwrap())
            ],
            pauli_frame::into_sorted_pauli_storage(tracker.into_storage())
        );
        assert_eq!(
            vec![
                (0, PauliVec::new()),
                (1, PauliVec::try_from(("00", "10")).unwrap()),
                (2, PauliVec::try_from(("0", "1")).unwrap())
            ],
            pauli_frame::into_sorted_pauli_storage(storage)
        );
    }

    #[test]
    fn toffoli() {
        let mut circ = Circuit::new();
        let mut tracker = Frames::<MappedVector>::init(10);
        let mut storage = FullMap::new();

        impl Circuit {
            fn t_tele(&mut self, origin: usize, new: usize) {
                self.cx(origin, new);
                self.measure(origin);
                self.tracked_z(new);
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

        for gate in circ.iter() {
            gate.apply_on_pauli_tracker(&mut tracker, &mut storage);
        }

        assert_eq!(
            vec![
                (3, PauliVec::try_from(("0000000", "1101010")).unwrap()),
                (6, PauliVec::try_from(("0000000", "0001111")).unwrap()),
                (9, PauliVec::try_from(("0000001", "0000000")).unwrap()),
            ],
            pauli_frame::into_sorted_pauli_storage(tracker.into_storage())
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
            pauli_frame::into_sorted_pauli_storage(storage)
        );
    }
}
