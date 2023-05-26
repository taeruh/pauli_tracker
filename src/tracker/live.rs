use std::{
    cmp::Ordering,
    ops::{
        Deref,
        DerefMut,
    },
};

use super::{
    PauliString,
    Tracker,
};
use crate::{
    pauli::Pauli,
    slice_extension::GetTwoMutSlice,
};

#[derive(Clone, Debug, Default)]
pub struct BitVector {
    // this will become a bitvector later ...
    inner: Vec<Pauli>,
}

impl Deref for BitVector {
    type Target = Vec<Pauli>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for BitVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Tracker for BitVector {
    type Stack = Pauli;

    fn init(num_bits: usize) -> Self {
        BitVector {
            inner: vec![Pauli::new_i(); num_bits],
        }
    }

    fn new_qubit(&mut self, bit: usize) -> Option<usize> {
        let len = self.len();
        match bit.cmp(&len) {
            Ordering::Less => Some(bit),
            Ordering::Equal => {
                self.push(Pauli::new_i());
                None
            }
            Ordering::Greater => {
                let diff = bit - len;
                self.try_reserve(diff).unwrap();
                self.extend(std::iter::repeat(Pauli::new_i()).take(diff));
                None
            }
        }
    }

    fn track_pauli(&mut self, bit: usize, pauli: Pauli) {
        if let Some(p) = self.get_mut(bit) {
            p.xor(pauli)
        }
    }
    fn track_pauli_string(&mut self, string: PauliString) {
        for (bit, pauli) in string {
            if let Some(p) = self.get_mut(bit) {
                p.xor(pauli)
            }
        }
    }

    fn h(&mut self, bit: usize) {
        self[bit].h();
    }
    fn s(&mut self, bit: usize) {
        self[bit].s();
    }

    fn cx(&mut self, control: usize, target: usize) {
        let (c, t) = self.get_two_mut(control, target).unwrap();
        t.xor_u8(c.left_mask());
        c.xor_u8(t.right_mask());
    }
    fn cz(&mut self, bit_a: usize, bit_b: usize) {
        let (a, b) = self.get_two_mut(bit_a, bit_b).unwrap();
        a.xor_u8(b.left_mask() >> 1);
        b.xor_u8(a.left_mask() >> 1);
    }

    fn move_z_to_x(&mut self, source: usize, destination: usize) {
        let (s, d) = self.get_two_mut(source, destination).unwrap();
        d.xor_u8(s.right_mask() << 1);
        s.set_z(false);
    }
    fn move_z_to_z(&mut self, source: usize, destination: usize) {
        let (s, d) = self.get_two_mut(source, destination).unwrap();
        d.xor_u8(s.right_mask());
        s.set_z(false);
    }

    fn measure(&mut self, bit: usize) -> Option<Self::Stack> {
        Some(self[bit])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_qubit_gates() {
        // double-pauli p = abcd in binary;
        // encoding: x_0 = a, z_0 = b, x_1 = c, z_2 = d
        type Action = fn(&mut BitVector, usize, usize);
        const GATES: [(
            // action
            Action,
            // name for debugging
            &str,
            // result: calculated by hand
            // encoded input: p = 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
            [u8; 16],
        ); 2] = [
            (
                BitVector::cx, // left->control, right->target
                "CX",
                [0, 5, 2, 7, 4, 1, 6, 3, 10, 15, 8, 13, 14, 11, 12, 9],
            ),
            (
                BitVector::cz,
                "CZ",
                [0, 1, 6, 7, 4, 5, 2, 3, 9, 8, 15, 14, 13, 12, 11, 10],
            ),
        ];

        // masks to decode p in 0..16 into two paulis and vice versa
        const FIRST: u8 = 12;
        const FIRST_SHIFT: u8 = 2;
        const SECOND: u8 = 3;

        for action in GATES {
            for (input, check) in (0u8..).zip(action.2) {
                let mut tracker = BitVector::init(2);
                tracker.track_pauli_string(vec![
                    (0, Pauli::try_from((input & FIRST) >> FIRST_SHIFT).unwrap()),
                    (1, Pauli::try_from(input & SECOND).unwrap()),
                ]);
                (action.0)(&mut tracker, 0, 1);
                let frame = &tracker.inner;
                let mut result = 0;
                for (i, p) in frame.iter().enumerate() {
                    if i == 0 {
                        result += p.storage() << FIRST_SHIFT
                    } else if i == 1 {
                        result += p.storage()
                    }
                }
                assert_eq!(result, check, "{}, {}", action.1, input);
            }
        }
    }

    #[test]
    fn movement() {
        type Action = fn(&mut BitVector, usize, usize);
        const MOVEMENT: [(Action, &str, [u8; 16]); 2] = [
            (
                BitVector::move_z_to_x,
                "xz",
                [0, 1, 2, 3, 2, 3, 0, 1, 8, 9, 10, 11, 10, 11, 8, 9],
            ),
            (
                BitVector::move_z_to_z,
                "zz",
                [0, 1, 2, 3, 1, 0, 3, 2, 8, 9, 10, 11, 9, 8, 11, 10],
            ),
        ];

        // masks to decode p in 0..16 into two paulis and vice versa
        const FIRST: u8 = 12;
        const FIRST_SHIFT: u8 = 2;
        const SECOND: u8 = 3;

        for action in MOVEMENT {
            for (input, check) in (0u8..).zip(action.2) {
                let mut tracker = BitVector::init(2);
                tracker.track_pauli_string(vec![
                    (0, Pauli::try_from((input & FIRST) >> FIRST_SHIFT).unwrap()),
                    (1, Pauli::try_from(input & SECOND).unwrap()),
                ]);
                (action.0)(&mut tracker, 0, 1);
                let frame = &tracker.inner;
                let mut result = 0;
                for (i, p) in frame.iter().enumerate() {
                    if i == 0 {
                        result += p.storage() << FIRST_SHIFT
                    } else if i == 1 {
                        result += p.storage()
                    }
                }
                assert_eq!(result, check, "{}, {}", action.1, input);
            }
        }
    }
}
