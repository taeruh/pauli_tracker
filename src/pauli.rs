//! Dense encoding for a single Pauli operator.

/// Pauli encoding into two bits.
///
/// It is basically an "u2". The inner storage holds the invariant that it's value is
/// between 0 and 3 (inclusive).
///
/// Unsafe code might rely on that invariant (e.g., via accessing the storage with
/// [Self::storage] and using it to index a pointer), therefore, functions that make it
/// possible to circumvent the invariant are unsafe.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Pauli {
    storage: u8,
}

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

impl TryFrom<u8> for Pauli {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 3 { Err(value) } else { Ok(Self { storage: value }) }
    }
}

impl Pauli {
    pub fn new(x: bool, z: bool) -> Self {
        Self { storage: x.left() ^ z.right() }
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

    pub fn set_x(&mut self, x: bool) {
        self.storage ^= x.left();
    }

    pub fn set_z(&mut self, z: bool) {
        self.storage ^= z.right();
    }

    pub fn and(&mut self, other: &Self) {
        self.storage &= other.storage;
    }

    pub fn or(&mut self, other: &Self) {
        self.storage |= other.storage;
    }

    pub fn xor(&mut self, other: &Self) {
        self.storage ^= other.storage;
    }

    pub fn x(&self) -> bool {
        self.storage & 2 != 0
    }

    pub fn z(&self) -> bool {
        self.storage & 1 != 0
    }

    // ...
}

