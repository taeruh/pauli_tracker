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

use std::mem;

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use self::storage::StackStorage;
use super::{
    PauliString,
    Tracker,
};
use crate::{
    boolean_vector::BooleanVector,
    pauli::{
        Pauli,
        PauliVec,
    },
};

pub mod storage;

/// A container of multiple Pauli frames, using a generic `Storage` type  as internal
/// storage, that implemenst [Tracker].
///
/// The type implements the core functionality to track the Pauli frames through a
/// Clifford circuit. To be useful, the generic `Storage` type should implement
/// [StackStorage]. The explicit storage type should have the [PauliVec]s on it's minor
/// axis (this is more or less enforced by [StackStorage]). The module [storage]
/// provides some compatible storage types.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Frames<Storage /* : StackStorage */> {
    storage: Storage,
    frames_num: usize,
}

impl<Storage> Frames<Storage> {
    /// Get the underlining storage.
    pub fn as_storage(&self) -> &Storage {
        &self.storage
    }

    /// Get the number of tracked frames.
    pub fn frames_num(&self) -> usize {
        self.frames_num
    }

    /// Convert the object into the underlining storage.
    pub fn into_storage(self) -> Storage {
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

impl<Storage> Frames<Storage>
where
    Storage: StackStorage,
{
    /// Pop the last tracked Pauli frame.
    pub fn pop_frame(&mut self) -> Option<PauliString> {
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
        storage: &mut impl StackStorage<BoolVec = Storage::BoolVec>,
    ) -> Result<(), String> {
        storage.insert_pauli(
            bit,
            match self.measure(bit) {
                Some(p) => p,
                None => return Err(format!("{bit} is not present")),
            },
        );
        Ok(())
    }

    /// Measure all qubits and put the according stack of Paulis into `storage`, i.e.,
    /// do [Frames::measure_and_store] for all qubits.
    pub fn measure_and_store_all(
        &mut self,
        storage: &mut impl StackStorage<BoolVec = Storage::BoolVec>,
    ) {
        for (bit, pauli) in
            mem::replace(&mut self.storage, Storage::init(0)).into_iter()
        {
            storage.insert_pauli(bit, pauli);
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

impl<Storage> Tracker for Frames<Storage>
where
    Storage: StackStorage,
{
    type Stack = PauliVec<Storage::BoolVec>;

    fn init(num_qubits: usize) -> Self {
        Self {
            storage: Storage::init(num_qubits),
            frames_num: 0,
        }
    }

    fn new_qubit(&mut self, qubit: usize) -> Option<usize> {
        self.storage
            .insert_pauli(qubit, Self::Stack::zeros(self.frames_num))
            .map(|_| qubit)
    }

    fn track_pauli(&mut self, qubit: usize, pauli: Pauli) {
        if self.storage.is_empty() {
            return;
        }
        for (i, p) in self.storage.iter_mut() {
            if i == qubit {
                p.push(pauli);
            } else {
                p.push(unsafe { Pauli::from_unchecked(0) });
            }
        }
        self.frames_num += 1;
    }

    fn track_pauli_string(&mut self, string: PauliString) {
        if self.storage.is_empty() {
            return;
        }
        for (_, p) in self.storage.iter_mut() {
            p.push(unsafe { Pauli::from_unchecked(0) });
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

    fn measure(&mut self, bit: usize) -> Option<PauliVec<Storage::BoolVec>> {
        self.storage.remove_pauli(bit)
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
            super::{
                storage::*,
                *,
            },
            test,
            *,
        };
        use crate::tracker::test::{
            self,
            *,
        };

        // maybe todo: in the following functions there's a pattern behind how we encode
        // one-qubit and two-qubit actions, it's like a "TwoBitVec"; one could probably
        // implement that in connection with [Pauli]

        type ThisTracker = Frames<Vector<bitvec::vec::BitVec>>;
        // type ThisTracker =
        //     Frames<Vector<crate::boolean_vector::bitvec_simd::SimdBitVec>>;

        #[test]
        fn single() {
            type Action = SingleAction<ThisTracker>;

            const ACTIONS: [Action; N_SINGLES] = [Frames::h, Frames::s];

            #[cfg_attr(coverage_nightly, no_coverage)]
            fn runner(action: Action, result: SingleResult) {
                let mut tracker: ThisTracker = Frames::init(1);
                for input in (0..4).rev() {
                    tracker
                        .track_pauli_string(vec![(0, Pauli::try_from(input).unwrap())]);
                }
                (action)(&mut tracker, 0);
                for (input, check) in (0u8..).zip(result.1) {
                    assert_eq!(
                        *tracker.pop_frame().unwrap().get(0).unwrap().1.storage(),
                        check,
                        "{}, {}",
                        result.0,
                        input
                    );
                }
            }

            test::single_check(runner, ACTIONS)
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
            fn runner(action: Action, result: DoubleResult) {
                let mut tracker: ThisTracker = Frames::init(2);
                for pauli in (0..16).rev() {
                    tracker.track_pauli_string(utils::double_init(pauli));
                }
                (action)(&mut tracker, 0, 1);
                for (input, check) in (0u8..).zip(result.1) {
                    let output = utils::double_output(tracker.pop_frame().unwrap());
                    assert_eq!(output, check, "{}, {}", result.0, input);
                }
            }

            test::double_check(runner, ACTIONS);
        }
    }
}
