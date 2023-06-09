//! This module defines a common interface [BooleanVector] over boolean storage types
//! that we use in [frames](crate::tracker::frames) and for
//! [PauliVec](crate::pauli::PauliVec).
//!
//! We provide optional implementations for the foreign types
//! [bitvec::vec::BitVec](https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html),
//! [bitvec_simd::BitVec] (included via the corresponding features) and
//! [bit_vec::BitVec](https://docs.rs/bit-vec/latest/bit_vec/struct.BitVec.html).
//! However, note that these types of bit-vector implementations might not be the most
//! efficient for your problem, e.g., while [bitvec_simd::BitVec] uses SIMD operations,
//! it also uses the crate [smallvec](https://docs.rs/smallvec/1.10.0/smallvec/) for its
//! inner storage, which can be disadvantageous, depending on the situation. There are
//! other bit-vector libraries too, for which it should be easy to implement
//! [BooleanVector].
//!
//! [bitvec_simd::BitVec]:
//! https://docs.rs/bitvec_simd/latest/bitvec_simd/type.BitVec.html

use std::fmt::Debug;

/// This trait defines the interface that we require for storage types of boolean
/// values in [storage].
///
/// It is basically an interface that can be easily fullfilled by types like
/// [Vec]<[bool]>, "bit-vectors" or similar structures. Types that implement the trait,
/// can be used as generic parameter for the provided storage types in [storage].
///
/// [storage]: crate::tracker::frames::storage
pub trait BooleanVector:
    Clone + FromIterator<bool> + IntoIterator<Item = bool> + Debug
{
    type IterVals<'l>: Iterator<Item = bool>
    where
        Self: 'l;

    fn new() -> Self;

    /// Create [Self] with `len` many `false/0` elements.
    fn zeros(len: usize) -> Self;

    /// Set the element at `idx` to `flag`.
    ///
    /// # Panics
    /// Panics if `idx` is out of bounds.
    fn set(&mut self, idx: usize, flag: bool);

    fn xor_inplace(&mut self, rhs: &Self);

    fn or_inplace(&mut self, rhs: &Self);

    /// Resize the boolean vector to contain `len` elements, where new values are
    /// initialized with `flag`.
    fn resize(&mut self, len: usize, flag: bool);

    fn push(&mut self, flag: bool);

    fn pop(&mut self) -> Option<bool>;

    /// Return the number of contained elements.
    fn len(&self) -> usize;

    /// Check whether the vector is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over the stored values. Note that in contrast to the conventional iter()
    /// functions, the returned Iterator has `bool` items and not `&bool`. This is
    /// because some bit-vector iterators provide only this kind of iter() and for the
    /// other we can just deref the item via [map](Iterator::map).
    fn iter_vals(&self) -> Self::IterVals<'_>;

    fn sum_up(&self, measurements: &[bool]) -> u8 {
        self.iter_vals()
            .enumerate()
            .filter_map(|(i, f)| if measurements[i] { Some(f as u8) } else { None })
            .sum::<u8>()
            % 2
    }
}

#[cfg(feature = "bitvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitvec")))]
mod bitvec;

#[cfg(feature = "bitvec_simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitvec_simd")))]
pub mod bitvec_simd;

#[cfg(feature = "bit-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "bit-vec")))]
pub mod bit_vec;
