/*!
This module defines a common interface [BooleanVector] over boolean storage types that
we use in [frames](crate::tracker::frames) and for [PauliVec](crate::pauli::PauliVec).

We provide optional implementations for the foreign types
[bitvec::vec::BitVec](https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html),
[bitvec_simd::BitVec] (included via the corresponding features) and
[bit_vec::BitVec](https://docs.rs/bit-vec/latest/bit_vec/struct.BitVec.html).
However, note that these types of bit-vector implementations might not be the most
efficient for your problem, e.g., while [bitvec_simd::BitVec] uses SIMD operations,
it also uses the crate [smallvec](https://docs.rs/smallvec/1.10.0/smallvec/) for its
inner storage, which can be disadvantageous, depending on the situation. There are other
bit-vector libraries too, for which it should be easy to implement [BooleanVector].

[bitvec_simd::BitVec]: https://docs.rs/bitvec_simd/latest/bitvec_simd/type.BitVec.html
*/

use std::fmt::Debug;

/// This trait defines the interface that we require for storage types of boolean
/// values in [storage].
///
/// It is basically an interface that can be easily fullfilled by types like
/// [Vec]<[bool]>, "bit-vectors" or similar structures. Types that implement the trait,
/// can be used as generic parameter for the provided storage types in [storage].
/// Depending on the context, we use true/false or 1/0 to when talking about the
/// elements of the vector.
///
/// [storage]: crate::tracker::frames::storage
pub trait BooleanVector:
    Clone + FromIterator<bool> + IntoIterator<Item = bool> + Debug
{
    /// An iterator over the [bool]ean values of the vector. It can be created with
    /// [Self::iter_vals].
    type IterVals<'l>: Iterator<Item = bool>
    where
        Self: 'l;

    /// Create a new empty boolean vector.
    fn new() -> Self;

    /// Create [Self] with `len` many `false/0` elements.
    fn zeros(len: usize) -> Self;

    /// Set the element at `idx` to `flag`.
    ///
    /// # Panics
    /// Panics if `idx` is out of bounds.
    fn set(&mut self, idx: usize, flag: bool);

    /// Perform XOR between `self` and `rhs` elementwise, updating self.
    ///
    /// # Panics
    /// Panics if self.len() \neq rhs.len().
    fn xor_inplace(&mut self, rhs: &Self);

    /// Perform OR between `self` and `rhs` elementwise, updating self.
    ///
    /// # Panics
    /// Panics if self.len() \neq rhs.len().
    fn or_inplace(&mut self, rhs: &Self);

    /// Resize the boolean vector to contain `len` elements, where new values are
    /// initialized with `flag`.
    fn resize(&mut self, len: usize, flag: bool);

    /// Push a new element onto the vector.
    fn push(&mut self, flag: bool);

    /// Pop the last element from the vector and return it. Returns [None] if the vector
    /// is empty.
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

    /// Sum up the elements modulo 2 with a `filter`. We represent `true <-> 1`, `false
    /// <-> 0` and sum the filtered elements mod 2. An element `e` is filtered if
    /// `filter[i] = true` where `i` is `e`'s index in
    /// [iter_vals](BooleanVector::iter_vals).
    ///
    /// # Panics
    /// Panics if `filter.len()` < number of itered elements (should be self.len()).
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::boolean_vector::BooleanVector;
    /// let bools = vec![true, false, true, false, true, false];
    /// let filter = [true, true, true, false, false, false];
    /// assert_eq!(bools.sum_up(&filter), 0);
    /// # }
    /// ```
    fn sum_up(&self, filter: &[bool]) -> u8 {
        self.iter_vals()
            .enumerate()
            .filter_map(|(i, f)| if filter[i] { Some(f as u8) } else { None })
            .sum::<u8>()
            % 2
    }
}

mod std_vec;

#[cfg(feature = "bitvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitvec")))]
mod bitvec;

#[cfg(feature = "bitvec_simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitvec_simd")))]
pub mod bitvec_simd;

#[cfg(feature = "bit-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "bit-vec")))]
mod bit_vec;
