//! Encoding of a single Pauli operator.

use std::fmt::{
    Debug,
    Display,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

// just to effectively have an impl bool to make things more convenient here; the
// disadvantage is that we cannot define the methods to be const but we don't need that
trait ResolvePauli {
    fn left(self) -> u8;
    fn right(self) -> u8;
}

impl ResolvePauli for bool {
    #[inline(always)]
    fn left(self) -> u8 {
        (self as u8) << 1
    }
    #[inline(always)]
    fn right(self) -> u8 {
        self as u8
    }
}

/// Pauli encoding into two bits. It is basically an "u2", in terms of a single Pauli
/// operator (without phases). The Pauli is specified as product of X and Z. Note that
/// it is Y = XZ, up to a phase (and (anti)cyclical)
///
/// The inner storage holds the invariant that it's value is between 0 and 3
/// (inclusive). The encoding is as follows: 0 <-> identity, 1 <-> Z, 2 <-> X, 3 <-> Y.
///
/// Unsafe code might rely on that invariant (e.g., via accessing the storage with
/// [Self::storage] and using it to index a pointer), therefore, functions that make it
/// possible to circumvent the invariant are unsafe.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pauli {
    storage: u8,
}

impl TryFrom<u8> for Pauli {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 3 { Err(value) } else { Ok(Self { storage: value }) }
    }
}

impl From<Pauli> for u8 {
    #[inline(always)]
    fn from(value: Pauli) -> u8 {
        value.storage
    }
}

impl Pauli {
    /// Create a the new Pauli (X if x)(Z if z).
    pub fn new(x: bool, z: bool) -> Self {
        Self { storage: x.left() ^ z.right() }
    }

    // Safety: hardcoded
    /// Create a new identity Pauli.
    pub fn new_i() -> Self {
        unsafe { Self::from_unchecked(0) }
    }
    /// Create a new X Pauli.
    pub fn new_x() -> Self {
        unsafe { Self::from_unchecked(2) }
    }
    /// Create a new Y Pauli.
    pub fn new_y() -> Self {
        unsafe { Self::from_unchecked(3) }
    }
    /// Create a new Z Pauli.
    pub fn new_z() -> Self {
        unsafe { Self::from_unchecked(1) }
    }

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

    /// Get access to the underlining storage.
    pub fn storage(&self) -> &u8 {
        &self.storage
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
    pub fn set_storage(&mut self, storage: u8) {
        assert!(storage <= 3);
        self.storage = storage;
    }

    /// Set whether the Pauli producs contains X.
    pub fn set_x(&mut self, x: bool) {
        self.storage &= x.left() | 1;
        self.storage |= x.left();
    }
    /// Set whether the Pauli producs contains Z.
    pub fn set_z(&mut self, z: bool) {
        self.storage &= z.right() | 2;
        self.storage |= z.right();
    }

    /// Get whether the Pauli producs contains X.
    pub fn get_x(&self) -> bool {
        self.storage & 2 != 0
    }
    /// Get whether the Pauli producs contains Z.
    pub fn get_z(&self) -> bool {
        self.storage & 1 != 0
    }

    /// Conjugate the Pauli with the Hadamard Gate.
    pub fn h(&mut self) {
        self.storage ^= (self.storage & 1) << 1;
        self.storage ^= (self.storage & 2) >> 1;
        self.storage ^= (self.storage & 1) << 1;
    }
    /// Conjugate the Pauli with the S Gate.
    pub fn s(&mut self) {
        self.storage ^= (self.storage & 2) >> 1;
    }

    // is mask the correct word here?
    // write examples
    /// Get the X mask of the encoded storage.
    ///
    /// # Examples
    /// ```
    /// # use pauli_tracker::pauli::Pauli;
    /// assert_eq!(2, Pauli::new_x().xmask());
    /// assert_eq!(0, Pauli::new_z().xmask());
    /// ```
    pub fn xmask(&self) -> u8 {
        self.storage & 2
    }
    /// Get the Z mask of the encoded storage.
    /// # Examples
    /// ```
    /// # use pauli_tracker::pauli::Pauli;
    /// assert_eq!(0, Pauli::new_x().zmask());
    /// assert_eq!(1, Pauli::new_z().zmask());
    /// ```
    pub fn zmask(&self) -> u8 {
        self.storage & 1
    }

    /// Apply xor on the encoded storage of `self` and the storage `other`, updating the
    /// storage of `self` inplace.
    pub fn xor(&mut self, other: Self) {
        self.storage ^= other.storage;
    }

    /// Apply xor on the encoded storage of `self` and `other`, updating the storage of
    /// `self` inplace.
    pub fn xor_u8(&mut self, other: u8) {
        self.storage ^= other;
    }
}

impl Display for Pauli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.storage)
    }
}
impl Debug for Pauli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn set() {
        type Action = fn(&mut Pauli, bool);
        const ACTIONS: [(Action, &str, [/* false, false */ [u8; 4]; 2]); 2] = [
            (Pauli::set_x, "set_x", [[0, 1, 0, 1], [2, 3, 2, 3]]),
            (Pauli::set_z, "set_z", [[0, 0, 2, 2], [1, 1, 3, 3]]),
        ];
        let mut pauli = Pauli::new_i();
        for action in ACTIONS {
            for (flag, checks) in [false, true].into_iter().zip(action.2) {
                for (input, check) in (0u8..).zip(checks) {
                    pauli.set_storage(input);
                    (action.0)(&mut pauli, flag);
                    assert_eq!(
                        pauli.storage, check,
                        "{}, {}, {}",
                        action.1, input, flag
                    );
                }
            }
        }
    }
}
