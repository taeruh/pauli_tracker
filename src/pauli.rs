//! Dense encoding for a single Pauli operator.

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

/// Pauli encoding into two bits.
///
/// It is basically an "u2", in terms of a single Pauli operator.

/// The inner storage holds the invariant that it's value is between 0 and 3
/// (inclusive).
///
/// Unsafe code might rely on that invariant (e.g., via accessing the storage with
/// [Self::storage] and using it to index a pointer), therefore, functions that make it
/// possible to circumvent the invariant are unsafe.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
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

trait Sealed {}

impl Pauli {
    pub fn new(x: bool, z: bool) -> Self {
        Self { storage: x.left() ^ z.right() }
    }

    // Safety: hardcoded
    pub fn new_i() -> Self {
        unsafe { Self::from_unchecked(0) }
    }
    pub fn new_x() -> Self {
        unsafe { Self::from_unchecked(2) }
    }
    pub fn new_y() -> Self {
        unsafe { Self::from_unchecked(3) }
    }
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

    pub fn set_storage(&mut self, storage: u8) -> Option<u8> {
        if storage > 3 {
            Some(storage)
        } else {
            self.storage = storage;
            None
        }
    }

    pub fn set_x(&mut self, x: bool) {
        self.storage &= x.left() | 1;
        self.storage |= x.left();
    }
    pub fn set_z(&mut self, z: bool) {
        self.storage &= z.right() | 2;
        self.storage |= z.right();
    }

    pub fn get_x(&self) -> bool {
        self.storage & 2 != 0
    }
    pub fn get_z(&self) -> bool {
        self.storage & 1 != 0
    }

    pub fn h(&mut self) {
        self.storage ^= (self.storage & 1) << 1;
        self.storage ^= (self.storage & 2) >> 1;
        self.storage ^= (self.storage & 1) << 1;
    }
    pub fn s(&mut self) {
        self.storage ^= (self.storage & 2) >> 1;
    }

    pub fn left_mask(&self) -> u8 {
        self.storage & 2
    }
    pub fn right_mask(&self) -> u8 {
        self.storage & 1
    }

    pub fn xor(&mut self, other: Self) {
        self.storage ^= other.storage;
    }
    pub fn xor_u8(&mut self, other: u8) {
        self.storage ^= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_definitions() {
        type Action = fn(&mut Pauli);
        const ACTIONS: [(
            // action
            Action,
            // name for debugging
            &str,
            // result: calculated by hand
            // input for storage: p = 0 1 2 3
            [u8; 4],
        ); 2] = [(Pauli::h, "H", [0, 2, 1, 3]), (Pauli::s, "S", [0, 1, 3, 2])];
        let mut pauli = Pauli::new_i();
        for action in ACTIONS {
            for (input, check) in (0u8..).zip(action.2) {
                assert!(pauli.set_storage(input).is_none());
                (action.0)(&mut pauli);
                assert_eq!(pauli.storage, check, "{}, {}", action.1, input);
            }
        }
    }

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
                    assert!(pauli.set_storage(input).is_none());
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
