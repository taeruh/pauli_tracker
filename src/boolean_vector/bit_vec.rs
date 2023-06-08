use bit_vec::{
    BitVec,
    Iter,
};

use super::BooleanVector;

impl BooleanVector for BitVec {
    fn new() -> Self {
        BitVec::new()
    }

    fn zeros(len: usize) -> Self {
        // not sure whether that is the fastest way
        let rest = len % 8;
        let bytes = (len - rest) / 8;
        let mut ret = BitVec::from_bytes(&vec![0; bytes]);
        for _ in 0..rest {
            ret.push(false)
        }
        ret
    }

    fn set(&mut self, idx: usize, flag: bool) {
        self.set(idx, flag);
    }

    fn xor_inplace(&mut self, rhs: &Self) {
        self.xor(rhs);
    }

    fn or_inplace(&mut self, rhs: &Self) {
        self.or(rhs);
    }

    fn resize(&mut self, len: usize, flag: bool) {
        let current_len = self.len();
        match current_len.cmp(&len) {
            std::cmp::Ordering::Less => self.grow(len - current_len, flag),
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => self.truncate(len),
        }
    }

    fn push(&mut self, flag: bool) {
        self.push(flag)
    }

    fn pop(&mut self) -> Option<bool> {
        self.pop()
    }

    fn num_bools(&self) -> usize {
        self.len()
    }

    type IterVals<'l> = Iter<'l>
    where
        Self: 'l;

    fn iter_vals(&self) -> Self::IterVals<'_> {
        self.iter()
    }
}
