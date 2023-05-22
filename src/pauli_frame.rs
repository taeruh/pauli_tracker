use std::mem;

use bit_vec::BitVec;

pub mod storage;

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

/// Multiple encoded Paulis compressed into two [BitVec]s.
// each Pauli can be described by two bits (neglecting phases)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct PauliVec {
    // the bit representing the left qubit on the left-hand side in the tableau
    // representation, i.e., X
    pub left: BitVec,
    // right-hand side, i.e., Z
    pub right: BitVec,
}

// that's actually not that handy; it's better to have more specialized froms
impl<T, B> From<T> for PauliVec
where
    T: IntoIterator<Item = B>,
    B: Into<(bool, bool)>,
{
    fn from(value: T) -> Self {
        let mut ret = PauliVec::new();
        for (l, r) in value.into_iter().map(|b| b.into()) {
            ret.push(l, r);
        }
        ret
    }
}

impl PauliVec {
    pub fn new() -> Self {
        Self {
            left: BitVec::new(),
            right: BitVec::new(),
        }
    }

    pub fn zeros(len: usize) -> Self {
        let zero = zero_bitvec(len);
        Self { left: zero.clone(), right: zero }
    }

    pub fn push(&mut self, x: bool, z: bool) {
        self.left.push(x);
        self.right.push(z);
    }

    pub fn pop(&mut self) -> Option<Pauli> {
        let l = self.left.pop()?;
        let r = self.right.pop()?;
        Some(Pauli::new(l, r))
    }

    pub fn clear(&mut self) {
        self.left.clear();
        self.right.clear();
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
        mem::swap(
            // Safety:
            // we don't do anything with the storage itself, so we should be good
            unsafe { self.left.storage_mut() },
            unsafe { self.right.storage_mut() },
        );
    }

    /// Apply Phase S
    #[inline]
    pub fn s(&mut self) {
        self.right.xor(&self.left);
    }
}

// not sure whether that is the fastest way
fn zero_bitvec(len: usize) -> BitVec {
    let rest = len % 8;
    let bytes = (len - rest) / 8;
    let mut ret = BitVec::from_bytes(&vec![0; bytes]);
    for _ in 0..rest {
        ret.push(false)
    }
    ret
}

/// A vector describing an encoded Pauli string, for example, one frame of [Frames] (via
/// [Frames::pop_frame]). The `usize` element is the qubit index of the `Pauli` However,
/// importantly note, that it is not optimal to build arrays with PauliStrings on the
/// minor access. The library is build to use implementors of [PauliStorage], which
/// should have [PauliVec]s on the minor array axis, as workhorses. This vector should
/// be mainly used to analyze single Pauli strings.
pub type PauliString = Vec<(usize, Pauli)>;

/// This trait describes the functionality that a storage of [PauliVec]s must provide to
/// be used as storage for [Frames].
// pub trait PauliStorage: IntoIterator<Item = (usize, PauliVec)> {
pub trait PauliStorage: IntoIterator<Item = (usize, PauliVec)> {
    type IterMut<'a>: Iterator<Item = (usize, &'a mut PauliVec)>
    where
        Self: 'a;

    type Iter<'a>: Iterator<Item = (usize, &'a PauliVec)>
    where
        Self: 'a;

    fn insert_pauli(&mut self, qubit: usize, pauli: PauliVec) -> Option<PauliVec>;
    fn remove_pauli(&mut self, qubit: usize) -> Option<PauliVec>;
    fn get(&self, qubit: usize) -> Option<&PauliVec>;
    fn get_mut(&mut self, qubit: usize) -> Option<&mut PauliVec>;
    fn iter(&self) -> Self::Iter<'_>;
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
    fn init(num_qubits: usize) -> Self;
}

/// A container of multiple Pauli frames, using a generic `Storage` type (that
/// implements [PauliStorage] if it shall be useful) as internal storage. The type
/// implements the core functionality to track the Pauli frames through a Clifford
/// circuit. As example view the documentation of [Circuit](crate::circuit::Circuit).
/// The explicit storage type should have the [PauliVec]s on it's minor axis (this is
/// more or less enforced by [PauliStorage]). The module [storage] provides some
/// compatible storage types.
#[derive(Clone, Debug, Default)]
pub struct Frames<Storage /* : PauliStorageMap */> {
    storage: Storage,
    frames_num: usize,
}

impl<Storage> Frames<Storage> {
    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn frames_num(&self) -> usize {
        self.frames_num
    }

    pub fn into_storage(self) -> Storage {
        self.storage
    }

    // Pauli gates don't do anything; we just include them for completeness and since it
    // might be more convenient to have them on the caller side
    /// Apply Pauli X, note that it is just the identity.
    #[inline(always)]
    pub fn x(&self, _: usize) {}
    /// Apply Pauli Z, note that it is just the identity.
    #[inline(always)]
    pub fn z(&self, _: usize) {}
    /// Apply Pauli Y, note that it is just the identity.
    #[inline(always)]
    pub fn y(&self, _: usize) {}
}

impl<Storage: PauliStorage> Frames<Storage> {
    pub fn init(num_qubits: usize) -> Self {
        Self {
            storage: Storage::init(num_qubits),
            frames_num: 0,
        }
    }

    pub fn new_qubit(&mut self, qubit: usize) -> Option<usize> {
        self.storage
            .insert_pauli(qubit, PauliVec::zeros(self.frames_num))
            .map(|_| qubit)
    }

    pub fn track_pauli(&mut self, qubit: usize, pauli: Pauli) {
        for (i, p) in self.storage.iter_mut() {
            if i == qubit {
                p.push(pauli.x(), pauli.z());
            } else {
                p.push(false, false);
            }
        }
        self.frames_num += 1;
    }

    pub fn track_pauli_string(&mut self, string: PauliString) {
        for (_, p) in self.storage.iter_mut() {
            p.push(false, false);
        }
        self.frames_num += 1;
        for (i, p) in string {
            match self.storage.get_mut(i) {
                Some(v) => v.left.set(self.frames_num - 1, p.x()),
                None => continue,
            }
            self.storage
                .get_mut(i)
                .expect("bug; already checked above")
                .right
                .set(self.frames_num - 1, p.z());
        }
    }

    pub fn pop_frame(&mut self) -> Option<PauliString> {
        let mut ret = Vec::new();
        for (i, p) in self.storage.iter_mut() {
            ret.push((i, p.pop()?));
        }
        self.frames_num -= 1;
        Some(ret)
    }

    /// Safety:
    /// The referred PauliVecs behind `first` and `second` have to be different,
    /// otherwise we would return two mutable references to the same object which would
    /// be unsound and can cause might cause *[undefined behavior] (noalias violation)*.
    ///
    /// [undefined behavior]:
    /// https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    unsafe fn get_two_mut_unchecked(
        &mut self,
        first: usize,
        second: usize,
    ) -> (&mut PauliVec, &mut PauliVec) {
        // this causes miri to error when run with stacked-borrows; however, note that
        // the stacked-borrow rules are experimental (at the time of writing this
        // comment here); we can set MIRIFLAGS="-Zmiri-tree-borrows" to use the
        // tree-borrow instead of the stacked-borrow rules, however they are even more
        // experimental, but with them, we don't get an error; I didn't look into these
        // borrow rules very deeply, however, I think that what we are doing here is
        // okay (also note that we cannot circumvent the stack-borrow error by using
        // pointers all the way down when we call something like xor in cx; we'd just
        // get the same error directly in cx when creating the references from the
        // pointers, so that we can use them in xor)
        (
            unsafe { &mut *(self.storage.get_mut(first).unwrap() as *mut PauliVec) },
            unsafe { &mut *(self.storage.get_mut(second).unwrap() as *mut PauliVec) },
        )
    }

    fn get_two_mut(
        &mut self,
        first: usize,
        second: usize,
    ) -> (&mut PauliVec, &mut PauliVec) {
        // Safety: checked below in the assert
        let (f, s) = unsafe { self.get_two_mut_unchecked(first, second) };
        assert_ne!(f as *mut PauliVec as usize, s as *mut PauliVec as usize);
        (f, s)
    }

    /// Apply the Hadamard gate.
    pub fn h(&mut self, qubit: usize) {
        self.storage
            .get_mut(qubit)
            .unwrap_or_else(|| panic!("qubit {qubit} does not exist"))
            .h();
    }

    /// Apply the Phase gate S.
    pub fn s(&mut self, qubit: usize) {
        self.storage
            .get_mut(qubit)
            .unwrap_or_else(|| panic!("qubit {qubit} does not exist"))
            .s();
    }

    /// Apply the Control X (Control Not) gate.
    pub fn cx(&mut self, control: usize, target: usize) {
        let (c, t) = self.get_two_mut(control, target);
        t.left.xor(&c.left);
        c.right.xor(&t.right);
    }

    /// Apply the Control Z gate.
    pub fn cz(&mut self, qubit_a: usize, qubit_b: usize) {
        let (a, b) = self.get_two_mut(qubit_a, qubit_b);
        a.right.xor(&b.left);
        b.right.xor(&a.left);
    }

    /// Perform an unspecified measurement. This removes the according qubit from being
    /// tracked.
    ///
    /// Returns the according [PauliVec] if it is a valid measurement, i.e., the qubit
    /// existed.
    pub fn measure(&mut self, qubit: usize) -> Option<PauliVec> {
        self.storage.remove_pauli(qubit)
    }

    pub fn measure_and_store(&mut self, qubit: usize, storage: &mut impl PauliStorage) {
        storage.insert_pauli(qubit, self.measure(qubit).unwrap());
    }

    pub fn measure_and_store_all(self, storage: &mut impl PauliStorage) {
        for (i, p) in self.storage.into_iter() {
            storage.insert_pauli(i, p);
        }
    }

    pub fn clear_all(&mut self) {
        for (_, p) in self.storage.iter_mut() {
            p.clear()
        }
        self.frames_num = 0;
    }
}

pub fn sort_pauli_storage(storage: &impl PauliStorage) -> Vec<(usize, &PauliVec)> {
    let mut ret = storage.iter().collect::<Vec<(usize, &PauliVec)>>();
    ret.sort_by_key(|(i, _)| *i);
    ret
}

pub fn into_sorted_pauli_storage(storage: impl PauliStorage) -> Vec<(usize, PauliVec)> {
    let mut ret = storage
        .into_iter()
        .map(|(i, p)| (i, p))
        .collect::<Vec<(usize, PauliVec)>>();
    ret.sort_by_key(|(i, _)| *i);
    ret
}

#[cfg(test)]
mod tests {
    use super::{
        storage::*,
        *,
    };

    // we only check the basic functionality here, more complicated circuits are tested
    // in [super::circuit] to test the tracker and the circuit at once

    mod gate_definition_check {
        use super::*;

        // maybe todo: in the following functions there's a pattern behind how we encode
        // one-qubit and two-qubit actions, it's like a "TwoBitVec"; one could probably
        // maybe implement that in connection with [Pauli]

        #[test]
        fn one_qubit() {
            // pauli p = ab in binary; encoding: x = a, z = b
            type Action = dyn Fn(&mut Frames<FullMap>, usize);
            const GATES: [(
                // action
                &Action,
                // name for debugging
                &str,
                // result: calculated by hand
                // encoded input: p = 3 2 1 0
                [u8; 4],
            ); 5] = [
                (&|f, b| Frames::x(f, b), "X", [3, 2, 1, 0]),
                (&|f, b| Frames::z(f, b), "Z", [3, 2, 1, 0]),
                (&|f, b| Frames::y(f, b), "Y", [3, 2, 1, 0]),
                (&Frames::h, "H", [3, 1, 2, 0]),
                (&Frames::s, "S", [2, 3, 1, 0]),
            ];

            for action in GATES {
                let mut frames = Frames::<FullMap>::default();
                frames.new_qubit(0);
                for pauli in 0..4 {
                    frames
                        .track_pauli_string(vec![(0, Pauli::try_from(pauli).unwrap())]);
                }
                (action.0)(&mut frames, 0);
                for (check, input) in (0..4).zip((0..4).rev()) {
                    assert_eq!(
                        *frames.pop_frame().unwrap().get(0).unwrap().1.storage(),
                        action.2[check],
                        "{}, {}",
                        action.1,
                        input
                    );
                }
            }
        }

        #[test]
        fn two_qubit() {
            // double-pauli p = abcd in binary;
            // encoding: x_0 = a, z_0 = b, x_1 = c, z_2 = d
            type Action = dyn Fn(&mut Frames<FullMap>, usize, usize);
            const GATES: [(
                // action
                &Action,
                // name for debugging
                &str,
                // result: calculated by hand
                // encoded input: p = 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0
                [u8; 16],
            ); 2] = [
                (
                    &Frames::cx, // left->control, right->target
                    "CX",
                    [9, 12, 11, 14, 13, 8, 15, 10, 3, 6, 1, 4, 7, 2, 5, 0],
                ),
                (
                    &Frames::cz,
                    "CZ",
                    [10, 11, 12, 13, 14, 15, 8, 9, 3, 2, 5, 4, 7, 6, 1, 0],
                ),
            ];

            // masks to decode p in 0..16 into two paulis and vice versa
            const FIRST: u8 = 12;
            const FIRST_SHIFT: u8 = 2;
            const SECOND: u8 = 3;

            for action in GATES {
                let mut frames = Frames::<FullMap>::default();
                frames.new_qubit(0);
                frames.new_qubit(1);
                for pauli in 0..16 {
                    frames.track_pauli_string(vec![
                        (0, Pauli::try_from((pauli & FIRST) >> FIRST_SHIFT).unwrap()),
                        (1, Pauli::try_from(pauli & SECOND).unwrap()),
                    ]);
                }
                (action.0)(&mut frames, 0, 1);
                for (check, input) in (0..16).zip((0..16).rev()) {
                    let frame = frames.pop_frame().unwrap();
                    let mut result = 0_u8;
                    for (i, p) in frame {
                        if i == 0 {
                            result += p.storage() << FIRST_SHIFT
                        } else if i == 1 {
                            result += p.storage()
                        }
                    }
                    assert_eq!(result, action.2[check], "{}, {}", action.1, input);
                }
            }
        }
    }
}
