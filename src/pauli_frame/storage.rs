use std::{
    cmp::Ordering,
    collections::{
        hash_map,
        HashMap,
    },
    iter::{
        Enumerate,
        Map,
        Zip,
    },
    ops::{
        Deref,
        DerefMut,
        Index,
        IndexMut,
    },
    slice,
};

use itertools::Itertools;

use super::{
    PauliStorage,
    PauliVec,
};

pub type FullMap = HashMap<usize, PauliVec>;

impl PauliStorage for FullMap {
    type IterMut<'a> = Map<
        hash_map::IterMut<'a, usize, PauliVec>,
        fn((&usize, &'a mut PauliVec)) -> (usize, &'a mut PauliVec),
    >;
    type Iter<'a> = Map<
        hash_map::Iter<'a, usize, PauliVec>,
        fn((&usize, &'a PauliVec)) -> (usize, &'a PauliVec),
    >;

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

    fn iter(&self) -> Self::Iter<'_> {
        self.iter().map(|(&i, p)| (i, p))
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut().map(|(&i, p)| (i, p))
    }

    fn init(num_qubits: usize) -> Self {
        let mut ret = HashMap::with_capacity(num_qubits);
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
        Some(self.frames.index_mut(*self.position.get_mut(&qubit)?))
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

pub struct FixedVector {
    frames: Vec<PauliVec>,
}

impl Deref for FixedVector {
    type Target = Vec<PauliVec>;
    fn deref(&self) -> &Self::Target {
        &self.frames
    }
}

impl DerefMut for FixedVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frames
    }
}

impl IntoIterator for FixedVector {
    type Item = (usize, PauliVec);

    type IntoIter = Enumerate<<Vec<PauliVec> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.frames.into_iter().enumerate()
    }
}

impl PauliStorage for FixedVector {
    type IterMut<'a> = Enumerate<slice::IterMut<'a, PauliVec>>
    where
        Self: 'a;
    type Iter<'a> = Enumerate<slice::Iter<'a, PauliVec>>
    where
        Self: 'a;

    fn insert_pauli(&mut self, qubit: usize, pauli: PauliVec) -> Option<PauliVec> {
        match qubit.cmp(&self.len()) {
            Ordering::Less => Some(pauli),
            Ordering::Equal => {
                self.push(pauli);
                None
            }
            Ordering::Greater => panic!("..."),
        }
    }

    fn remove_pauli(&mut self, qubit: usize) -> Option<PauliVec> {
        match qubit.cmp(&self.len()) {
            Ordering::Less => panic!("..."),
            Ordering::Equal => Some(self.pop().unwrap()),
            Ordering::Greater => None,
        }
    }

    #[inline(always)]
    fn get(&self, qubit: usize) -> Option<&PauliVec> {
        self.frames.get(qubit)
    }

    #[inline(always)]
    fn get_mut(&mut self, qubit: usize) -> Option<&mut PauliVec> {
        self.frames.get_mut(qubit)
    }

    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        self.frames.iter().enumerate()
    }

    #[inline(always)]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.frames.iter_mut().enumerate()
    }

    #[inline(always)]
    fn init(num_qubits: usize) -> Self {
        Self {
            frames: vec![PauliVec::new(); num_qubits],
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // First we test the methods of [FullMap] that are not just simple redirections.
    // Then we use [FullMap] to as reference to test the other storages

    #[test]
    fn full_map() {
        /* all trivial */
    }

    #[test]
    fn mapped_vec() {
        // do some fuzzing using dispatch_storage_operation_comparison below
    }
}

// #[cfg(test)]
// fn dispatch_storage_operation_comparison(
//     storage: &mut (impl PauliStorage + PartialEq + Clone),
//     other: &mut FullMap,
//     operation: u8,
//     bit: usize,
// ) {
//     let operation = operation % 3;
//     match operation {
//         0 => {
//             assert_eq!(
//                 storage.insert_pauli(bit, PauliVec::new()),
//                 other.insert_pauli(bit, PauliVec::new())
//             );
//         }
//         1 => {
//             assert_eq!(storage.remove_pauli(bit), other.remove_pauli(bit));
//         }
//         2 => {
//             assert_eq!(storage.get(bit), other.get(&bit));
//         }
//         _ => {}
//     }
//     let compare = FullMap::from_iter(storage.clone().into_iter());
// }
