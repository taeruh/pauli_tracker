use std::mem;

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::single::Pauli;
use crate::boolean_vector::BooleanVector;

/// Multiple encoded Paulis compressed into two [BooleanVector]s.
///
/// Instead of having a vector over [Pauli](super::Pauli)s, we separate the X and Z
/// parts into two vectors. This enables us to efficiently perform (Clifford) operations
/// on those [PauliVec]s.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PauliVec<T /* : BooleanVector */> {
    // the bit representing the left qubit on the left-hand side in the tableau
    // representation, i.e., X
    pub left: T,
    // right-hand side, i.e., Z
    pub right: T,
}

impl<T: BooleanVector> PauliVec<T> {
    pub fn new() -> Self {
        Self { left: T::new(), right: T::new() }
    }

    pub fn try_from_str(left: &str, right: &str) -> Result<Self, String> {
        fn to_bool(c: char) -> Result<bool, String> {
            match c.to_digit(2) {
                Some(d) => Ok(d == 1),
                None => Err(format!("{} is not a valid binary", c)),
            }
        }
        Ok(Self {
            left: left.chars().flat_map(to_bool).collect(),
            right: right.chars().flat_map(to_bool).collect(),
        })
    }

    pub fn zeros(len: usize) -> Self {
        let zero = T::zeros(len);
        Self { left: zero.clone(), right: zero }
    }

    pub fn push(&mut self, pauli: Pauli) {
        self.left.push(pauli.get_x());
        self.right.push(pauli.get_z());
    }

    pub fn pop_or_false(&mut self) -> Pauli {
        let l = self.left.pop().unwrap_or(false);
        let r = self.right.pop().unwrap_or(false);
        Pauli::new(l, r)
    }

    // we can define the action of local gates

    // Pauli gates don't do anything; we just include them for completeness and since it
    // might be more convenient to have them on the caller side
    /// Apply Pauli X, note that it is just the identity
    #[inline(always)]
    pub fn x(&self) {}
    /// Apply Pauli Z, note that it is just the identity
    #[inline(always)]
    pub fn z(&self) {}
    /// Apply Pauli Y, note that it is just the identity
    #[inline(always)]
    pub fn y(&self) {}

    /// Apply Hadamard
    #[inline]
    pub fn h(&mut self) {
        mem::swap(&mut self.left, &mut self.right);
    }

    /// Apply Phase S
    #[inline]
    pub fn s(&mut self) {
        // self.right.xor(&self.left);
        self.right.xor_inplace(&self.left);
    }
}
