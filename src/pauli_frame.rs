use std::mem;

use bit_vec::BitVec;

pub mod storage;

#[derive(Clone, Copy, Debug, Default)]
pub struct Pauli {
    storage: u8,
}

// just to effectively have an impl bool to make things more convenient; the
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

    /// # Safety
    ///
    /// `storage` < 4 must be valid, otherwise using this Pauli might cause undefined
    /// behavior.
    ///
    /// Use [TryFrom] as checked safe variant.
    pub unsafe fn from_raw(storage: u8) -> Self {
        Self { storage }
    }

    pub fn storage(&self) -> &u8 {
        &self.storage
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
}

// each Pauli can be described by two bits (neglecting phases)
#[derive(Clone, Debug, Default)]
pub struct PauliVec {
    // the bit representing the left qubit on the left-hand side in the tableau
    // representation, i.e., X
    pub left: BitVec,
    // right-hand side, i.e., Z
    pub right: BitVec,
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
    /// Apply Pauli X, note that it is just the identity.
    #[inline(always)]
    pub fn x(&self) {}
    /// Apply Pauli Z, note that it is just the identity.
    #[inline(always)]
    pub fn z(&self) {}
    /// Apply Pauli Y, note that it is just the identity.
    #[inline(always)]
    pub fn y(&self) {}

    // hadamard just swaps x with z
    #[inline]
    pub fn h(&mut self) {
        mem::swap(
            // Safety:
            // we don't do anything with the storage itself, so we are good
            unsafe { self.left.storage_mut() },
            unsafe { self.right.storage_mut() },
        );
    }

    #[inline]
    pub fn s(&mut self) {
        todo!()
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

pub type PauliString = Vec<(usize, Pauli)>;

pub trait PauliStorageMap: IntoIterator<Item = (usize, PauliVec)> {
    type IterMut<'a>: Iterator<Item = (&'a usize, &'a mut PauliVec)>
    where
        Self: 'a;

    type Iter<'a>: Iterator<Item = (&'a usize, &'a PauliVec)>
    where
        Self: 'a;

    fn insert_pauli(&mut self, qubit: usize, pauli: PauliVec) -> Option<PauliVec>;
    fn remove_pauli(&mut self, qubit: usize) -> Option<PauliVec>;
    fn get(&self, qubit: usize) -> Option<&PauliVec>;
    fn get_mut(&mut self, qubit: usize) -> Option<&mut PauliVec>;
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
    fn iter(&self) -> Self::Iter<'_>;
    fn init(num_qubits: usize) -> Self;
}

#[derive(Clone, Debug, Default)]
pub struct Frames<Storage /* : PauliStorageMap */> {
    pub storage: Storage,
    pub frames_num: usize,
}

impl<T> Frames<T> {
    pub fn new(storage: T, frames_num: usize) -> Self {
        Self { storage, frames_num }
    }
}

impl<Storage: PauliStorageMap> Frames<Storage> {
    pub fn init(num_qubits: usize) -> Self {
        Self {
            storage: Storage::init(num_qubits),
            frames_num: 0,
        }
    }

    // pub fn new_qubit(&mut self, qubit: usize) -> Option<usize> {
    pub fn new_qubit(&mut self, qubit: usize) -> Option<usize> {
        self.storage
            .insert_pauli(qubit, PauliVec::zeros(self.frames_num))
            .map(|_| qubit)
    }

    pub fn track_pauli(&mut self, qubit: usize, pauli: Pauli) {
        for (i, p) in self.storage.iter_mut() {
            if *i == qubit {
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
                .expect("bug, checked already above")
                .right
                .set(self.frames_num - 1, p.z());
        }
    }

    pub fn pop_frame(&mut self) -> Option<PauliString> {
        let mut ret = Vec::new();
        for (&i, p) in self.storage.iter_mut() {
            ret.push((i, p.pop()?));
        }
        self.frames_num -= 1;
        Some(ret)
    }

    /// Safety:
    /// [first] and [second] must not be the same as this might cause undefined behavior
    #[inline]
    unsafe fn get_two_mut_unchecked(
        &mut self,
        first: usize,
        second: usize,
    ) -> (&mut PauliVec, &mut PauliVec) {
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
        assert_ne!(first, second, "first and second have to be different");
        // Safety: we checked that first neq second
        unsafe { self.get_two_mut_unchecked(first, second) }
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

    pub fn cnot(&mut self, control: usize, target: usize) {
        let (c, t) = self.get_two_mut(control, target);
        t.left.xor(&c.left);
        c.right.xor(&t.right);
    }

    /// Perform an unspecified measurement. This removes the according qubit from being
    /// tracked.
    ///
    /// Returns the according [PauliVec] if it is a valid measurement, i.e., the qubit
    /// existed.
    pub fn measure(&mut self, qubit: usize) -> Option<PauliVec> {
        self.storage.remove_pauli(qubit)
    }

    pub fn measure_and_store(
        &mut self,
        qubit: usize,
        storage: &mut impl PauliStorageMap,
    ) {
        storage.insert_pauli(qubit, self.measure(qubit).unwrap());
    }

    pub fn measure_and_store_all(self, storage: &mut impl PauliStorageMap) {
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

pub fn sort(storage: &impl PauliStorageMap) -> Vec<(usize, &PauliVec)> {
    let mut ret = storage
        .iter()
        .map(|(i, p)| (*i, p))
        .collect::<Vec<(usize, &PauliVec)>>();
    ret.sort_by_key(|(i, _)| *i);
    ret
}

#[cfg(test)]
mod tests {
    use super::{
        storage::*,
        *,
    };

    #[test]
    fn gate_definition_check() {
        // double-pauli p = abcd in binary; encoding: x_0 = a, z_0 = b, x_1 = c, z_2 = d
        type Action = dyn Fn(&mut Frames<SmallPauliStorage>, usize, usize);
        const GATES: [(
            // action
            &Action,
            // name for debugging
            &str,
            // result: calculated by hand
            // encoded order input: p in 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0
            [u8; 16],
        ); 4] = [
            (
                &|f, b, _| Frames::x(f, b),
                "X",
                [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            ),
            (
                &|f, b, _| Frames::z(f, b),
                "Z",
                [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            ),
            (
                &|f, b, _| Frames::h(f, b),
                "H",
                [15, 14, 13, 12, 7, 6, 5, 4, 11, 10, 9, 8, 3, 2, 1, 0],
            ),
            (
                &Frames::cnot, // left->control, right->target
                "CNOT",
                [9, 12, 11, 14, 13, 8, 15, 10, 3, 6, 1, 4, 7, 2, 5, 0],
            ),
        ];

        // masks to decode p in 0..16 into two paulis and vice versa
        const FIRST: u8 = 12;
        const FIRST_SHIFT: u8 = 2;
        const SECOND: u8 = 3;

        for action in GATES {
            let mut frames = Frames::<SmallPauliStorage>::default();
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

    // #[test]
    fn identities() {
        let mut frames = Frames::<SmallPauliStorage>::default();
        let mut storage = SmallPauliStorage::default();
        frames.new_qubit(0);
        frames.track_pauli(0, Pauli::new(true, false));
        frames.new_qubit(1);
        frames.cnot(0, 1);
        // frames.track_pauli(1, true, false);
        // frames.new_qubit(2);
        // frames.track_pauli(1, true, false);
        // frames.measure_and_store(1, &mut storage);
        // frames.new_qubit(4);
        // frames.h(1);
        frames.measure_and_store_all(&mut storage);
        println!("{:#?}", sort(&storage));
    }

    // #[test]
    // fn test() {
    //     let mut a = BitVec::new();
    //     a.push(true);
    //     a.push(true);
    //     println!("{:?}", a);
    //     a.truncate(1);
    //     println!("{:?}", a);
    // }
}
