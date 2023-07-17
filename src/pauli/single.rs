use std::fmt::{
    self,
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
/// operator (without phases).
///
/// The Pauli is specified as product of X and Z. Note that
/// it is Y = XZ, up to a phase (and (anti)cyclical)
///
/// The inner storage holds the invariant that it's value is between 0 and 3
/// (inclusive). The encoding is as follows: 0 <-> identity, 1 <-> Z, 2 <-> X, 3 <-> Y
/// (cf. [encoding](super::encoding). This encoding is often used under the name tableau
/// representation.
///
/// Unsafe code might rely on that invariant (e.g., via accessing the storage with
/// [Self::storage] and using it to index a pointer), therefore, functions that make it
/// possible to circumvent the invariant are unsafe.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pauli {
    storage: u8,
}

macro_rules! const_pauli {
    ($(($name:ident, $value:expr, $doc:literal),)*) => {$(
        /// Encoded Pauli
        #[doc = $doc]
        /// .
        pub const $name: Pauli = Pauli { storage: $value };
    )*};
}

const_pauli!(
    (PAULI_I, 0, "I"),
    (PAULI_X, 2, "X"),
    (PAULI_Y, 3, "Y"),
    (PAULI_Z, 1, "%"),
);

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

macro_rules! new {
    ($(($name:ident, $gate:ident),)*) => {$(
        /// Create a new
        #[doc = stringify!($gate)]
        /// Pauli.
        #[inline]
        pub fn $name() -> Self {
            // Safety: hardcoded in super::encoding
            unsafe { Self::from_unchecked(super::encoding::$gate) }
        }
    )*};
}

impl Pauli {
    /// Create a the new Pauli (X if x)(Z if z).
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::Pauli;
    /// assert_eq!(Pauli::new(false, false), Pauli::new_i());
    /// assert_eq!(Pauli::new(false, true), Pauli::new_z());
    /// assert_eq!(Pauli::new(true, false), Pauli::new_x());
    /// assert_eq!(Pauli::new(true, true), Pauli::new_y());
    /// # }
    pub fn new(x: bool, z: bool) -> Self {
        Self { storage: x.left() ^ z.right() }
    }

    new!((new_i, I), (new_x, X), (new_y, Y), (new_z, Z),);

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
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::Pauli;
    /// assert_eq!(Pauli::new_x().storage(), 2);
    /// # }
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
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::Pauli;
    /// let mut pauli = Pauli::new_i();
    /// pauli.set_storage(1);
    /// assert_eq!(pauli, Pauli::new_z());
    /// # }
    pub fn set_storage(&mut self, storage: u8) {
        assert!(storage <= 3);
        self.storage = storage;
    }

    /// Set whether the Pauli products contains X.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::Pauli;
    /// let mut pauli = Pauli::new_y();
    /// pauli.set_x(false);
    /// assert_eq!(pauli, Pauli::new_z());
    /// # }
    pub fn set_x(&mut self, x: bool) {
        self.storage &= x.left() | 1;
        self.storage |= x.left();
    }

    /// Set whether the Pauli products contains Z.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::Pauli;
    /// let mut pauli = Pauli::new_y();
    /// pauli.set_z(false);
    /// assert_eq!(pauli, Pauli::new_x());
    /// # }
    pub fn set_z(&mut self, z: bool) {
        self.storage &= z.right() | 2;
        self.storage |= z.right();
    }

    /// Get whether the Pauli products contains X.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::Pauli;
    /// let pauli = Pauli::new_y();
    /// assert_eq!(pauli.get_x(), true);
    /// # }
    pub fn get_x(&self) -> bool {
        self.storage & 2 != 0
    }

    /// Get whether the Pauli products contains Z.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::Pauli;
    /// let pauli = Pauli::new_y();
    /// assert_eq!(pauli.get_z(), true);
    /// # }
    pub fn get_z(&self) -> bool {
        self.storage & 1 != 0
    }

    /// Conjugate the Pauli with the Hadamard Gate ignoring phases.
    pub fn h(&mut self) {
        self.storage ^= (self.storage & 1) << 1;
        self.storage ^= (self.storage & 2) >> 1;
        self.storage ^= (self.storage & 1) << 1;
    }
    /// Conjugate the Pauli with the S Gate ignoring phases.
    pub fn s(&mut self) {
        self.storage ^= (self.storage & 2) >> 1;
    }

    // is mask the correct word here?
    // write examples
    /// Get the X mask of the encoded storage.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::Pauli;
    /// assert_eq!(2, Pauli::new_x().xmask());
    /// assert_eq!(0, Pauli::new_z().xmask());
    /// # }
    /// ```
    pub fn xmask(&self) -> u8 {
        self.storage & 2
    }
    /// Get the Z mask of the encoded storage.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// # use pauli_tracker::pauli::Pauli;
    /// assert_eq!(0, Pauli::new_x().zmask());
    /// assert_eq!(1, Pauli::new_z().zmask());
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

impl Display for Pauli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.storage {
            0 => write!(f, "I"),
            1 => write!(f, "Z"),
            2 => write!(f, "X"),
            3 => write!(f, "Y"),
            _ => panic!("unvalid {self:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

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

    // gate conjugation is tested in live_vector
}
