use std::{
    collections::{
        hash_map,
        HashMap,
    },
    iter::Zip,
    ops::{
        Index,
        IndexMut,
    },
    slice,
};

use itertools::Itertools;

use super::{
    PauliStorageMap,
    PauliVec,
};

pub type SmallPauliStorage = HashMap<usize, PauliVec>;

impl PauliStorageMap for SmallPauliStorage {
    type IterMut<'a> = hash_map::IterMut<'a, usize, PauliVec>;
    type Iter<'a> = hash_map::Iter<'a, usize, PauliVec>;

    fn insert_pauli(&mut self, qubit: usize, pauli: PauliVec) -> Option<PauliVec> {
        self.insert(qubit, pauli)
    }

    fn remove_pauli(&mut self, qubit: usize) -> Option<PauliVec> {
        self.remove(&qubit)
    }

    fn get(&self, qubit: usize) -> Option<&PauliVec> {
        self.get(&qubit)
    }

    fn get_mut(&mut self, qubit: usize) -> Option<&mut PauliVec> {
        self.get_mut(&qubit)
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    fn init(num_qubits: usize) -> Self {
        let mut ret = HashMap::new();
        for i in 0..num_qubits {
            ret.insert(i, PauliVec::new());
        }
        ret
    }
}

#[derive(Debug, Default)]
// this is basically a HashMap<key=usize, value=PauliVec> splitted into
// HashMap<key=usize, position_in_vec_=usize> and Vec<value=PauliVec>; we do this
// because it is more memory-efficient for many PauliVec(s) since HashMaps kinda need
// the memory even if there's no key
pub struct PauliStorage {
    // note that we are effectively using an array of array; this wouldn't be optimal
    // if the inner array has a fixed size (then one could do the usual thing and
    // flatten the arrays into one array), however, this is not necessarily true
    // for us since we might continuesly add frames and remove qubits (when it is
    // measured) to reduce the required memory
    frames: Vec<PauliVec>,
    position: HashMap<usize, usize>,
    inverse_position: Vec<usize>,
}

impl IntoIterator for PauliStorage {
    type Item = (usize, PauliVec);

    type IntoIter = Zip<
        <Vec<usize> as IntoIterator>::IntoIter,
        <Vec<PauliVec> as IntoIterator>::IntoIter,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position.into_iter().zip(self.frames.into_iter())
    }
}

impl PauliStorageMap for PauliStorage {
    type IterMut<'a> = Zip<slice::Iter<'a, usize>, slice::IterMut<'a, PauliVec>>
    where
        Self: 'a;
    type Iter<'a> = Zip<slice::Iter<'a, usize>, slice::Iter<'a, PauliVec>>
    where
        Self: 'a;

    fn insert_pauli(&mut self, qubit: usize, pauli: PauliVec) -> Option<PauliVec> {
        if self.position.insert(qubit, self.frames.len()).is_some() {
            return Some(pauli);
        }
        self.frames.push(pauli);
        self.inverse_position.push(qubit);
        None
    }

    fn remove_pauli(&mut self, qubit: usize) -> Option<PauliVec> {
        let current = self.position.remove(&qubit)?;
        self.inverse_position.swap_remove(current);
        *self.position.get_mut(&self.inverse_position[current]).unwrap() = current;
        Some(self.frames.swap_remove(current))
    }

    fn get(&self, qubit: usize) -> Option<&PauliVec> {
        Some(self.frames.index(*self.position.get(&qubit)?))
    }

    fn get_mut(&mut self, qubit: usize) -> Option<&mut PauliVec> {
        Some(self.frames.index_mut(*self.position.get_mut(&qubit)?))
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.inverse_position.iter().zip(self.frames.iter_mut())
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.inverse_position.iter().zip(self.frames.iter())
    }

    fn init(num_qubits: usize) -> Self {
        let (frames, position, inverse_position) =
            (0..num_qubits).map(|i| (PauliVec::new(), (i, i), i)).multiunzip();
        Self {
            frames,
            position,
            inverse_position,
        }
    }
}

impl PauliStorage {
    /// Create an empty set of frames.
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            position: HashMap::new(),
            inverse_position: Vec::new(),
        }
    }
}
