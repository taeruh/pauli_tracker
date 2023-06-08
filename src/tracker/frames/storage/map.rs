use std::{
    collections::{
        hash_map,
        HashMap,
    },
    iter,
};

use super::{
    super::StackStorage,
    PauliVec,
};
use crate::boolean_vector::BooleanVector;

/// A HashMap of [PauliVec]s. Much more flexible than [Vector], but for restricted use
/// cases [Vector] might be more efficient.
///
///[Vector]: super::vector::Vector
pub type Map<B> = HashMap<usize, PauliVec<B>>;

impl<B: BooleanVector> StackStorage for Map<B> {
    type BoolVec = B;
    type IterMut<'l> = iter::Map<
        hash_map::IterMut<'l, usize, PauliVec<B>>,
        fn((&usize, &'l mut PauliVec<B>)) -> (usize, &'l mut PauliVec<B>),
    > where B: 'l;
    type Iter<'l> = iter::Map<
        hash_map::Iter<'l, usize, PauliVec<B>>,
        fn((&usize, &'l PauliVec<B>)) -> (usize, &'l PauliVec<B>),
    > where B: 'l;

    #[inline]
    fn insert_pauli(
        &mut self,
        qubit: usize,
        pauli: PauliVec<B>,
    ) -> Option<PauliVec<B>> {
        self.insert(qubit, pauli)
    }

    #[inline]
    fn remove_pauli(&mut self, qubit: usize) -> Option<PauliVec<B>> {
        self.remove(&qubit)
    }

    #[inline]
    fn get(&self, qubit: usize) -> Option<&PauliVec<Self::BoolVec>> {
        self.get(&qubit)
    }

    #[inline]
    fn get_mut(&mut self, qubit: usize) -> Option<&mut PauliVec<B>> {
        self.get_mut(&qubit)
    }

    fn get_two_mut(
        &mut self,
        qubit_a: usize,
        qubit_b: usize,
    ) -> Option<(&mut PauliVec<B>, &mut PauliVec<B>)> {
        if qubit_a == qubit_b {
            return None;
        }
        // Safety: we checked above that the keys are different, so it is impossible
        // that we create two mutable references to the same object (except if there is
        // a bug in the bucket assigment of the HashMap)
        //
        // I do not know why this isn't triggering an stack-borrow error in miri; doing
        // the same with Vec/slice does trigger an error. In general it would be cleaner
        // to go over pointers as I do it for the MappedVector but a HashMap is more
        // complicated and the tools for that are not stable yet
        let a = unsafe { &mut *(self.get_mut(&qubit_a)? as *mut PauliVec<B>) };
        let b = unsafe { &mut *(self.get_mut(&qubit_b)? as *mut PauliVec<B>) };
        // that would catch a bug in the bucket assignment
        // assert!(!std::ptr::eq(a, b));
        Some((a, b))
    }

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter().map(|(&i, p)| (i, p))
    }

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut().map(|(&i, p)| (i, p))
    }

    fn init(num_qubits: usize) -> Self {
        let mut ret = HashMap::with_capacity(num_qubits);
        for i in 0..num_qubits {
            ret.insert(i, PauliVec::<B>::new());
        }
        ret
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
