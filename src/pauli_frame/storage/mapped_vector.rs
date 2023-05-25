use std::{
    collections::HashMap,
    iter::{
        Map,
        Zip,
    },
    ops::{
        Index,
        IndexMut,
    },
    slice,
};

use itertools::Itertools;

use super::{
    super::{
        PauliStorage,
        PauliVec,
    },
    GetTwoMutSlice,
};

#[derive(Debug, Default)]
// this is basically a HashMap<key=usize, value=PauliVec> splitted into
// HashMap<key=usize, position_in_vec_=usize> and Vec<value=PauliVec>; we do this
// because it is more memory-efficient for many PauliVec(s) since HashMaps kinda need
// the memory even if there's no key
pub struct MappedVector {
    // note that we are effectively using an array of array; this wouldn't be optimal
    // if the inner array has a fixed size (then one could do the usual thing and
    // flatten the arrays into one array), however, this is not necessarily true
    // for us since we might continuesly add frames and remove qubits (when it is
    // measured) to reduce the required memory
    frames: Vec<PauliVec>,
    position: HashMap<usize, usize>,
    inverse_position: Vec<usize>,
}

impl IntoIterator for MappedVector {
    type Item = (usize, PauliVec);

    type IntoIter = Zip<
        <Vec<usize> as IntoIterator>::IntoIter,
        <Vec<PauliVec> as IntoIterator>::IntoIter,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position.into_iter().zip(self.frames.into_iter())
    }
}

impl PauliStorage for MappedVector {
    type IterMut<'a> = Zip<
        Map< slice::Iter<'a, usize>, fn(&usize) -> usize>, slice::IterMut<'a, PauliVec>>
    where
        Self: 'a;
    type Iter<'a> = Zip<
        Map<slice::Iter<'a, usize>, fn(&usize) -> usize>, slice::Iter<'a, PauliVec>>
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
        if current != self.inverse_position.len() {
            *self.position.get_mut(&self.inverse_position[current]).unwrap() = current;
        }
        Some(self.frames.swap_remove(current))
    }

    fn get(&self, qubit: usize) -> Option<&PauliVec> {
        Some(self.frames.index(*self.position.get(&qubit)?))
    }

    fn get_mut(&mut self, qubit: usize) -> Option<&mut PauliVec> {
        Some(self.frames.index_mut(*self.position.get(&qubit)?))
    }

    fn get_two_mut(
        &mut self,
        qubit_a: usize,
        qubit_b: usize,
    ) -> Option<(&mut PauliVec, &mut PauliVec)> {
        self.frames
            .get_two_mut(*self.position.get(&qubit_a)?, *self.position.get(&qubit_b)?)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.inverse_position
            .iter()
            .map(
                (|i| *i)
                 // compiler needs help coercing that closure, since Self::Iter is too
                 // complex
                 as fn(&usize) -> usize,
            )
            .zip(self.frames.iter())
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.inverse_position
            .iter()
            .map((|i| *i) as fn(&usize) -> usize)
            .zip(self.frames.iter_mut())
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

impl MappedVector {
    pub fn frames(&self) -> &Vec<PauliVec> {
        &self.frames
    }

    pub fn inverse_position(&self) -> &Vec<usize> {
        &self.inverse_position
    }
}
