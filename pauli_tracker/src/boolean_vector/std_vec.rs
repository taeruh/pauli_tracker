use std::{
    iter::Copied,
    slice::Iter,
};

use super::BooleanVector;

impl BooleanVector for Vec<bool> {
    type IterVals<'l> = Copied<Iter<'l, bool>>
    where
        Self: 'l;

    fn new() -> Self {
        Vec::new()
    }

    fn zeros(len: usize) -> Self {
        vec![false; len]
    }

    fn set(&mut self, idx: usize, flag: bool) {
        *self.get_mut(idx).unwrap() = flag;
    }

    fn xor_inplace(&mut self, rhs: &Self) {
        check_len(self, rhs);
        for (l, r) in self.iter_mut().zip(rhs) {
            *l ^= r;
        }
    }

    fn or_inplace(&mut self, rhs: &Self) {
        check_len(self, rhs);
        for (l, r) in self.iter_mut().zip(rhs) {
            *l |= r;
        }
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

    fn iter_vals(&self) -> Self::IterVals<'_> {
        self.iter().copied()
    }
}

fn check_len<T>(lhs: &[T], rhs: &[T]) {
    assert_eq!(
        lhs.len(),
        rhs.len(),
        "left and right-hand side must have the same length"
    );
}
