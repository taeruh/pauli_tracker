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
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::super::{
    PauliVec,
    StackStorage,
};
use crate::slice_extension::GetTwoMutSlice;

#[derive(Debug, Default)]
// this is basically a HashMap<key=usize, value=PauliVec> splitted into
// HashMap<key=usize, position_in_vec_=usize> and Vec<value=PauliVec>; we do this
// because it is more memory-efficient for many PauliVec(s) since HashMaps kinda need
// the memory even if there's no key
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl<'l> IntoIterator for &'l MappedVector {
    type Item = (usize, &'l PauliVec);
    type IntoIter = Zip<
        Map<slice::Iter<'l, usize>, fn(&usize) -> usize>, slice::Iter<'l, PauliVec>>
    where
        Self: 'l;

    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position
            .iter()
            .map((|i| *i) as fn(&usize) -> usize)
            .zip(self.frames.iter())
    }
}

impl<'l> IntoIterator for &'l mut MappedVector {
    type Item = (usize, &'l mut PauliVec);
    type IntoIter = Zip<
        Map<slice::Iter<'l, usize>, fn(&usize) -> usize>,
        slice::IterMut<'l, PauliVec>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position
            .iter()
            .map((|i| *i) as fn(&usize) -> usize)
            .zip(self.frames.iter_mut())
    }
}

impl StackStorage for MappedVector {
    type IterMut<'l> = <&'l mut Self as IntoIterator>::IntoIter;
    type Iter<'l> = <&'l Self as IntoIterator>::IntoIter;

    fn insert_pauli(&mut self, bit: usize, pauli: PauliVec) -> Option<PauliVec> {
        if self.position.insert(bit, self.frames.len()).is_some() {
            return Some(pauli);
        }
        self.frames.push(pauli);
        self.inverse_position.push(bit);
        None
    }

    fn remove_pauli(&mut self, bit: usize) -> Option<PauliVec> {
        let bit_position = self.position.remove(&bit)?;
        self.inverse_position.swap_remove(bit_position);
        if bit_position != self.inverse_position.len() {
            // when things are thoroughly tested, use get_unchecked here
            *self
                .position
                .get_mut(
                    self.inverse_position
                        .get(bit_position)
                        .expect("that's an implementation bug; please report"),
                )
                .expect("that's an implementation bug; please report") = bit_position;
        }
        Some(self.frames.swap_remove(bit_position))
    }

    #[inline]
    fn get(&self, bit: usize) -> Option<&PauliVec> {
        Some(self.frames.index(*self.position.get(&bit)?))
    }

    #[inline]
    fn get_mut(&mut self, bit: usize) -> Option<&mut PauliVec> {
        Some(self.frames.index_mut(*self.position.get(&bit)?))
    }

    fn get_two_mut(
        &mut self,
        bit_a: usize,
        bit_b: usize,
    ) -> Option<(&mut PauliVec, &mut PauliVec)> {
        self.frames
            .get_two_mut(*self.position.get(&bit_a)?, *self.position.get(&bit_b)?)
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.into_iter()
    }

    fn init(num_bits: usize) -> Self {
        let (frames, position, inverse_position) =
            (0..num_bits).map(|i| (PauliVec::new(), (i, i), i)).multiunzip();
        Self {
            frames,
            position,
            inverse_position,
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.frames.is_empty()
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
