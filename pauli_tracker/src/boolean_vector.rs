/*!
This module defines a common interface [BooleanVector] over boolean storage types that
we use in [frames](crate::tracker::frames) and for
[PauliStack](crate::pauli::PauliStack).

The trait is implement for [`Vec<bool>`] and
optionally for the foreign types [bitvec::vec::BitVec], [bitvec_simd::BitVec] and
[bit_vec::BitVec] (included via the corresponding features). There are other bit-vector
libraries too, for which it should be easy to implement [BooleanVector].

[bitvec::vec::BitVec]: https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html
[bit_vec::BitVec]: https://docs.rs/bit-vec/latest/bit_vec/struct.BitVec.html
[bitvec_simd::BitVec]: https://docs.rs/bitvec_simd/latest/bitvec_simd/type.BitVec.html
[smallvec]: https://docs.rs/smallvec/1.10.0/smallvec/
*/

use std::fmt::Debug;

macro_rules! inplace {
    ($(($name:ident, $action:literal),)*) => {$(
        /// Perform
        #[doc=$action]
        /// between `self` and `rhs` elementwise, updating self.
        ///
        /// # Panics
        /// Might panic if self.len() \neq rhs.len(). In general, if it does not panic, it
        /// probably applies the operation only on the elements of `self` maybe with some
        /// padding values for `rhs`, depending on the implementation.
        fn $name(&mut self, rhs: &Self);
    )*}
}

/// This trait defines the interface that we effectively require for the inner types of
/// [PauliStack](crate::pauli::PauliStack).
///
/// It is basically an interface that can be easily fullfilled by types like
/// [Vec]<[bool]>, "bit-vectors" or similar structures. Depending on the context, we use
/// true/false or 1/0 to when talking about the elements of the vector.
pub trait BooleanVector:
    Debug + Clone + Default + FromIterator<bool> + IntoIterator<Item = bool>
{
    /// An iterator over the [bool]ean values of the vector. It can be created with
    /// [Self::iter_vals].
    type IterVals<'l>: Iterator<Item = bool>
    where
        Self: 'l;

    /// Create a new empty boolean vector.
    fn new() -> Self;

    /// Create a boolean vector with `len` many `false/0` elements.
    ///
    /// # Examples
    ///```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// use pauli_tracker::boolean_vector::BooleanVector;
    /// assert_eq!(Vec::<bool>::zeros(3), vec![false, false, false])
    /// # }
    /// ```
    fn zeros(len: usize) -> Self;

    /// Set the element at `idx` to `flag`.
    ///
    /// # Panics
    /// Panics if `idx` is out of bounds.
    ///
    /// # Examples
    ///```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// use pauli_tracker::boolean_vector::BooleanVector;
    /// let mut vec = vec![true, false];
    /// vec.set(1, true);
    /// assert_eq!(vec, vec![true, true]);
    /// # }
    /// ```
    fn set(&mut self, idx: usize, flag: bool);

    inplace!((xor_inplace, "XOR"), (or_inplace, "OR"),);

    /// Resize the boolean vector to contain `len` elements, where new values are
    /// initialized with `flag`.
    ///
    /// # Examples
    ///```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// use pauli_tracker::boolean_vector::BooleanVector;
    /// let mut vec = vec![true, false];
    /// vec.resize(3, true);
    /// assert_eq!(vec, vec![true, false, true]);
    /// vec.resize(1, true);
    /// assert_eq!(vec, vec![true]);
    /// # }
    /// ```
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

    /// Get an element from the vector.
    fn get(&self, idx: usize) -> Option<bool>;

    /// Iterate over the stored values. Note that in contrast to the conventional iter()
    /// functions, the returned Iterator has `bool` items and not `&bool`. This is
    /// because some bit-vector iterators provide only this kind of iter() and for the
    /// other we can just deref the item via [map](Iterator::map).
    ///
    /// # Examples
    ///```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// use pauli_tracker::boolean_vector::BooleanVector;
    /// let vec = vec![true, false];
    /// let mut iter = vec.iter_vals();
    /// assert_eq!(iter.next(), Some(true));
    /// assert_eq!(iter.next(), Some(false));
    /// assert_eq!(iter.next(), None);
    /// # }
    /// ```
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
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::boolean_vector::BooleanVector;
    /// let bools = vec![true, false, true, false, true, false];
    /// let filter = [true, true, true, false, false, false];
    /// assert_eq!(bools.sum_up(&filter), false);
    /// # }
    /// ```
    fn sum_up(&self, filter: &[bool]) -> bool {
        self.iter_vals()
            .enumerate()
            .filter_map(|(i, f)| if filter[i] { Some(f) } else { None })
            .fold(false, |acc, next| acc ^ next)
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

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;

    #[test]
    fn is_empty() {
        assert!(<Vec<bool> as BooleanVector>::is_empty(&vec![]));
        assert!(!<Vec<bool> as BooleanVector>::is_empty(&vec![true]));
    }
}
