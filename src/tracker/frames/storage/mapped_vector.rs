use std::{
    collections::HashMap,
    iter::{
        Map,
        Zip,
    },
    mem,
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

use super::{
    super::StackStorage,
    PauliVec,
};
use crate::{
    boolean_vector::BooleanVector,
    slice_extension::GetTwoMutSlice,
};

#[derive(Debug, Default)]
// this is basically a HashMap<key=usize, value=PauliVec> splitted into
// HashMap<key=usize, position_in_vec_=usize> and Vec<value=PauliVec>; we do this
// because it is more memory-efficient for many PauliVec(s) since HashMaps kinda need
// the memory even if there's no key
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MappedVector<B> {
    // note that we are effectively using an array of array; this wouldn't be optimal
    // if the inner array has a fixed size (then one could do the usual thing and
    // flatten the arrays into one array), however, this is not necessarily true
    // for us since we might continuesly add frames and remove qubits (when it is
    // measured) to reduce the required memory
    frames: Vec<PauliVec<B>>,
    position: HashMap<usize, usize>,
    inverse_position: Vec<usize>,
}

impl<B> IntoIterator for MappedVector<B> {
    type Item = (usize, PauliVec<B>);
    type IntoIter = Zip<
        <Vec<usize> as IntoIterator>::IntoIter,
        <Vec<PauliVec<B>> as IntoIterator>::IntoIter,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position.into_iter().zip(self.frames.into_iter())
    }
}

impl<'l, B> IntoIterator for &'l MappedVector<B> {
    type Item = (usize, &'l PauliVec<B>);
    type IntoIter = Zip<
        Map<slice::Iter<'l, usize>, fn(&usize) -> usize>,
        slice::Iter<'l, PauliVec<B>>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position
            .iter()
            .map((|i| *i) as fn(&usize) -> usize)
            .zip(self.frames.iter())
    }
}

impl<'l, B> IntoIterator for &'l mut MappedVector<B> {
    type Item = (usize, &'l mut PauliVec<B>);
    type IntoIter = Zip<
        Map<slice::Iter<'l, usize>, fn(&usize) -> usize>,
        slice::IterMut<'l, PauliVec<B>>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.inverse_position
            .iter()
            .map((|i| *i) as fn(&usize) -> usize)
            .zip(self.frames.iter_mut())
    }
}

impl<B: BooleanVector> StackStorage for MappedVector<B> {
    type BoolVec = B;
    type IterMut<'l> = <&'l mut Self as IntoIterator>::IntoIter where B: 'l;
    type Iter<'l> = <&'l Self as IntoIterator>::IntoIter where B: 'l;

    fn insert_pauli(&mut self, bit: usize, pauli: PauliVec<B>) -> Option<PauliVec<B>> {
        if let Some(&key) = self.position.get(&bit) {
            let old = mem::replace(self.frames.index_mut(key), pauli);
            return Some(old);
        }
        self.position.insert(bit, self.frames.len());
        self.frames.push(pauli);
        self.inverse_position.push(bit);
        None
    }

    fn remove_pauli(&mut self, bit: usize) -> Option<PauliVec<B>> {
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
    fn get(&self, bit: usize) -> Option<&PauliVec<B>> {
        Some(self.frames.index(*self.position.get(&bit)?))
    }

    #[inline]
    fn get_mut(&mut self, bit: usize) -> Option<&mut PauliVec<B>> {
        Some(self.frames.index_mut(*self.position.get(&bit)?))
    }

    fn get_two_mut(
        &mut self,
        bit_a: usize,
        bit_b: usize,
    ) -> Option<(&mut PauliVec<B>, &mut PauliVec<B>)> {
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

impl<B> MappedVector<B> {
    pub fn frames(&self) -> &Vec<PauliVec<B>> {
        &self.frames
    }

    pub fn inverse_position(&self) -> &Vec<usize> {
        &self.inverse_position
    }
}
