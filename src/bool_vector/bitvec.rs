use bitvec::{
    order::Lsb0,
    slice::{
        BitSlice,
        BitValIter,
    },
    vec::BitVec,
};

use super::BoolVector;

impl BoolVector for BitVec {
    fn new() -> Self {
        BitVec::new()
    }

    fn zeros(len: usize) -> Self {
        bitvec::bitvec![0; len]
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

    fn bits(&self) -> usize {
        self.len()
    }

    type Iter<'l> = BitValIter<'l, usize, Lsb0>
    where
        Self: 'l;

    fn iter_vals(&self) -> Self::Iter<'_> {
        BitSlice::iter(self).by_vals()
    }
}
