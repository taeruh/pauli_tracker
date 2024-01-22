/*!
A [BooleanVector] implementation for a SIMD vector.

Compare the documentation of [bitvec_simd](https://docs.rs/bitvec_simd/latest/bitvec_simd/index.html).
*/

use bitvec_simd::BitVec;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::BooleanVector;

/// A newtype wrapper around
/// [bitvec_simd::BitVec](https://docs.rs/bitvec_simd/latest/bitvec_simd/type.BitVec.html).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimdBitVec(pub BitVec);

impl FromIterator<bool> for SimdBitVec {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut res = SimdBitVec::new();
        for f in iter {
            res.push(f);
        }
        res
    }
}

impl Default for SimdBitVec {
    fn default() -> Self {
        Self::zeros(0)
    }
}

/// An [Iterator] over [SimdBitVec]. Create with [IntoIterator].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Iter {
    vec: SimdBitVec,
    current: usize,
}
impl Iterator for Iter {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        self.vec.0.get(self.current - 1)
    }
}

/// An [Iterator] over &[SimdBitVec]. Create with [BooleanVector::iter_vals].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IterFromRef<'l> {
    vec: &'l SimdBitVec,
    current: usize,
}
impl<'l> Iterator for IterFromRef<'l> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        self.vec.0.get(self.current - 1)
    }
}

impl IntoIterator for SimdBitVec {
    type Item = bool;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter { vec: self, current: 0 }
    }
}

impl BooleanVector for SimdBitVec {
    type IterVals<'l> = IterFromRef<'l>;

    fn new() -> Self {
        Self::zeros(0)
    }

    fn zeros(len: usize) -> Self {
        Self(BitVec::zeros(len))
    }

    fn set(&mut self, idx: usize, flag: bool) {
        assert!(idx < self.len());
        self.0.set(idx, flag);
    }

    fn xor_inplace(&mut self, rhs: &Self) {
        self.0.xor_inplace(&rhs.0);
    }

    fn or_inplace(&mut self, rhs: &Self) {
        self.0.or_inplace(&rhs.0);
    }

    fn resize(&mut self, len: usize, flag: bool) {
        self.0.resize(len, flag);
    }

    fn push(&mut self, flag: bool) {
        // let len = self.num_bools();
        // println!("test: {}, {}, {}", flag, self.num_bools(), self.0.count_ones());
        // // why do we have to do that, is this a bug in bitvec_simd? okay I don't get
        // the set function, it also // breaks (in the roundtrip proptest) when we reach
        // len=256 (which is bitvec_simds "bucket" size)
        // if len == 0 {
        //     if flag {
        //         self.0 = BitVec::ones(1);
        //     } else {
        //         self.0 = BitVec::zeros(1);
        //     }
        // } else {
        //     self.0.set(len, flag);
        // }
        self.0.resize(self.len() + 1, flag)
    }

    fn pop(&mut self) -> Option<bool> {
        let last = self.len().checked_sub(1)?;
        // last > self.0.len is not possible because of the above
        let res = self.0.get_unchecked(last);
        self.0.shrink_to(last);
        Some(res)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn get(&self, idx: usize) -> Option<bool> {
        self.0.get(idx)
    }

    fn iter_vals(&self) -> Self::IterVals<'_> {
        IterFromRef { vec: self, current: 0 }
    }
}
