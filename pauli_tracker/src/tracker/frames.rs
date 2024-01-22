/*!
Track Pauli gates when constructing a circuit.

This module provides the [Frames] Pauli tracker. Each new tracked Pauli introduces a new
frame on the qubits, for example corresponding to a measurement, with the tracked Pauli
on the qubit where it has been initialized and identities on all other qubits. The
Clifford gates act on this frame, according to the conjugation rules, causing the
tracked Pauli to being copied, moved, swaped, ... within the frame. The frames of
multiple tracked Paulis are stacked up, not effectiving each other; this is the main
difference to the [Live] tracker, which adds up the frames.

When using this tracker, you probably only want to track Paulis induced by
non-deterministic measurements via the `track_x/y/z` methods (and track fixed Paulis with
the [Live] tracker if needed). This is because this tracker is used to analyze the
dependencies induced by the measurements in form of a time ordering. To track Paulis
during the actual execution of a circuit, the [Live] tracker is more useful.

[Live]: super::live::Live
*/

use std::mem;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{MissingBit, PauliString, Tracker};
use crate::{
    boolean_vector::BooleanVector,
    collection::{Base, Full, Init, IterableBase},
    pauli::{Pauli, PauliStack, PauliTuple},
};

pub mod induced_order;

/// A container of multiple Pauli frames that implements [Tracker].
///
/// Compare the [module documentation](super::frames). To be useful, the generic
/// `Storage` type should implement [IterableBase] (or better [Full]). The explicit
/// storage type should have the [PauliStack]s on it's minor axis (this is more or less
/// enforced by the [collection] traits). The [collection] module provides some
/// compatible storage types.
///
/// [collection]: crate::collection
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Frames<Storage> {
    storage: Storage,
    frames_num: usize,
}

/// The Error when one overwrites a qubit's Pauli stack.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("the Pauli stack for bit {bit} has been overwritten")]
pub struct OverwriteStack<T> {
    /// The bit.
    pub bit: usize,
    /// The stack that was overwritten.
    pub stack: PauliStack<T>,
}

impl<Storage> AsRef<Storage> for Frames<Storage> {
    fn as_ref(&self) -> &Storage {
        &self.storage
    }
}

/// The Error when one tries to measure a qubit and store it stacks in another storage,
/// as in [measure_and_store](Frames::measure_and_store).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum MoveError<T> {
    /// See [OverwriteStack].
    #[error(transparent)]
    OverwriteStack(#[from] OverwriteStack<T>),
    /// See [MissingBit].
    #[error(transparent)]
    MissingBit(#[from] MissingBit),
}

#[doc = non_semantic_default!()]
impl<T: Default> Default for MoveError<T> {
    fn default() -> Self {
        Self::MissingBit(MissingBit::default())
    }
}

impl<S> Frames<S> {
    /// Create a new [Frames] instance with a given storage and number of frames.
    ///
    /// It does not check whether the storage is compatible with the number of frames.
    /// If it is not, i.e., if not all stacks have the same length and this length is
    /// `frames_num`, using the create instance might result in logic errors and panics.
    /// This function is mainly useful to put some parts of a frames storage into a new
    /// one.
    pub fn new_unchecked(storage: S, frames_num: usize) -> Self {
        Self { storage, frames_num }
    }

    /// Reference the underlining storage.
    pub fn as_storage(&self) -> &S {
        &self.storage
    }

    /// Convert the object into the underlining storage.
    pub fn into_storage(self) -> S {
        self.storage
    }

    /// Get the number of tracked frames, i.e., the length of the stacks in the inner
    /// storage.
    pub fn frames_num(&self) -> usize {
        self.frames_num
    }
}

impl<S: Init> Init for Frames<S> {
    fn init(len: usize) -> Self {
        Self {
            storage: S::init(len),
            frames_num: 0,
        }
    }
}

impl<S: Base<TB = T>, T> Frames<S> {
    /// Returns a reference to `bit`s Pauli stack; [None] if `bit` is not present.
    pub fn get(&self, bit: usize) -> Option<&T> {
        self.storage.get(bit)
    }
}

macro_rules! single {
    ($($name:ident,)*) => {$(
        fn $name(&mut self, bit: usize) {
            unwrap_get_mut!(self.storage, bit, stringify!($name)).$name()
        }
    )*};
}

macro_rules! movements {
    ($((
        $name:ident,
        $from_side:ident,
        $to_side:ident,
        $from_doc:literal,
        $to_doc:literal
    ),)*) => {$(
        /// "Move" the
        #[doc=$from_doc]
        /// Pauli stack from the `origin` qubit to to `destination` qubit, transforming
        /// it to an
        #[doc=$to_doc]
        /// stack. "Moving" means literally removing the stack from `origin` memory and
        /// adding (mod 2) on `destination`. Because of that, this operation should only
        /// be used directly before the `origin` qubit is measured; otherwise it breaks
        /// the logic of other methods and might cause panics.
        fn $name(&mut self, source: usize, destination: usize) {
            let (s, d) = unwrap_get_two_mut!(
                self.storage,
                source,
                destination,
                stringify!($name)
            );
            d.$to_side.xor_inplace(&s.$from_side);
            s.$from_side.resize(0, false)
        }
    )*}
}

/// Note that the methods that add or remove memory hold the invariants of (S)torage's
/// [Base] implementation.
impl<S, B> Tracker for Frames<S>
where
    S: IterableBase<T = PauliStack<B>>,
    B: BooleanVector,
{
    type Stack = PauliStack<B>;
    type Pauli = PauliTuple;

    fn new_qubit(&mut self, qubit: usize) -> Option<Self::Stack> {
        self.storage.insert(qubit, Self::Stack::zeros(self.frames_num))
    }

    fn track_pauli(&mut self, qubit: usize, pauli: Self::Pauli) {
        if self.storage.is_empty() {
            return;
        }
        for (i, p) in self.storage.iter_pairs_mut() {
            if i == qubit {
                p.push(pauli);
            } else {
                p.push(Self::Pauli::new_i());
            }
        }
        self.frames_num += 1;
    }

    fn track_pauli_string(&mut self, string: PauliString<Self::Pauli>) {
        if self.storage.is_empty() {
            return;
        }
        for (_, p) in self.storage.iter_pairs_mut() {
            p.push(Self::Pauli::new_i());
        }
        for (i, p) in string {
            match self.storage.get_mut(i) {
                Some(pauli) => {
                    pauli.z.set(self.frames_num, p.get_z());
                    pauli.x.set(self.frames_num, p.get_x());
                },
                None => continue,
            }
        }
        self.frames_num += 1;
    }

    single!(s, h, sh, hs, shs,);
    fn cz(&mut self, bit_a: usize, bit_b: usize) {
        let (a, b) = unwrap_get_two_mut!(self.storage, bit_a, bit_b, "cz");
        a.z.xor_inplace(&b.x);
        b.z.xor_inplace(&a.x);
    }

    fn cx(&mut self, control: usize, target: usize) {
        let (c, t) = unwrap_get_two_mut!(self.storage, control, target, "cx");
        t.x.xor_inplace(&c.x);
        c.z.xor_inplace(&t.z);
    }

    fn cy(&mut self, control: usize, target: usize) {
        let (c, t) = unwrap_get_two_mut!(self.storage, control, target, "cy");
        // (c)ontrol, (t)arget, z, x, (o)ld, (n)ew
        // tzn = tzo + cxo
        // txn = txo + cxo
        // czn = tzo + czo + txo
        // cxn = cxo
        c.z.xor_inplace(&t.z);
        c.z.xor_inplace(&t.x);
        t.z.xor_inplace(&c.x);
        t.x.xor_inplace(&c.x);
        // this has the same number of (xor_inplace)(xor_inplace) operations as the
        // default implementation
    }

    fn swap(&mut self, bit_a: usize, bit_b: usize) {
        let (a, b) = unwrap_get_two_mut!(self.storage, bit_a, bit_b, "swap");
        mem::swap(a, b)
    }

    fn iswap(&mut self, bit_a: usize, bit_b: usize) {
        let (a, b) = unwrap_get_two_mut!(self.storage, bit_a, bit_b, "iswap");
        // something smarter here ...?
        mem::swap(a, b);
        a.z.xor_inplace(&b.x);
        a.z.xor_inplace(&a.x);
        b.z.xor_inplace(&b.x);
        b.z.xor_inplace(&a.x);
        // as in the Live implementation, we could save one xor_inplace by saving a.x ^
        // b.x in a temporary variable, but it's not clear whether that would be faster
    }

    movements!(
        (move_z_to_z, z, z, "Z", "Z"),
        (move_z_to_x, z, x, "Z", "X"),
        (move_x_to_z, x, z, "X", "Z"),
        (move_x_to_x, x, x, "X", "X"),
    );

    fn measure(&mut self, bit: usize) -> Result<PauliStack<B>, MissingBit> {
        self.storage.remove(bit).ok_or(MissingBit(bit))
    }
}

impl<S, B> Frames<S>
where
    S: IterableBase<T = PauliStack<B>>,
    B: BooleanVector,
{
    /// Pop the last tracked Pauli frame.
    ///
    /// If you do this to get all frames, you might want to use
    /// [transpose_reverted](Frames::transpose_reverted).
    pub fn pop_frame<P: Pauli>(&mut self) -> Option<PauliString<P>> {
        if self.storage.is_empty() || self.frames_num == 0 {
            return None;
        }
        let mut ret = Vec::new();
        for (i, p) in self.storage.iter_pairs_mut() {
            if let Some(pauli) = p.pop() {
                ret.push((i, pauli))
            }
        }
        self.frames_num -= 1;
        Some(ret)
    }

    /// Measure a qu`bit` and store the according stack of tracked Paulis into
    /// `storage`. Errors when the qu`bit` is not present in the tracker.
    pub fn measure_and_store(
        &mut self,
        bit: usize,
        storage: &mut impl Base<TB = PauliStack<B>>,
    ) -> Result<(), MoveError<B>> {
        match storage.insert(bit, self.measure(bit)?) {
            Some(p) => Err(OverwriteStack { bit, stack: p }.into()),
            None => Ok(()),
        }
    }

    /// Transpose the frames, with reverted order of the frames and sorted qubits. The
    /// result is a non-sparse matrix of Paulis.
    ///
    /// Depending on the use case,
    /// [stacked_transpose_reverted](Self::stacked_transpose_reverted) can be more
    /// efficient in use.
    ///
    /// # Panics
    /// Panics if `num_qubits` is smaller the highest qubit index that has been tracked.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, coverage(off))]
    /// # fn main() {
    /// # use pauli_tracker::{collection::NaiveVector, pauli::{self, PauliTuple},
    /// #     tracker::frames::Frames};
    /// type PauliStack = pauli::PauliStack<Vec<bool>>;
    /// assert_eq!(
    ///     Frames::<NaiveVector<_>>::new_unchecked(vec![
    ///         //               frame  Z 12  X 12              qubit
    ///         PauliStack::try_from_str("10", "01").unwrap(), // 0
    ///         PauliStack::try_from_str("11", "10").unwrap(), // 1
    ///         PauliStack::try_from_str("11", "01").unwrap(), // 2
    ///     ].into(), 2).transpose_reverted::<PauliTuple>(3),
    ///     vec![ // qubit (Z, X)   0       1       2      frame
    ///                     vec![(0, 1), (1, 0), (1, 1)], // 2
    ///                     vec![(1, 0), (1, 1), (1, 0)], // 1
    ///     ].into_iter().map(|frame| frame.into_iter().map(|(z, x)|
    ///         PauliTuple(z==1, x==1)).collect::<Vec<PauliTuple>>()).collect::<Vec<_>>()
    /// );
    /// # }
    // for efficiency, one should flatten the matrix internally, but the matrix is not
    // really used in matrix operations (usually, I think), so to have a return type with
    // a more flexibel API, we don't do that
    pub fn transpose_reverted<P: Pauli + Clone>(
        mut self,
        num_qubits: usize,
    ) -> Vec<Vec<P>> {
        let mut ret = Vec::with_capacity(self.frames_num);
        while let Some(frame) = self.pop_frame::<P>() {
            let mut paulis = vec![P::I; num_qubits];
            for (i, p) in frame {
                paulis[i] = p;
            }
            ret.push(paulis);
        }
        ret
    }

    /// Similar to [transpose_reverted](Self::transpose_reverted), but use [PauliStack]
    /// for the frames.
    ///
    /// # Panics
    /// Panics if `num_qubits` is smaller the highest qubit index that has been tracked.
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, coverage(off))]
    /// # fn main() {
    /// # use pauli_tracker::{collection::NaiveVector, pauli::{self, PauliTuple},
    /// #     tracker::frames::Frames};
    /// type PauliStack = pauli::PauliStack<Vec<bool>>;
    /// assert_eq!(
    ///     Frames::<NaiveVector<_>>::new_unchecked(vec![
    ///         PauliStack::try_from_str("10", "01").unwrap(),
    ///         PauliStack::try_from_str("11", "10").unwrap(),
    ///         PauliStack::try_from_str("11", "01").unwrap(),
    ///     ].into(), 2).stacked_transpose_reverted(3),
    ///     vec![
    ///         PauliStack::try_from_str("011", "101").unwrap(),
    ///         PauliStack::try_from_str("111", "010").unwrap(),
    ///     ]
    /// );
    /// # }
    pub fn stacked_transpose_reverted(mut self, num_qubits: usize) -> Vec<PauliStack<B>> {
        let mut ret = Vec::with_capacity(self.frames_num);
        while let Some(frame) = self.pop_frame::<PauliTuple>() {
            let mut stack = PauliStack::<B>::zeros(num_qubits);
            for (i, p) in frame {
                stack.z.set(i, p.0);
                stack.x.set(i, p.1);
            }
            ret.push(stack);
        }
        ret
    }
}

impl<S, B> Frames<S>
where
    S: Full<T = PauliStack<B>> + Default,
    B: BooleanVector,
{
    /// Measure all qubits and put the according stack of Paulis into `storage`, i.e.,
    /// do [Frames::measure_and_store] for all qubits.
    pub fn measure_and_store_all(&mut self, storage: &mut impl Base<TB = PauliStack<B>>) {
        for (bit, pauli) in
            // mem::replace(&mut self.storage, S::init(0, PauliStack::default()))
            mem::take(&mut self.storage).into_iter()
        {
            storage.insert(bit, pauli);
        }
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;

    // we only check the basic functionality here, more complicated circuits are tested
    // in [super::circuit] to test the tracker and the circuit at once

    mod action_definition_check {
        use super::{super::*, test, *};
        use crate::{
            collection::BufferedVector,
            pauli::PauliDense,
            tracker::tests::utils::{
                self, DoubleAction, DoubleResults, SingleAction, SingleResults,
                N_DOUBLES, N_SINGLES,
            },
        };

        // maybe todo: in the following functions there's a pattern behind how we encode
        // one-qubit and two-qubit actions, it's like a "TwoBitVec"; one could probably
        // implement that in connection with [Pauli]

        type ThisTracker = Frames<BufferedVector<PauliStack<bit_vec::BitVec>>>;
        // type ThisTracker =
        //     Frames<Vector<crate::boolean_vector::bitvec_simd::SimdBitVec>>;

        #[test]
        fn single() {
            type Action = SingleAction<ThisTracker>;

            const ACTIONS: [Action; N_SINGLES] = utils::single_actions!(ThisTracker);

            #[cfg_attr(coverage_nightly, coverage(off))]
            fn runner(action: Action, result: SingleResults) {
                let mut tracker: ThisTracker = Frames::init(1);
                for input in (0..4).rev() {
                    tracker.track_pauli_string(vec![(
                        0,
                        PauliDense::try_from(input).unwrap().into(),
                    )]);
                }
                (action)(&mut tracker, 0);
                for (input, check) in (0u8..).zip(result.1) {
                    let computed = tracker
                        .pop_frame::<PauliDense>()
                        .unwrap()
                        .first()
                        .unwrap()
                        .1
                        .storage();
                    assert_eq!(
                        computed, check,
                        "gate: {}, input: {}, expected: {}, computed: {}",
                        result.0, input, check, computed
                    );
                }
            }

            utils::single_check(runner, ACTIONS)
        }

        #[test]
        fn double() {
            type Action = DoubleAction<ThisTracker>;

            const ACTIONS: [Action; N_DOUBLES] = utils::double_actions!(ThisTracker);

            #[cfg_attr(coverage_nightly, coverage(off))]
            fn runner(action: Action, result: DoubleResults) {
                let mut tracker: ThisTracker = Frames::init(2);
                for pauli in (0..16).rev() {
                    tracker.track_pauli_string(utils::double_init(pauli));
                }
                (action)(&mut tracker, 1, 0);
                for (input, check) in (0u8..).zip(result.1) {
                    let computed =
                        utils::double_output(tracker.pop_frame::<PauliDense>().unwrap());
                    assert_eq!(
                        computed, check,
                        "gate: {}, input: {}, expected: {:?}, computed: {:?}",
                        result.0, input, check, computed
                    );
                }
            }

            utils::double_check(runner, ACTIONS);
        }
    }
}
