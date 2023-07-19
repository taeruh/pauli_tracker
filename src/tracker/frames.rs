/*!
The [Frames] type can be used as [Tracker] to analyze how the tracked Paulis effect the
qubits.

Each new tracked Pauli introduces a new frame on the qubits, for example corresponding
to a measurement, with the tracked Pauli on the qubit where it has been initialized and
on all other qubits identities. The Clifford gates act on this frame, according to the
conjugation rules, causing the tracked Pauli to being copied, moved, swaped, ... within
the frame. The frames of multiple tracked Paulis are stacked up together, not
effectiving each other; this is the main difference to the tracker defined in
[live](super::live).
*/

use std::{
    error::Error,
    fmt::{
        self,
        Debug,
        Display,
        Formatter,
    },
    mem,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    MissingStack,
    PauliString,
    Tracker,
};
use crate::{
    boolean_vector::BooleanVector,
    collection::Collection,
    pauli::{
        Pauli,
        PauliStack,
        PauliTuple,
    },
};

// pub mod storage;
pub mod dependency_graph;

/// A container of multiple Pauli frames, using a generic `Storage` type  as internal
/// storage, that implements [Tracker].
///
/// The type implements the core functionality to track the Pauli frames through a
/// Clifford circuit. To be useful, the generic `Storage` type should implement
/// [Collection]. The explicit storage type should have the [PauliStack]s on it's minor
/// axis (this is more or less enforced by [Collection]). The module
/// [collection](crate::collection) provides some compatible storage types.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Frames<Storage> {
    storage: Storage,
    frames_num: usize,
}

/// The Error when we overwrite a qubit's Pauli stack.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OverwriteStack<T> {
    /// The qubit.
    pub bit: usize,
    /// The stack we have overwritten.
    pub stack: PauliStack<T>,
}
impl<T> Display for OverwriteStack<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "the Pauli stack for qubit {} has been overwritten", self.bit)
    }
}
impl<T: Debug> Error for OverwriteStack<T> {}

impl<Storage> AsRef<Storage> for Frames<Storage> {
    fn as_ref(&self) -> &Storage {
        self.as_storage()
    }
}

/// The Error when one tries to measure a qubit and store it stacks in another storage,
/// as in [measure_and_store](Frames::measure_and_store).
///
/// It can be either a [MissingStack] error, if the qubit is missing, or an
/// [OverwriteStack] error if the qubit was already present in the other storage.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StoreError<T> {
    /// If one would overwrite the stack in the other storage.
    OverwriteStack(OverwriteStack<T>),
    /// If the qubit and its stack are missing.
    MissingStack(MissingStack),
}
impl<T> Display for StoreError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::OverwriteStack(e) => write!(f, "{e}"),
            StoreError::MissingStack(e) => write!(f, "{e}"),
        }
    }
}
impl<T: Debug> Error for StoreError<T> {}

impl<T> From<OverwriteStack<T>> for StoreError<T> {
    fn from(value: OverwriteStack<T>) -> Self {
        StoreError::OverwriteStack(value)
    }
}
impl<T> From<MissingStack> for StoreError<T> {
    fn from(value: MissingStack) -> Self {
        StoreError::MissingStack(value)
    }
}

impl<S> Frames<S> {
    /// Create a new [Frames] instance.
    pub fn new(storage: S, frames_num: usize) -> Self {
        Self { storage, frames_num }
    }

    /// Get the underlining storage.
    pub fn as_storage(&self) -> &S {
        &self.storage
    }

    /// Get the number of tracked frames.
    pub fn frames_num(&self) -> usize {
        self.frames_num
    }

    /// Convert the object into the underlining storage.
    pub fn into_storage(self) -> S {
        self.storage
    }

    // Pauli gates don't do anything; we just include them for completeness and because
    // it might be more convenient to have them on the caller side
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

impl<S, B> Frames<S>
where
    S: Collection<T = PauliStack<B>>,
    B: BooleanVector,
{
    /// Pop the last tracked Pauli frame.
    pub fn pop_frame<P: Pauli>(&mut self) -> Option<PauliString<P>> {
        if self.storage.is_empty() || self.frames_num == 0 {
            return None;
        }
        let mut ret = Vec::new();
        for (i, p) in self.storage.iter_mut() {
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
        storage: &mut impl Collection<T = PauliStack<B>>,
    ) -> Result<(), StoreError<B>> {
        match storage.insert(bit, self.measure(bit)?) {
            Some(p) => Err(OverwriteStack { bit, stack: p }.into()),
            None => Ok(()),
        }
    }

    /// Measure all qubits and put the according stack of Paulis into `storage`, i.e.,
    /// do [Frames::measure_and_store] for all qubits.
    pub fn measure_and_store_all(
        &mut self,
        storage: &mut impl Collection<T = PauliStack<B>>,
    ) {
        for (bit, pauli) in mem::replace(&mut self.storage, S::init(0)).into_iter() {
            storage.insert(bit, pauli);
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
/// [Collection] implementation.
impl<S, B> Tracker for Frames<S>
where
    S: Collection<T = PauliStack<B>>,
    B: BooleanVector,
{
    type Stack = PauliStack<B>;
    type Pauli = PauliTuple;

    fn init(num_qubits: usize) -> Self {
        Self {
            storage: S::init(num_qubits),
            frames_num: 0,
        }
    }

    fn new_qubit(&mut self, qubit: usize) -> Option<Self::Stack> {
        self.storage.insert(qubit, Self::Stack::zeros(self.frames_num))
    }

    fn track_pauli(&mut self, qubit: usize, pauli: Self::Pauli) {
        if self.storage.is_empty() {
            return;
        }
        for (i, p) in self.storage.iter_mut() {
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
        for (_, p) in self.storage.iter_mut() {
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

    fn cx(&mut self, control: usize, target: usize) {
        let (c, t) = unwrap_get_two_mut!(self.storage, control, target, "cx");
        t.left.xor_inplace(&c.left);
        c.right.xor_inplace(&t.right);
    }

    fn cz(&mut self, bit_a: usize, bit_b: usize) {
        let (a, b) = unwrap_get_two_mut!(self.storage, bit_a, bit_b, "cz");
        a.right.xor_inplace(&b.left);
        b.right.xor_inplace(&a.left);
    }

    movements!(
        (move_x_to_x, left, left, "X", "X"),
        (move_x_to_z, left, right, "X", "Z"),
        (move_z_to_x, right, left, "Z", "X"),
        (move_z_to_z, right, right, "Z", "Z")
    );

    fn measure(&mut self, bit: usize) -> Result<PauliStack<B>, MissingStack> {
        self.storage.remove(bit).ok_or(MissingStack { bit })
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
            tracker::test::impl_utils::{
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

            const ACTIONS: [Action; N_SINGLES] = [Frames::h, Frames::s];

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

            impl_utils::single_check(runner, ACTIONS)
        }

        #[test]
        fn double() {
            type Action = DoubleAction<ThisTracker>;

            const ACTIONS: [Action; N_DOUBLES] = [
                Frames::cx,
                Frames::cz,
                Frames::move_x_to_x,
                Frames::move_x_to_z,
                Frames::move_z_to_x,
                Frames::move_z_to_z,
            ];

            #[cfg_attr(coverage_nightly, no_coverage)]
            fn runner(action: Action, result: DoubleResults) {
                let mut tracker: ThisTracker = Frames::init(2);
                for pauli in (0..16).rev() {
                    tracker.track_pauli_string(impl_utils::double_init(pauli));
                }
                (action)(&mut tracker, 0, 1);
                for (input, check) in (0u8..).zip(result.1) {
                    let output = impl_utils::double_output(
                        tracker.pop_frame::<PauliDense>().unwrap(),
                    );
                    assert_eq!(output, check, "{}, {}", result.0, input);
                }
            }

            impl_utils::double_check(runner, ACTIONS);
        }
    }
}
