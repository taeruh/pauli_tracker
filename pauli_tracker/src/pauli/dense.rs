use std::fmt::{self, Debug, Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Pauli, tableau_encoding};

/// Pauli encoding into two bits. It is basically an "u2", in terms of a single Pauli
/// operator (without phases).
///
/// The inner storage holds the invariant that it's value is between 0 and 3
/// (inclusive). The encoding follows [tableau_encoding]. Compare
/// [PauliEnum](super::PauliEnum) for a similar representation.
///
/// Unsafe code might rely on that invariant (e.g., via accessing the storage with
/// [Self::storage] and using it to index a pointer), therefore, functions that make it
/// possible to circumvent the invariant are unsafe.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PauliDense {
    storage: u8,
}

impl PauliDense {
    /// Create a [Pauli] from a [u8] without checking the types invariant.
    ///
    /// # Safety
    ///
    /// `storage` < 4 must be valid.
    ///
    /// Use [TryFrom] as checked safe variant.
    pub unsafe fn from_unchecked(storage: u8) -> Self {
        Self { storage }
    }

    /// Get the underlining storage.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// assert_eq!(PauliDense::new_x().storage(), 2);
    /// # }
    /// ```
    pub fn storage(&self) -> u8 {
        self.storage
    }
    /// Get mutable access to the underlining storage.
    ///
    /// # Safety
    ///
    /// Any changes must upheld `storage` < 4.
    pub unsafe fn storage_mut(&mut self) -> &mut u8 {
        &mut self.storage
    }

    /// Directly specify the underlining encoded storage of the Pauli.
    ///
    /// # Panics
    ///
    /// If the input is invalid, i.e., `storage` > 3.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// let mut pauli = PauliDense::I;
    /// pauli.set_storage(1);
    /// assert_eq!(pauli, Pauli::Z);
    /// # }
    /// ```
    pub fn set_storage(&mut self, storage: u8) {
        assert!(storage <= 3);
        self.storage = storage;
    }

    // is mask the correct word here?
    /// Get the X mask of the encoded storage.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// assert_eq!(2, PauliDense::new_x().xmask());
    /// assert_eq!(0, PauliDense::new_z().xmask());
    /// # }
    /// ```
    pub fn xmask(&self) -> u8 {
        self.storage & 2
    }
    /// Get the Z mask of the encoded storage.
    ///
    /// # Examples
    /// ```
    /// # fn main() { #![cfg_attr(coverage_nightly, coverage(off))]
    /// # use pauli_tracker::pauli::{Pauli, PauliDense};
    /// assert_eq!(0, PauliDense::new_x().zmask());
    /// assert_eq!(1, PauliDense::new_z().zmask());
    /// # }
    /// ```
    pub fn zmask(&self) -> u8 {
        self.storage & 1
    }

    /// Apply XOR on the encoded storage of `self` and the storage `other`, updating the
    /// storage of `self` inplace.
    pub fn xor(&mut self, other: Self) {
        self.storage ^= other.storage;
    }

    /// Apply XOR on the encoded storage of `self` and `other`, updating the storage of
    /// `self` inplace.
    pub fn xor_u8(&mut self, other: u8) {
        self.storage ^= other;
    }
}

macro_rules! const_pauli {
    ($($name:ident,)*) => {$(
        const $name: Self = Self { storage: tableau_encoding::$name };
    )*};
}

impl Pauli for PauliDense {
    const_pauli!(I, X, Y, Z,);

    fn new_product(z: bool, x: bool) -> Self {
        Self { storage: z.z() ^ x.x() }
    }

    fn multiply(&mut self, other: Self) {
        self.xor(other);
    }

    fn add(&mut self, other: Self) {
        self.multiply(other);
    }

    fn s(&mut self) {
        self.storage ^= (self.storage & 2) >> 1;
    }
    fn h(&mut self) {
        self.storage ^= (self.storage & 1) << 1;
        self.storage ^= (self.storage & 2) >> 1;
        self.storage ^= (self.storage & 1) << 1;
    }
    fn sh(&mut self) {
        // cf. stack impl
        self.h();
        self.s();
    }
    fn hs(&mut self) {
        // cf. stack impl
        self.s();
        self.h();
    }
    fn shs(&mut self) {
        self.storage ^= (self.storage & 1) << 1;
    }

    fn xpx(&mut self, other: &Self) {
        self.xor_u8(other.xmask());
    }

    fn xpz(&mut self, other: &Self) {
        self.xor_u8(other.zmask() << 1);
    }

    fn zpx(&mut self, other: &Self) {
        self.xor_u8(other.xmask() >> 1);
    }

    fn zpz(&mut self, other: &Self) {
        self.xor_u8(other.zmask());
    }

    fn get_x(&self) -> bool {
        self.storage & 2 != 0
    }

    fn get_z(&self) -> bool {
        self.storage & 1 != 0
    }

    fn set_x(&mut self, x: bool) {
        self.storage &= x.x() | 1;
        self.storage |= x.x();
    }

    fn set_z(&mut self, z: bool) {
        self.storage &= z.z() | 2;
        self.storage |= z.z();
    }

    fn tableau_encoding(&self) -> u8 {
        self.storage
    }
}

use thiserror::Error;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("{0} is not between 0 and 3")]
pub struct InvalidU8(pub u8);

impl TryFrom<u8> for PauliDense {
    type Error = InvalidU8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 3 {
            Err(InvalidU8(value))
        } else {
            Ok(Self { storage: value })
        }
    }
}

impl From<PauliDense> for u8 {
    fn from(value: PauliDense) -> u8 {
        value.storage
    }
}

impl Display for PauliDense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.storage {
            tableau_encoding::I => write!(f, "I"),
            tableau_encoding::Z => write!(f, "Z"),
            tableau_encoding::X => write!(f, "X"),
            tableau_encoding::Y => write!(f, "Y"),
            _ => panic!("unvalid {self:?}"),
        }
    }
}

// just to effectively have an impl bool to make things more convenient here; the
// disadvantage is that we cannot define the methods to be const but we don't need that
trait ResolvePauli {
    fn z(self) -> u8;
    fn x(self) -> u8;
}
impl ResolvePauli for bool {
    fn z(self) -> u8 {
        self as u8
    }
    fn x(self) -> u8 {
        (self as u8) << 1
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;

    #[test]
    fn set_storage() {
        let mut pauli = PauliDense::I;
        for (storage, expected) in [
            (0, PauliDense::I),
            (1, PauliDense::Z),
            (2, PauliDense::X),
            (3, PauliDense::Y),
        ] {
            pauli.set_storage(storage);
            assert_eq!(pauli, expected);
        }
    }
}
