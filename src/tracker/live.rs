//! This module provides [Tracker]s that are similar to the ones in
//! [frames](super::frames), with the major difference that there's effectively only one
//! frames, which adds up multiple tracked Paulis.

use std::{
    self,
    cmp::Ordering,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    PauliString,
    Tracker,
};
use crate::{
    pauli::Pauli,
    slice_extension::GetTwoMutSlice,
};

// todo: also do it with a hashmap

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiveVector {
    inner: Vec<Pauli>,
}

impl LiveVector {
    pub fn get(&self, bit: usize) -> Option<&Pauli> {
        self.inner.get(bit)
    }
    pub fn get_mut(&mut self, bit: usize) -> Option<&mut Pauli> {
        self.inner.get_mut(bit)
    }

    #[inline]
    fn unwrap_get_two_mut(
        &mut self,
        bit_a: usize,
        bit_b: usize,
    ) -> (&mut Pauli, &mut Pauli) {
        self.inner
            .get_two_mut(bit_a, bit_b)
            .unwrap_or_else(|| panic!("qubit {bit_a} and/or {bit_b} do not exist"))
    }
}

macro_rules! single {
    ($($name:ident),*) => {$(
        fn $name(&mut self, bit: usize) {
            self.inner
                .get_mut(bit)
                .unwrap_or_else(|| panic!("qubit {bit} does not exist"))
                .$name();
        }
    )*};
}

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
                let diff = bit - len - 1;
                self.inner.try_reserve(diff).unwrap_or_else(|e| {
                    panic!("error when trying to reserve enough memory: {e}")
                });
                self.inner.extend(std::iter::repeat(Pauli::new_i()).take(diff));
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
        let (c, t) = self.unwrap_get_two_mut(control, target);
        t.xor_u8(c.xmask());
        c.xor_u8(t.zmask());
    }
    fn cz(&mut self, bit_a: usize, bit_b: usize) {
        let (a, b) = self.unwrap_get_two_mut(bit_a, bit_b);
        a.xor_u8(b.xmask() >> 1);
        b.xor_u8(a.xmask() >> 1);
    }

    fn move_z_to_x(&mut self, source: usize, destination: usize) {
        let (s, d) = self.unwrap_get_two_mut(source, destination);
        d.xor_u8(s.zmask() << 1);
        s.set_z(false);
    }
    fn move_z_to_z(&mut self, source: usize, destination: usize) {
        let (s, d) = self.unwrap_get_two_mut(source, destination);
        d.xor_u8(s.zmask());
        s.set_z(false);
    }
    fn move_x_to_x(&mut self, source: usize, destination: usize) {
        let (s, d) = self.unwrap_get_two_mut(source, destination);
        d.xor_u8(s.xmask());
        s.set_x(false);
    }
    fn move_x_to_z(&mut self, source: usize, destination: usize) {
        let (s, d) = self.unwrap_get_two_mut(source, destination);
        d.xor_u8(s.xmask() >> 1);
        s.set_x(false);
    }

    fn measure(&mut self, bit: usize) -> Option<Self::Stack> {
        Some(*self.get(bit)?)
    }
}

#[cfg(test)]
mod tests {
    mod action_definition_check {
        use super::super::*;
        use crate::tracker::test::{
            self,
            *,
        };

        #[test]
        fn single() {
            type Action = SingleAction<LiveVector>;

            const ACTIONS: [Action; N_SINGLES] = [LiveVector::h, LiveVector::s];

            fn runner(action: Action, result: SingleResult) {
                for (input, check) in (0u8..).zip(result.1) {
                    let mut tracker = LiveVector::init(2);
                    tracker
                        .track_pauli_string(utils::single_init(input));
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

            test::single_check(runner, ACTIONS);
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

            fn runner(action: Action, result: DoubleResult) {
                for (input, check) in (0u8..).zip(result.1) {
                    let mut tracker = LiveVector::init(2);
                    tracker.track_pauli_string(utils::double_init(input));
                    (action)(&mut tracker, 0, 1);
                    let output =
                        utils::double_output(tracker.inner.into_iter().enumerate());
                    assert_eq!(output, check, "{}, {}", result.0, input);
                }
            }

            test::double_check(runner, ACTIONS);
        }
    }
}
