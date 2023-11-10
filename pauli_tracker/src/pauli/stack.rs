/*!
  This module provides the [PauliStack] type, which stores multiple encoded Paulis.
*/

use std::{
    cmp::Ordering,
    mem,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};
use thiserror::Error;

use super::{
    Pauli,
    PauliTuple,
};
use crate::boolean_vector::BooleanVector;

/// Multiple encoded Paulis compressed into two [BooleanVector]s.
///
/// Instead of having a vector over [Pauli]s, we separate the X and Z parts into two
/// vectors (cf. [Pauli] for encoding). This enables us to efficiently perform
/// (Clifford) operations on those [PauliStack]s.
///
/// Note that the fields are public and the methods are mainly convenience methods.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PauliStack<T /* : BooleanVector */> {
    /// The bits representing the left qubit on the left-hand side in the tableau
    /// representation, i.e., X
    pub left: T,
    /// The bits representing the left qubit on the left-hand side in the tableau
    /// representation, i.e., Z
    pub right: T,
}

/// The Error when one tries to parse a char into a bool.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("'{chr}' is neither '0' nor '1'")]
pub struct BitCharError {
    /// The invalid char.
    pub chr: char,
}

impl<T: BooleanVector> PauliStack<T> {
    /// Create a new empty [PauliStack].
    pub fn new() -> Self {
        Self { left: T::new(), right: T::new() }
    }

    /// Create a [PauliStack] from two strings. `left` (`right`) corresponds to
    /// [PauliStack]s `left` (`right`) field.
    ///
    /// Errors if the strings do not consist only of '0' and '1' characters.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, coverage(off))]
    /// # fn main() {
    /// # use pauli_tracker::pauli::PauliStack;
    /// assert_eq!(
    ///     PauliStack::<Vec<bool>>::try_from_str("01", "10"),
    ///     Ok(PauliStack::<Vec<bool>> {
    ///         left: vec![false, true],
    ///         right: vec![true, false]
    ///     })
    /// )
    /// # }
    /// ```
    pub fn try_from_str(left: &str, right: &str) -> Result<Self, BitCharError> {
        fn to_bool(c: char) -> Result<bool, BitCharError> {
            match c.to_digit(2) {
                Some(d) => Ok(d == 1),
                None => Err(BitCharError { chr: c }),
            }
        }
        Ok(Self {
            left: left.chars().flat_map(to_bool).collect(),
            right: right.chars().flat_map(to_bool).collect(),
        })
    }

    /// Create a new [PauliStack] with both sides `left` and `right` initialized with
    /// `len` 0/false elements.
    pub fn zeros(len: usize) -> Self {
        let zero = T::zeros(len);
        Self { left: zero.clone(), right: zero }
    }

    /// Push a new [Pauli] onto the Pauli stack. If one part of the stack, i.e, `left`
    /// or `right`, is shorter than the other, it is filled up with `false/0` to have the
    /// same length, before the `pauli` is pushed.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, coverage(off))]
    /// # fn main() {
    /// # use pauli_tracker::pauli::{Pauli, PauliTuple, PauliStack};
    /// let mut pauli = PauliStack::try_from_str("1", "").unwrap();
    /// pauli.push::<PauliTuple>(Pauli::new_z());
    /// assert_eq!(
    ///     pauli,
    ///     PauliStack::<Vec<bool>> {
    ///         left: vec![true, false],
    ///         right: vec![false, true]
    ///     }
    /// );
    /// # }
    pub fn push<P: Pauli>(&mut self, pauli: P) {
        let left = self.left.len();
        let right = self.right.len();
        match left.cmp(&right) {
            Ordering::Less => self.left.resize(right, false),
            Ordering::Equal => {}
            Ordering::Greater => self.right.resize(left, false),
        }
        self.left.push(pauli.get_x());
        self.right.push(pauli.get_z());
    }

    /// Pop the last element from the stack and return it. If one part of the stack,
    /// i.e., `left` or `right` is shorter than the other, it `false/0` is substituted
    /// for the missing value. Returns [None] if both parts of the stacks are empty.
    /// is empty.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, coverage(off))]
    /// # fn main() {
    /// # use pauli_tracker::pauli::{Pauli, PauliTuple, PauliStack};
    /// let mut pauli = PauliStack::<Vec<bool>>::try_from_str("01", "1").unwrap();
    /// assert_eq!(pauli.pop(), Some(PauliTuple::X));
    /// assert_eq!(pauli.pop(), Some(PauliTuple::Z));
    /// assert_eq!(pauli.pop::<PauliTuple>(), None);
    /// # }
    pub fn pop<P: Pauli>(&mut self) -> Option<P> {
        match self.left.len().cmp(&self.right.len()) {
            Ordering::Less => Some(P::new_product(
                false,
                match self.right.pop() {
                    Some(v) => v,
                    // since right.len > left.len >= 0
                    None => unreachable!(),
                },
            )),
            Ordering::Equal => {
                Some(P::new_product(self.left.pop()?, self.right.pop()?))
            }
            Ordering::Greater => Some(P::new_product(
                match self.left.pop() {
                    Some(v) => v,
                    // since left.len > right.len >= 0
                    None => unreachable!(),
                },
                false,
            )),
        }
    }

    /// Get the Pauli at index `idx` from the stack.
    ///
    /// If one part of the stack, i.e., `left` or `right`, doesn't have the
    /// corresponding element, but the other does, the missing element is substituted
    /// with `false/0`. If both parts of the stack are too short, [None] is returned.
    pub fn get_with_default<P: Pauli>(&self, idx: usize) -> Option<P> {
        if idx < self.left.len() {
            Some(P::new_product(
                match self.left.get(idx) {
                    Some(v) => v,
                    // since idx < self.left.len()
                    None => unreachable!(),
                },
                self.right.get(idx).unwrap_or(false),
            ))
        } else if idx < self.right.len() {
            Some(P::new_product(
                self.left.get(idx).unwrap_or(false),
                match self.right.get(idx) {
                    Some(v) => v,
                    // since idx < self.right.len()
                    None => unreachable!(),
                },
            ))
        } else {
            None
        }
    }

    /// Get the Pauli at index `idx` from the stack, assuming that both stack parts have
    /// an element at that index.
    pub fn get<P: Pauli>(&self, idx: usize) -> Option<P> {
        P::new_product(self.left.get(idx)?, self.right.get(idx)?).into()
    }

    /// Perform a bitwise XOR between the left and right stacks of `self` and `other`,
    /// respectively, updating `self` in place.
    pub fn xor_inplace(&mut self, other: &Self) {
        self.left.xor_inplace(&other.left);
        self.right.xor_inplace(&other.right);
    }

    // we can define the action of local gates

    // Pauli gates don't do anything; we just include them for completeness and since it
    // might be more convenient to have them on the caller side
    /// Apply Pauli X, note that it is just the identity.
    pub fn x(&self) {}
    /// Apply Pauli Z, note that it is just the identity.
    pub fn z(&self) {}
    /// Apply Pauli Y, note that it is just the identity.
    pub fn y(&self) {}

    /// Conjugate the Paulistack with the S gate ignoring phases.
    pub fn s(&mut self) {
        self.right.xor_inplace(&self.left);
    }
    /// Conjugate the PauliStack with the Hadamard gate ignoring phases.
    pub fn h(&mut self) {
        mem::swap(&mut self.left, &mut self.right);
    }
    /// Conjugate the Paulistack with the SH gate ignoring phases.
    pub fn sh(&mut self) {
        // this is just sh ... is there a simpler way?
        mem::swap(&mut self.left, &mut self.right);
        self.right.xor_inplace(&self.left);
    }
    /// Conjugate the Paulistack with the HS gate ignoring phases.
    pub fn hs(&mut self) {
        // this is just hs ... is there a simpler way?
        self.right.xor_inplace(&self.left);
        mem::swap(&mut self.left, &mut self.right);
    }
    /// Conjugate the Paulistack with the SHS gate ignoring phases.
    pub fn shs(&mut self) {
        self.left.xor_inplace(&self.right);
    }

    /// Conjugate the Paulistack with the sqrt(X) gate ignoring phases.
    #[deprecated(since = "0.3.1", note = "use `shs` instead")]
    pub fn sx(&mut self) {
        self.left.xor_inplace(&self.right);
    }

    /// Multiply the Paulis, i.e., summing them up mod 2 in the tableau representation,
    /// with a `filter`, neglecting any phases. An element `e` is filtered if `filter[i]
    /// = true` where `i` is `e`'s index in [iter_vals](BooleanVector::iter_vals).
    /// Compare [BooleanVector::sum_up].
    ///
    /// # Panics
    /// Panics if `filter.len()` < number of Paulis
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, coverage(off))]
    /// # fn main() {
    /// # use pauli_tracker::{
    /// #    pauli::{PauliTuple, Pauli, PauliStack}, boolean_vector::BooleanVector};
    /// let paulis = [
    ///     PauliTuple::new_x(),
    ///     PauliTuple::new_y(),
    ///     PauliTuple::new_z(),
    ///     PauliTuple::new_x(),
    ///     PauliTuple::new_y(),
    ///     PauliTuple::new_z(),
    /// ]
    /// .into_iter()
    /// .collect::<PauliStack<Vec<bool>>>();
    /// let filter = [true, true, true, false, false, false];
    /// assert_eq!(paulis.sum_up(&filter), Pauli::new_i());
    /// # }
    /// ```
    pub fn sum_up(&self, filter: &[bool]) -> PauliTuple {
        PauliTuple::new_product(self.left.sum_up(filter), self.right.sum_up(filter))
    }
}

impl<T: BooleanVector, P: Pauli> FromIterator<P> for PauliStack<T> {
    fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Self {
        let mut ret = PauliStack::new();
        for pauli in iter {
            ret.push(pauli);
        }
        ret
    }
}
