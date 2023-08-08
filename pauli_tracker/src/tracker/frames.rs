/*!
Track Pauli gates when constructing a circuit.

This module provides the [Frames] Pauli tracker. Each new tracked Pauli introduces a new
frame on the qubits, for example corresponding to a measurement, with the tracked Pauli
on the qubit where it has been initialized and identities on all other qubits. The
Clifford gates act on this frame, according to the conjugation rules, causing the
tracked Pauli to being copied, moved, swaped, ... within the frame. The frames of
multiple tracked Paulis are stacked up, not effectiving each other; this is the main
difference to the [Live] tracker, which adds up the frames.

When using this tracer, you probably only want to track Paulis, via the `track_x/y/z`
methods, induced by non-deterministic measurements, and account for other Paulis simply
through the `x/y/z` methods (since these methods do literally nothing, you can actually
ignore them). This is because this tracker is used to analyze the dependency induced by
the measurements. To track Paulis during the actual execution of a circuit, the [Live]
tracker is more useful.

[Live]: super::live::Live
*/

use std::mem;

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};
use thiserror::Error;

use super::{
    MissingBit,
    PauliString,
    Tracker,
};
use crate::{
    boolean_vector::BooleanVector,
    collection::{
        Base,
        Full,
        Init,
        IterableBase,
    },
    pauli::{
        Pauli,
        PauliStack,
        PauliTuple,
    },
};

// pub mod storage;
pub mod dependency_graph;

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
        self.as_storage()
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
    /// Create a new [Frames] instance with a given storage and number of frames. It
    /// does not check whether the storage is compatible with the number of frames. If
    /// it is not, using the create instance might result in errors or panics. This
    /// function is mainly useful to put some parts of a frames storage into a new one.
    pub fn new_unchecked(storage: S, frames_num: usize) -> Self {
        Self { storage, frames_num }
    }

    /// Get the underlining storage.
    pub fn as_storage(&self) -> &S {
        &self.storage
    }

    /// Convert the object into the underlining storage.
    pub fn into_storage(self) -> S {
        self.storage
    }

    /// Get the number of tracked frames.
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

macro_rules! single {
    ($($name:ident),*) => {$(
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
    )),*) => {$(
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
                    pauli.left.set(self.frames_num, p.get_x());
                    pauli.right.set(self.frames_num, p.get_z());
                }
                None => continue,
            }
        }
        self.frames_num += 1;
    }

    single!(h, s);
    fn cz(&mut self, bit_a: usize, bit_b: usize) {
        let (a, b) = unwrap_get_two_mut!(self.storage, bit_a, bit_b, "cz");
        a.right.xor_inplace(&b.left);
        b.right.xor_inplace(&a.left);
    }

    fn cx(&mut self, control: usize, target: usize) {
        let (c, t) = unwrap_get_two_mut!(self.storage, control, target, "cx");
        t.left.xor_inplace(&c.left);
        c.right.xor_inplace(&t.right);
    }

    movements!(
        (move_x_to_x, left, left, "X", "X"),
        (move_x_to_z, left, right, "X", "Z"),
        (move_z_to_x, right, left, "Z", "X"),
        (move_z_to_z, right, right, "Z", "Z")
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
}

impl<S, B> Frames<S>
where
    S: Full<T = PauliStack<B>> + Default,
    B: BooleanVector,
{
    /// Measure all qubits and put the according stack of Paulis into `storage`, i.e.,
    /// do [Frames::measure_and_store] for all qubits.
    pub fn measure_and_store_all(
        &mut self,
        storage: &mut impl Base<TB = PauliStack<B>>,
    ) {
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

    #[cfg(feature = "bitvec")]
    // #[cfg(feature = "bitvec_simd")]
    mod action_definition_check {
        use super::{
            super::*,
            test,
            *,
        };
        use crate::{
            collection::BufferedVector,
            pauli::PauliDense,
            tracker::test::utils::{
                self,
                DoubleAction,
                DoubleResults,
                SingleAction,
                SingleResults,
                N_DOUBLES,
                N_SINGLES,
            },
        };

        // maybe todo: in the following functions there's a pattern behind how we encode
        // one-qubit and two-qubit actions, it's like a "TwoBitVec"; one could probably
        // implement that in connection with [Pauli]

        type ThisTracker = Frames<BufferedVector<PauliStack<bitvec::vec::BitVec>>>;
        // type ThisTracker =
        //     Frames<Vector<crate::boolean_vector::bitvec_simd::SimdBitVec>>;

        #[test]
        fn single() {
            type Action = SingleAction<ThisTracker>;

            const ACTIONS: [Action; N_SINGLES] = utils::single_actions!(ThisTracker);

            #[cfg_attr(coverage_nightly, no_coverage)]
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
                    assert_eq!(
                        tracker
                            .pop_frame::<PauliDense>()
                            .unwrap()
                            .first()
                            .unwrap()
                            .1
                            .storage(),
                        check,
                        "{}, {}",
                        result.0,
                        input
                    );
                }
            }

            utils::single_check(runner, ACTIONS)
        }

        #[test]
        fn double() {
            type Action = DoubleAction<ThisTracker>;

            const ACTIONS: [Action; N_DOUBLES] = utils::double_actions!(ThisTracker);

            #[cfg_attr(coverage_nightly, no_coverage)]
            fn runner(action: Action, result: DoubleResults) {
                let mut tracker: ThisTracker = Frames::init(2);
                for pauli in (0..16).rev() {
                    tracker.track_pauli_string(utils::double_init(pauli));
                }
                (action)(&mut tracker, 0, 1);
                for (input, check) in (0u8..).zip(result.1) {
                    let output = utils::double_output(
                        tracker.pop_frame::<PauliDense>().unwrap(),
                    );
                    assert_eq!(output, check, "{}, {}", result.0, input);
                }
            }

            utils::double_check(runner, ACTIONS);
        }
    }
}
