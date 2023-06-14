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
/// Instead of having a vector over [Pauli]s, we separate the X and Z parts into two
/// vectors (cf. [Pauli] for encoding). This enables us to efficiently perform
/// (Clifford) operations on those [PauliVec]s.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PauliVec<T /* : BooleanVector */> {
    /// The bits representing the left qubit on the left-hand side in the tableau
    /// representation, i.e., X
    pub left: T,
    /// The bits representing the left qubit on the left-hand side in the tableau
    /// representation, i.e., Z
    pub right: T,
}

impl<T: BooleanVector> PauliVec<T> {
    /// Create a new empty [PauliVec].
    pub fn new() -> Self {
        Self { left: T::new(), right: T::new() }
    }

    /// Create a [PauliVec] from two strings. `left` (`right`) corresponds to
    /// [PauliVec]s `left` (`right`) field.
    ///
    /// Errors if the strings do not consist only of '0' and '1' characters.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::PauliVec;
    /// assert_eq!(
    ///     PauliVec::<Vec<bool>>::try_from_str("01", "10"),
    ///     Ok(PauliVec::<Vec<bool>> {
    ///         left: vec![false, true],
    ///         right: vec![true, false]
    ///     })
    /// )
    /// # }
    /// ```
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

    /// Create a new [PauliVec] with both sides `left` and `right` initialized with
    /// `len` 0/false elements.
    pub fn zeros(len: usize) -> Self {
        let zero = T::zeros(len);
        Self { left: zero.clone(), right: zero }
    }

    /// Push a new [Pauli] onto the Pauli stack.
    pub fn push(&mut self, pauli: Pauli) {
        self.left.push(pauli.get_x());
        self.right.push(pauli.get_z());
    }

    /// Pop the last element from the stack and return it. Returns [None] if the vector
    /// is empty.
    pub fn pop(&mut self) -> Option<Pauli> {
        match self.left.len().cmp(&self.right.len()) {
            std::cmp::Ordering::Less => Some(Pauli::new(
                false,
                self.right
                    .pop()
                    .expect("shouldn't be possible since right.len > left.len >= 0"),
            )),
            std::cmp::Ordering::Equal => {
                Some(Pauli::new(self.left.pop()?, self.right.pop()?))
            }
            std::cmp::Ordering::Greater => Some(Pauli::new(
                self.left
                    .pop()
                    .expect("shouldn't be possible since left.len > right.len >= 0"),
                false,
            )),
        }
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

    /// Multiply the Paulis, i.e., summing them up mod 2 in the tableau representation,
    /// with a `filter` while neglecting any phases. An element `e` is filtered if
    /// `filter[i] = true` where `i` is `e`'s index in
    /// [iter_vals](BooleanVector::iter_vals). Compare [BooleanVector::sum_up].
    ///
    /// # Panics
    /// Panics if `filter.len()` < number of Paulis
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::{pauli::{Pauli, PauliVec}, boolean_vector::BooleanVector};
    /// let paulis = [
    ///     Pauli::new_x(),
    ///     Pauli::new_y(),
    ///     Pauli::new_z(),
    ///     Pauli::new_x(),
    ///     Pauli::new_y(),
    ///     Pauli::new_z(),
    /// ]
    /// .into_iter()
    /// .collect::<PauliVec<Vec<bool>>>();
    /// let filter = [true, true, true, false, false, false];
    /// assert_eq!(paulis.sum_up(&filter), Pauli::new_i());
    /// # }
    /// ```
    pub fn sum_up(&self, filter: &[bool]) -> Pauli {
        // Safety: BooleanVector::sum_up returns u8 <= 1
        unsafe {
            Pauli::from_unchecked(
                self.right.sum_up(filter) + self.left.sum_up(filter) * 2,
            )
        }
    }
}

impl<T: BooleanVector> FromIterator<Pauli> for PauliVec<T> {
    fn from_iter<I: IntoIterator<Item = Pauli>>(iter: I) -> Self {
        let mut ret = PauliVec::new();
        for pauli in iter {
            ret.push(pauli);
        }
        ret
    }
}
