use std::mem;

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use self::storage::{
    PauliVec,
    StackStorage,
};
use super::{
    PauliString,
    Tracker,
};
use crate::pauli::Pauli;

pub mod storage;

/// A container of multiple Pauli frames, using a generic `Storage` type (that
/// implements [StackStorage] if it shall be useful) as internal storage. The type
/// implements the core functionality to track the Pauli frames through a Clifford
/// circuit. The explicit storage type should have the [PauliVec]s on it's minor axis
/// (this is more or less enforced by [StackStorage]). The module [storage] provides
/// some compatible storage types.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Frames<Storage /* : StackStorage */> {
    storage: Storage,
    frames_num: usize,
}

impl<Storage> Frames<Storage> {
    /// Get the underlining storage.
    pub fn as_storage(&self) -> &Storage {
        &self.storage
    }

    /// Get the number of tracked frames.
    pub fn frames_num(&self) -> usize {
        self.frames_num
    }

    /// Convert the object into the underlining storage.
    pub fn into_storage(self) -> Storage {
        self.storage
    }

    // Pauli gates don't do anything; we just include them for completeness and because
    // it might be more convenient to have them on the caller side
    /// Apply Pauli X, note that it is just the identity.
    #[inline(always)]
    pub fn x(&self, _: usize) {}
    /// Apply Pauli Z, note that it is just the identity.
    #[inline(always)]
    pub fn z(&self, _: usize) {}
    /// Apply Pauli Y, note that it is just the identity.
    #[inline(always)]
    pub fn y(&self, _: usize) {}
}

impl<Storage: StackStorage> Frames<Storage> {
    /// Pop the last tracked Pauli frame.
    pub fn pop_frame(&mut self) -> Option<PauliString> {
        if self.storage.is_empty() || self.frames_num == 0 {
            return None;
        }
        let mut ret = Vec::new();
        for (i, p) in self.storage.iter_mut() {
            ret.push((i, p.pop_or_false()));
        }
        self.frames_num -= 1;
        Some(ret)
    }

    /// Measure a qu`bit` and store the according stack of tracked Paulis into
    /// `storage`. Errors when the qu`bit` is not present in the tracker.
    pub fn measure_and_store(
        &mut self,
        bit: usize,
        storage: &mut impl StackStorage,
    ) -> Result<(), String> {
        storage.insert_pauli(
            bit,
            match self.measure(bit) {
                Some(p) => p,
                None => return Err(format!("{bit} is not present")),
            },
        );
        Ok(())
    }

    /// Measure all qubits and put the according stack of Paulis into `storage`, i.e.,
    /// do [Frames::measure_and_store] for all qubits.
    pub fn measure_and_store_all(&mut self, storage: &mut impl StackStorage) {
        for (bit, pauli) in
            mem::replace(&mut self.storage, Storage::init(0)).into_iter()
        {
            storage.insert_pauli(bit, pauli);
        }
    }
}

macro_rules! movements {
    ($(($name:ident, $from_side:ident, $to_side:ident)),*) => {$(
        fn $name(&mut self, source: usize, destination: usize) {
            let (s, d) = self.storage
            .get_two_mut(source, destination)
            .unwrap_or_else(|| panic!(
                    "qubit {source} and/or {destination} do not exist"));
            d.$to_side.xor(&s.$from_side);
            s.$from_side.truncate(0)
        }
    )*}
}

macro_rules! single {
    ($($name:ident),*) => {$(
        fn $name(&mut self, bit: usize) {
            self.storage
                .get_mut(bit)
                .unwrap_or_else(|| panic!("qubit {bit} does not exist"))
                .$name();
        }
    )*};
}

impl<Storage: StackStorage> Tracker for Frames<Storage> {
    type Stack = PauliVec;

    fn init(num_qubits: usize) -> Self {
        Self {
            storage: Storage::init(num_qubits),
            frames_num: 0,
        }
    }

    fn new_qubit(&mut self, qubit: usize) -> Option<usize> {
        self.storage
            .insert_pauli(qubit, PauliVec::zeros(self.frames_num))
            .map(|_| qubit)
    }

    fn track_pauli(&mut self, qubit: usize, pauli: Pauli) {
        if self.storage.is_empty() {
            return;
        }
        for (i, p) in self.storage.iter_mut() {
            if i == qubit {
                p.push(pauli);
            } else {
                p.push(unsafe { Pauli::from_unchecked(0) });
            }
        }
        self.frames_num += 1;
    }

    fn track_pauli_string(&mut self, string: PauliString) {
        if self.storage.is_empty() {
            return;
        }
        for (_, p) in self.storage.iter_mut() {
            p.push(unsafe { Pauli::from_unchecked(0) });
        }
        for (i, p) in string {
            match self.storage.get_mut(i) {
                Some(pauli) => {
                    pauli.left.set(self.frames_num, p.get_x());
                    pauli.right.set(self.frames_num, p.get_z());
                }
                None => continue,
            }
        }
        self.frames_num += 1;
    }

    single!(h, s);

    fn cx(&mut self, control: usize, target: usize) {
        let (c, t) = self
            .storage
            .get_two_mut(control, target)
            .unwrap_or_else(|| panic!("qubit {control} and/or {target} do not exist"));
        t.left.xor(&c.left);
        c.right.xor(&t.right);
    }

    fn cz(&mut self, bit_a: usize, bit_b: usize) {
        let (a, b) = self
            .storage
            .get_two_mut(bit_a, bit_b)
            .unwrap_or_else(|| panic!("qubit {bit_a} and/or {bit_b} do not exist"));
        a.right.xor(&b.left);
        b.right.xor(&a.left);
    }

    // todo: test movements similar to the gates
    movements!(
        (move_x_to_x, left, left),
        (move_x_to_z, left, right),
        (move_z_to_x, right, left),
        (move_z_to_z, right, right)
    );

    fn measure(&mut self, bit: usize) -> Option<PauliVec> {
        self.storage.remove_pauli(bit)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        storage::*,
        *,
    };

    // we only check the basic functionality here, more complicated circuits are tested
    // in [super::circuit] to test the tracker and the circuit at once

    mod gate_definition_check {
        use super::*;

        // maybe todo: in the following functions there's a pattern behind how we encode
        // one-qubit and two-qubit actions, it's like a "TwoBitVec"; one could probably
        // implement that in connection with [Pauli]

        #[test]
        fn one_qubit() {
            // pauli p = ab in binary; encoding: x = a, z = b
            type Action = fn(&mut Frames<Vector>, usize);
            const GATES: [(
                // action
                Action,
                // name for debugging
                &str,
                // result: calculated by hand
                // encoded input: p = 0 1 2 3
                [u8; 4],
            ); 2] = [(Frames::h, "H", [0, 2, 1, 3]), (Frames::s, "S", [0, 1, 3, 2])];

            for action in GATES {
                let mut frames = Frames::<Vector>::default();
                frames.new_qubit(0);
                for pauli in (0..4).rev() {
                    frames
                        .track_pauli_string(vec![(0, Pauli::try_from(pauli).unwrap())]);
                }
                (action.0)(&mut frames, 0);
                for (input, check) in (0u8..).zip(action.2) {
                    assert_eq!(
                        *frames.pop_frame().unwrap().get(0).unwrap().1.storage(),
                        check,
                        "{}, {}",
                        action.1,
                        input
                    );
                }
            }
        }

        #[test]
        fn two_qubit() {
            // double-pauli p = abcd in binary;
            // encoding: x_0 = a, z_0 = b, x_1 = c, z_2 = d
            type Action = fn(&mut Frames<Vector>, usize, usize);
            const GATES: [(
                // action
                Action,
                // name for debugging
                &str,
                // result: calculated by hand
                // encoded input: p = 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
                [u8; 16],
            ); 2] = [
                (
                    Frames::cx, // left->control, right->target
                    "CX",
                    [0, 5, 2, 7, 4, 1, 6, 3, 10, 15, 8, 13, 14, 11, 12, 9],
                ),
                (
                    Frames::cz,
                    "CZ",
                    [0, 1, 6, 7, 4, 5, 2, 3, 9, 8, 15, 14, 13, 12, 11, 10],
                ),
            ];

            // masks to decode p in 0..16 into two paulis and vice versa
            const FIRST: u8 = 12;
            const FIRST_SHIFT: u8 = 2;
            const SECOND: u8 = 3;

            for action in GATES {
                let mut frames = Frames::<Vector>::default();
                frames.new_qubit(0);
                frames.new_qubit(1);
                for pauli in (0..16).rev() {
                    frames.track_pauli_string(vec![
                        (0, Pauli::try_from((pauli & FIRST) >> FIRST_SHIFT).unwrap()),
                        (1, Pauli::try_from(pauli & SECOND).unwrap()),
                    ]);
                }
                (action.0)(&mut frames, 0, 1);
                for (input, check) in (0u8..).zip(action.2) {
                    let frame = frames.pop_frame().unwrap();
                    let mut result = 0;
                    for (i, p) in frame {
                        if i == 0 {
                            result += p.storage() << FIRST_SHIFT
                        } else if i == 1 {
                            result += p.storage()
                        }
                    }
                    assert_eq!(result, check, "{}, {}", action.1, input);
                }
            }
        }
    }
}
