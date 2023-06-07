use std::fmt::Debug;

pub trait BoolVector:
    FromIterator<bool> + Clone + Debug + IntoIterator<Item = bool>
{
    type Iter<'l>: Iterator<Item = bool>
    where
        Self: 'l;

    fn new() -> Self;
    fn zeros(len: usize) -> Self;
    fn set(&mut self, idx: usize, flag: bool);
    fn xor_inplace(&mut self, rhs: &Self);
    fn or_inplace(&mut self, rhs: &Self);
    fn resize(&mut self, len: usize, flag: bool);
    fn push(&mut self, flag: bool);
    fn pop(&mut self) -> Option<bool>;
    fn bits(&self) -> usize;
    fn iter_vals(&self) -> Self::Iter<'_>;
}

#[cfg(feature = "bitvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitvec")))]
mod bitvec;
#[cfg(feature = "bitvec_simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitvec_simd")))]
pub mod bitvec_simd;
