/*!
  This module provides the [PauliStack] type, which stores multiple encoded Paulis.
*/

use std::{cmp::Ordering, mem};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{Pauli, PauliTuple};
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
    /// The Z Pauli mask, i.e., the bits flagging whether there's a Z Pauli.
    // backwards compatibility (at least when deserializing)
    #[cfg_attr(feature = "serde", serde(alias = "right"))]
    pub z: T,
    /// The X Pauli mask, i.e., the bits flagging whether there's a X Pauli.
    #[cfg_attr(feature = "serde", serde(alias = "left"))] // backwards comp....
    pub x: T,
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
        Self { z: T::new(), x: T::new() }
    }

    /// Create a [PauliStack] from two binary strings. '0' is interpreted as false and '1'
    /// is interpreted as true.
    ///
    /// Errors if the strings do not consist only of '0' and '1' characters.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::PauliStack;
    /// assert_eq!(
    ///     PauliStack::<Vec<bool>>::try_from_str("01", "10"),
    ///     Ok(PauliStack::<Vec<bool>> {
    ///         z: vec![false, true],
    ///         x: vec![true, false]
    ///     })
    /// )
    /// # }
    /// ```
    pub fn try_from_str(z: &str, x: &str) -> Result<Self, BitCharError> {
        fn to_bool(c: char) -> Result<bool, BitCharError> {
            match c.to_digit(2) {
                Some(d) => Ok(d == 1),
                None => Err(BitCharError { chr: c }),
            }
        }
        Ok(Self {
            z: z.chars().flat_map(to_bool).collect(),
            x: x.chars().flat_map(to_bool).collect(),
        })
    }

    /// Create a new [PauliStack] with both masks, `z` and `x` initialized with `len`
    /// 0/false elements.
    pub fn zeros(len: usize) -> Self {
        let zero = T::zeros(len);
        Self { z: zero.clone(), x: zero }
    }

    /// Push a new [Pauli] onto the Pauli stack. If one part of the stack, i.e, `z`
    /// or `x`, is shorter than the other, it is filled up with `false/0` to have the
    /// same length, before the `pauli` is pushed.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::{Pauli, PauliTuple, PauliStack};
    /// let mut pauli = PauliStack::try_from_str("", "1").unwrap();
    /// pauli.push::<PauliTuple>(Pauli::new_z());
    /// assert_eq!(
    ///     pauli,
    ///     PauliStack::<Vec<bool>> {
    ///         z: vec![false, true],
    ///         x: vec![true, false]
    ///     }
    /// );
    /// # }
    /// ```
    pub fn push<P: Pauli>(&mut self, pauli: P) {
        let z_len = self.z.len();
        let x_len = self.x.len();
        match z_len.cmp(&x_len) {
            Ordering::Less => self.z.resize(x_len, false),
            Ordering::Equal => {},
            Ordering::Greater => self.x.resize(z_len, false),
        }
        self.z.push(pauli.get_z());
        self.x.push(pauli.get_x());
    }

    /// Pop the last element from the stack and return it. If one part of the stack,
    /// i.e., `z` or `x` is shorter than the other, it `false/0` is substituted for the
    /// missing value. Returns [None] if both parts of the stacks are empty. is empty.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::{Pauli, PauliTuple, PauliStack};
    /// let mut pauli = PauliStack::<Vec<bool>>::try_from_str("1", "01").unwrap();
    /// assert_eq!(pauli.pop(), Some(PauliTuple::X));
    /// assert_eq!(pauli.pop(), Some(PauliTuple::Z));
    /// assert_eq!(pauli.pop::<PauliTuple>(), None);
    /// # }
    /// ```
    pub fn pop<P: Pauli>(&mut self) -> Option<P> {
        match self.z.len().cmp(&self.x.len()) {
            Ordering::Less => Some(P::new_product(
                false,
                match self.x.pop() {
                    Some(v) => v,
                    // since x.len > z.len >= 0
                    None => unreachable!(),
                },
            )),
            Ordering::Equal => Some(P::new_product(self.z.pop()?, self.x.pop()?)),
            Ordering::Greater => Some(P::new_product(
                match self.z.pop() {
                    Some(v) => v,
                    // since z.len > x.len >= 0
                    None => unreachable!(),
                },
                false,
            )),
        }
    }

    /// Get the Pauli at index `idx` from the stack.
    ///
    /// If one part of the stack, i.e., `x` or `z`, doesn't has the corresponding element,
    /// but the other does, the missing element is substituted with `false/0`. If both
    /// parts of the stack are too short, [None] is returned.
    pub fn get_with_default<P: Pauli>(&self, idx: usize) -> Option<P> {
        if idx < self.z.len() {
            Some(P::new_product(
                match self.z.get(idx) {
                    Some(v) => v,
                    // since 0 <= idx < self.z.len()
                    None => unreachable!(),
                },
                self.x.get(idx).unwrap_or(false),
            ))
        } else if idx < self.x.len() {
            Some(P::new_product(
                self.z.get(idx).unwrap_or(false),
                match self.x.get(idx) {
                    Some(v) => v,
                    // since 0 <= idx < self.x.len()
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
        P::new_product(self.z.get(idx)?, self.x.get(idx)?).into()
    }

    /// Perform a bitwise XOR between the z and x stacks of `self` and `other`,
    /// respectively, updating `self` in place.
    pub fn xor_inplace(&mut self, other: &Self) {
        self.z.xor_inplace(&other.z);
        self.x.xor_inplace(&other.x);
    }

    // we can define the action of local gates

    /// Conjugate the Paulistack with the S gate ignoring phases.
    pub fn s(&mut self) {
        self.z.xor_inplace(&self.x);
    }
    /// Conjugate the PauliStack with the Hadamard gate ignoring phases.
    pub fn h(&mut self) {
        mem::swap(&mut self.z, &mut self.x);
    }
    /// Conjugate the Paulistack with the SH gate ignoring phases.
    pub fn sh(&mut self) {
        // is there a simpler way?
        self.h();
        self.s();
    }
    /// Conjugate the Paulistack with the HS gate ignoring phases.
    pub fn hs(&mut self) {
        // is there a simpler way?
        self.s();
        self.h();
    }
    /// Conjugate the Paulistack with the SHS gate ignoring phases.
    pub fn shs(&mut self) {
        self.x.xor_inplace(&self.z);
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
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
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
        PauliTuple::new_product(self.z.sum_up(filter), self.x.sum_up(filter))
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
