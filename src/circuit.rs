//! An intuitive, canonical description of a circuit consisting only of certain Clifford
//! gates and (unspecified) measurements.

use std::ops::{
    Deref,
    DerefMut,
};

use crate::pauli_frame::{
    Frames,
    Pauli,
    PauliStorageMap,
};

#[derive(Clone, Copy, Debug)]
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
    /// Control Not
    Cnot(
        /// Control
        usize,
        /// Target
        usize,
    ),
    /// Unspecified measurement
    Measure(usize),
}

impl Gate {
    pub fn apply_on_pauli_tracker(
        &self,
        tracker: &mut Frames<impl PauliStorageMap>,
        storage: &mut impl PauliStorageMap,
    ) {
        match *self {
            Gate::X(b) => tracker.x(b),
            Gate::Y(b) => tracker.y(b),
            Gate::Z(b) => tracker.z(b),
            // Safety: storage < 4 is hardcoded
            Gate::TrackedX(b) => tracker.track_pauli(b, unsafe { Pauli::from_raw(2) }),
            Gate::TrackedY(b) => tracker.track_pauli(b, unsafe { Pauli::from_raw(3) }),
            Gate::TrackedZ(b) => tracker.track_pauli(b, unsafe { Pauli::from_raw(1) }),
            Gate::H(b) => tracker.h(b),
            Gate::S(b) => tracker.s(b),
            Gate::Cnot(c, t) => tracker.cnot(c, t),
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

    /// Push [Gate::Cnot]\(`bit`\)
    pub fn cnot(&mut self, control: usize, target: usize) {
        self.gates.push(Gate::Cnot(control, target));
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
    use std::collections::HashMap;

    use super::*;
    use crate::pauli_frame::{
        self,
        storage::{PauliStorage, SmallPauliStorage},
        Frames,
        PauliVec,
    };

    #[test]
    fn test() {
        let num_qubits = 3;
        let mut circ = Circuit::new();
        let mut tracker = Frames::<PauliStorage>::init(num_qubits);
        let mut storage = SmallPauliStorage::new();

        circ.h(0);
        circ.cnot(2, 1);
        circ.tracked_z(1);
        circ.cnot(0, 1);
        circ.measure(1);
        tracker.new_qubit(3);
        circ.tracked_y(2);
        circ.cnot(2, 3);

        circ.iter().for_each(|gate| {
            gate.apply_on_pauli_tracker(&mut tracker, &mut storage)
        });

        println!("{:?}", circ);
        // println!("{:#?}", tracker);
        println!("{:#?}", pauli_frame::sort(&tracker.storage));
        println!("{:#?}", pauli_frame::sort(&storage));
    }
}
