/*!
This module provides [Tracker]s that are similar to the ones in [frames](super::frames),
with the major difference that there's effectively only one frames, which adds up
multiple tracked Paulis.
*/

use std::{
    self,
    cmp::Ordering,
    iter,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    unwrap_get_mut,
    unwrap_get_two_mut,
    PauliString,
    Tracker,
};
use crate::{
    pauli::Pauli,
    slice_extension::GetTwoMutSlice,
};

// todo: make it generic and also do it with a hashmap

/// An implementor of [Tracker], similar to [Frames](super::frames::Frames), with the
/// difference, that instead of storing each Pauli frame, it adds the Pauli frames (mod
/// 2).
// I'm not sure what the most efficient inner type would be here, Vec<bool>, Vec<Pauli>,
// BitVec, ...
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiveVector {
    inner: Vec<Pauli>,
}

impl From<Vec<Pauli>> for LiveVector {
    fn from(value: Vec<Pauli>) -> Self {
        Self { inner: value }
    }
}

impl From<LiveVector> for Vec<Pauli> {
    fn from(value: LiveVector) -> Self {
        value.inner
    }
}

impl AsRef<Vec<Pauli>> for LiveVector {
    fn as_ref(&self) -> &Vec<Pauli> {
        &self.inner
    }
}

impl LiveVector {
    /// Returns a reference to an element at index. Returns [None] if out of bounds.
    pub fn get(&self, bit: usize) -> Option<&Pauli> {
        self.inner.get(bit)
    }
    /// Returns a mutable reference to an element at index. Returns [None] if out of
    /// bounds.
    pub fn get_mut(&mut self, bit: usize) -> Option<&mut Pauli> {
        self.inner.get_mut(bit)
    }
}

macro_rules! single {
    ($($name:ident),*) => {$(
        fn $name(&mut self, bit: usize) {
            unwrap_get_mut!(self.inner, bit, stringify!($name)).$name()
        }
    )*};
}

/// Note that the inner storage type is basically a vector. Therefore, the it may
/// contain buffer qubits, even though they were not explicitly initialized.
impl Tracker for LiveVector {
    type Stack = Pauli;

    fn init(num_bits: usize) -> Self {
        LiveVector {
            inner: vec![Pauli::new_i(); num_bits],
        }
    }

    fn new_qubit(&mut self, bit: usize) -> Option<usize> {
        let len = self.inner.len();
        match bit.cmp(&len) {
            Ordering::Less => Some(bit),
            Ordering::Equal => {
                self.inner.push(Pauli::new_i());
                None
            }
            Ordering::Greater => {
                let diff = bit - len + 1;
                self.inner.try_reserve(diff).unwrap_or_else(|e| {
                    panic!("error when trying to reserve enough memory: {e}")
                });
                self.inner.extend(iter::repeat(Pauli::new_i()).take(diff));
                None
            }
        }
    }

    fn track_pauli(&mut self, bit: usize, pauli: Pauli) {
        if let Some(p) = self.inner.get_mut(bit) {
            p.xor(pauli)
        }
    }
    fn track_pauli_string(&mut self, string: PauliString) {
        for (bit, pauli) in string {
            if let Some(p) = self.inner.get_mut(bit) {
                p.xor(pauli)
            }
        }
    }

    single!(h, s);

    fn cx(&mut self, control: usize, target: usize) {
        let (c, t) = unwrap_get_two_mut!(self.inner, control, target, "cx");
        t.xor_u8(c.xmask());
        c.xor_u8(t.zmask());
    }
    fn cz(&mut self, bit_a: usize, bit_b: usize) {
        let (a, b) = unwrap_get_two_mut!(self.inner, bit_a, bit_b, "cz");
        a.xor_u8(b.xmask() >> 1);
        b.xor_u8(a.xmask() >> 1);
    }

    fn move_x_to_x(&mut self, source: usize, destination: usize) {
        let (s, d) =
            unwrap_get_two_mut!(self.inner, source, destination, "move_x_to_x");
        d.xor_u8(s.xmask());
        s.set_x(false);
    }
    fn move_x_to_z(&mut self, source: usize, destination: usize) {
        let (s, d) =
            unwrap_get_two_mut!(self.inner, source, destination, "move_x_to_z");
        d.xor_u8(s.xmask() >> 1);
        s.set_x(false);
    }
    fn move_z_to_x(&mut self, source: usize, destination: usize) {
        let (s, d) =
            unwrap_get_two_mut!(self.inner, source, destination, "move_z_to_x");
        d.xor_u8(s.zmask() << 1);
        s.set_z(false);
    }
    fn move_z_to_z(&mut self, source: usize, destination: usize) {
        let (s, d) =
            unwrap_get_two_mut!(self.inner, source, destination, "move_z_to_z");
        d.xor_u8(s.zmask());
        s.set_z(false);
    }

    fn measure(&mut self, bit: usize) -> Option<Self::Stack> {
        Some(*self.get(bit)?)
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;

    mod action_definition_check {
        use super::{
            test,
            *,
        };
        use crate::tracker::test::impl_utils::{
            self,
            DoubleAction,
            DoubleResults,
            SingleAction,
            SingleResults,
            N_DOUBLES,
            N_SINGLES,
        };

        #[test]
        fn single() {
            type Action = SingleAction<LiveVector>;

            const ACTIONS: [Action; N_SINGLES] = [LiveVector::h, LiveVector::s];

            #[cfg_attr(coverage_nightly, no_coverage)]
            fn runner(action: Action, result: SingleResults) {
                for (input, check) in (0u8..).zip(result.1) {
                    let mut tracker = LiveVector::init(2);
                    tracker.track_pauli_string(impl_utils::single_init(input));
                    (action)(&mut tracker, 0);
                    assert_eq!(
                        *tracker.inner.get(0).unwrap().storage(),
                        check,
                        "{}, {}",
                        result.0,
                        input
                    );
                }
            }

            impl_utils::single_check(runner, ACTIONS);
        }

        #[test]
        fn double() {
            type Action = DoubleAction<LiveVector>;

            const ACTIONS: [Action; N_DOUBLES] = [
                LiveVector::cx,
                LiveVector::cz,
                LiveVector::move_x_to_x,
                LiveVector::move_x_to_z,
                LiveVector::move_z_to_x,
                LiveVector::move_z_to_z,
            ];

            #[cfg_attr(coverage_nightly, no_coverage)]
            fn runner(action: Action, result: DoubleResults) {
                for (input, check) in (0u8..).zip(result.1) {
                    let mut tracker = LiveVector::init(2);
                    tracker.track_pauli_string(impl_utils::double_init(input));
                    (action)(&mut tracker, 0, 1);
                    let output = impl_utils::double_output(
                        tracker.inner.into_iter().enumerate(),
                    );
                    assert_eq!(output, check, "{}, {}", result.0, input);
                }
            }

            impl_utils::double_check(runner, ACTIONS);
        }
    }

    #[test]
    fn new_qubit_and_measure() {
        let mut tracker = LiveVector::init(1);
        tracker.track_x(0);
        assert_eq!(tracker.new_qubit(0), Some(0));
        assert_eq!(tracker.new_qubit(1), None);
        assert_eq!(*tracker.as_ref(), vec![Pauli::new_x(), Pauli::new_i()]);
        assert_eq!(tracker.measure(0), Some(Pauli::new_x()));
        assert_eq!(tracker.new_qubit(3), None);
        assert_eq!(
            *tracker.as_ref(),
            vec![Pauli::new_x(), Pauli::new_i(), Pauli::new_i(), Pauli::new_i()]
        );
    }
}
