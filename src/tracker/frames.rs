use std::mem;

use bit_vec::BitVec;

use super::{
    PauliString,
    Tracker,
};
use crate::pauli::Pauli;

pub mod storage;

/// Multiple encoded Paulis compressed into two [BitVec]s.
// each Pauli can be described by two bits (neglecting phases)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct PauliVec {
    // the bit representing the left qubit on the left-hand side in the tableau
    // representation, i.e., X
    pub left: BitVec,
    // right-hand side, i.e., Z
    pub right: BitVec,
}

impl TryFrom<(&str, &str)> for PauliVec {
    type Error = String;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        fn to_bool(c: char) -> Result<bool, String> {
            match c.to_digit(2) {
                Some(d) => Ok(d == 1),
                None => Err(format!("{} is not a valid binary", c)),
            }
        }

        Ok(PauliVec {
            left: value.0.chars().flat_map(to_bool).collect(),
            right: value.1.chars().flat_map(to_bool).collect(),
        })
    }
}

impl PauliVec {
    pub fn new() -> Self {
        Self {
            left: BitVec::new(),
            right: BitVec::new(),
        }
    }

    pub fn zeros(len: usize) -> Self {
        let zero = zero_bitvec(len);
        Self { left: zero.clone(), right: zero }
    }

    pub fn push(&mut self, pauli: Pauli) {
        self.left.push(pauli.get_x());
        self.right.push(pauli.get_z());
    }

    pub fn pop_or_false(&mut self) -> Pauli {
        let l = self.left.pop().unwrap_or(false);
        let r = self.right.pop().unwrap_or(false);
        Pauli::new(l, r)
    }

    // pub fn reset(&mut self) {
    //     self.left.clear();
    //     self.right.clear();
    // }

    // we can define the action of local gates

    // Pauli gates don't do anything; we just include them for completeness and since it
    // might be more convenient to have them on the caller side
    /// Apply Pauli X, note that it is just the identity
    #[inline(always)]
    pub fn x(&self) {}
    /// Apply Pauli Z, note that it is just the identity
    #[inline(always)]
    pub fn z(&self) {}
    /// Apply Pauli Y, note that it is just the identity
    #[inline(always)]
    pub fn y(&self) {}

    /// Apply Hadamard
    #[inline]
    pub fn h(&mut self) {
        mem::swap(
            // Safety:
            // we don't do anything with the storage itself, so we should be good
            unsafe { self.left.storage_mut() },
            unsafe { self.right.storage_mut() },
        );
    }

    /// Apply Phase S
    #[inline]
    pub fn s(&mut self) {
        self.right.xor(&self.left);
    }
}

// not sure whether that is the fastest way
fn zero_bitvec(len: usize) -> BitVec {
    let rest = len % 8;
    let bytes = (len - rest) / 8;
    let mut ret = BitVec::from_bytes(&vec![0; bytes]);
    for _ in 0..rest {
        ret.push(false)
    }
    ret
}

/// This trait describes the functionality that a storage of [PauliVec]s must provide to
/// be used as storage for [Frames].
pub trait StackStorage: IntoIterator<Item = (usize, PauliVec)> {
    type IterMut<'a>: Iterator<Item = (usize, &'a mut PauliVec)>
    where
        Self: 'a;

    type Iter<'a>: Iterator<Item = (usize, &'a PauliVec)>
    where
        Self: 'a;

    fn insert_pauli(&mut self, qubit: usize, pauli: PauliVec) -> Option<PauliVec>;
    fn remove_pauli(&mut self, qubit: usize) -> Option<PauliVec>;
    fn get(&self, qubit: usize) -> Option<&PauliVec>;
    fn get_mut(&mut self, qubit: usize) -> Option<&mut PauliVec>;
    fn get_two_mut(
        &mut self,
        qubit_a: usize,
        qubit_b: usize,
    ) -> Option<(&mut PauliVec, &mut PauliVec)>;
    fn iter(&self) -> Self::Iter<'_>;
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
    fn init(num_qubits: usize) -> Self;
    fn is_empty(&self) -> bool;
}

/// A container of multiple Pauli frames, using a generic `Storage` type (that
/// implements [PauliStorage] if it shall be useful) as internal storage. The type
/// implements the core functionality to track the Pauli frames through a Clifford
/// circuit. As example view the documentation of [Circuit](crate::circuit). The
/// explicit storage type should have the [PauliVec]s on it's minor axis (this is more
/// or less enforced by [PauliStorage]). The module [storage] provides some compatible
/// storage types.
#[derive(Clone, Debug, Default)]
pub struct Frames<Storage /* : PauliStorageMap */> {
    storage: Storage,
    frames_num: usize,
}

impl<Storage> Frames<Storage> {
    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn frames_num(&self) -> usize {
        self.frames_num
    }

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

    pub fn measure_and_store(&mut self, qubit: usize, storage: &mut impl StackStorage) {
        storage.insert_pauli(qubit, self.measure(qubit).unwrap());
    }

    pub fn measure_and_store_all(self, storage: &mut impl StackStorage) {
        for (i, p) in self.storage.into_iter() {
            storage.insert_pauli(i, p);
        }
    }
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

    /// Apply the Hadamard gate.
    fn h(&mut self, qubit: usize) {
        self.storage
            .get_mut(qubit)
            .unwrap_or_else(|| panic!("qubit {qubit} does not exist"))
            .h();
    }

    /// Apply the Phase gate S.
    fn s(&mut self, qubit: usize) {
        self.storage
            .get_mut(qubit)
            .unwrap_or_else(|| panic!("qubit {qubit} does not exist"))
            .s();
    }

    /// Apply the Control X (Control Not) gate.
    fn cx(&mut self, control: usize, target: usize) {
        let (c, t) = self.storage.get_two_mut(control, target).unwrap();
        t.left.xor(&c.left);
        c.right.xor(&t.right);
    }

    /// Apply the Control Z gate.
    fn cz(&mut self, qubit_a: usize, qubit_b: usize) {
        let (a, b) = self.storage.get_two_mut(qubit_a, qubit_b).unwrap();
        a.right.xor(&b.left);
        b.right.xor(&a.left);
    }

    /// Perform an unspecified measurement. This removes the according qubit from being
    /// tracked.
    ///
    /// Returns the according [PauliVec] if it is a valid measurement, i.e., the qubit
    /// existed.
    fn measure(&mut self, qubit: usize) -> Option<PauliVec> {
        self.storage.remove_pauli(qubit)
    }

    // todo: test movements similar to the gates
    fn move_z_to_x(&mut self, source: usize, destination: usize) {
        let (s, d) = self.storage.get_two_mut(source, destination).unwrap();
        d.left.xor(&s.right);
        s.right.truncate(0)
    }

    fn move_z_to_z(&mut self, source: usize, destination: usize) {
        let (s, d) = self.storage.get_two_mut(source, destination).unwrap();
        d.right.xor(&s.right);
        s.right.truncate(0)
    }

    // pub fn reset_all(&mut self) {
    //     for (_, p) in self.storage.iter_mut() {
    //         p.reset()
    //     }
    //     self.frames_num = 0;
    // }
}

pub fn sort_pauli_storage(storage: &impl StackStorage) -> Vec<(usize, &PauliVec)> {
    let mut ret = storage.iter().collect::<Vec<(usize, &PauliVec)>>();
    ret.sort_by_key(|(i, _)| *i);
    ret
}

pub fn into_sorted_pauli_storage(storage: impl StackStorage) -> Vec<(usize, PauliVec)> {
    let mut ret = storage.into_iter().collect::<Vec<(usize, PauliVec)>>();
    ret.sort_by_key(|(i, _)| *i);
    ret
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
            type Action = fn(&mut Frames<FixedVector>, usize);
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
                let mut frames = Frames::<FixedVector>::default();
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
            type Action = fn(&mut Frames<FixedVector>, usize, usize);
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
                let mut frames = Frames::<FixedVector>::default();
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
