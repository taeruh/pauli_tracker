use bitvec::{
    order::Lsb0,
    slice::{
        BitSlice,
        BitValIter,
    },
    store::BitStore,
    vec::BitVec,
};

use super::BooleanVector;

impl<T: BitStore> BooleanVector for BitVec<T, Lsb0> {
    type IterVals<'l> = BitValIter<'l, T, Lsb0>
    where
        Self: 'l;

    fn new() -> Self {
        BitVec::new()
    }

    fn zeros(len: usize) -> Self {
        Self::repeat(false, len)
    }

    fn set(&mut self, idx: usize, flag: bool) {
        *self.get_mut(idx).unwrap() = flag;
    }

    fn xor_inplace(&mut self, rhs: &Self) {
        *self ^= rhs;
    }

    fn or_inplace(&mut self, rhs: &Self) {
        *self |= rhs;
    }

    fn resize(&mut self, len: usize, flag: bool) {
        self.resize(len, flag);
    }

    fn push(&mut self, flag: bool) {
        self.push(flag)
    }

    fn pop(&mut self) -> Option<bool> {
        self.pop()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, idx: usize) -> Option<bool> {
        (**self).get(idx).map(|b| *b)
    }

    fn iter_vals(&self) -> Self::IterVals<'_> {
        BitSlice::iter(self).by_vals()
    }
}
