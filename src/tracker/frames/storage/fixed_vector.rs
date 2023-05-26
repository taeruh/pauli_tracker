use std::{
    cmp::Ordering,
    iter::Enumerate,
    ops::{
        Deref,
        DerefMut,
    },
    slice,
};

use super::super::{
    PauliVec,
    StackStorage,
};
use crate::slice_extension::GetTwoMutSlice;

#[derive(Debug, Default)]
pub struct FixedVector {
    frames: Vec<PauliVec>,
}

impl Deref for FixedVector {
    type Target = Vec<PauliVec>;
    fn deref(&self) -> &Self::Target {
        &self.frames
    }
}

impl DerefMut for FixedVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frames
    }
}

impl IntoIterator for FixedVector {
    type Item = (usize, PauliVec);

    type IntoIter = Enumerate<<Vec<PauliVec> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.frames.into_iter().enumerate()
    }
}

impl StackStorage for FixedVector {
    type IterMut<'a> = Enumerate<slice::IterMut<'a, PauliVec>>
    where
        Self: 'a;
    type Iter<'a> = Enumerate<slice::Iter<'a, PauliVec>>
    where
        Self: 'a;

    fn insert_pauli(&mut self, qubit: usize, pauli: PauliVec) -> Option<PauliVec> {
        match qubit.cmp(&self.len()) {
            Ordering::Less => Some(pauli),
            Ordering::Equal => {
                self.push(pauli);
                None
            }
            Ordering::Greater => panic!("..."),
        }
    }

    fn remove_pauli(&mut self, qubit: usize) -> Option<PauliVec> {
        match qubit.cmp(&self.len()) {
            Ordering::Less => panic!("..."),
            Ordering::Equal => Some(self.pop().unwrap()),
            Ordering::Greater => None,
        }
    }

    #[inline(always)]
    fn get(&self, qubit: usize) -> Option<&PauliVec> {
        self.frames.get(qubit)
    }

    #[inline(always)]
    fn get_mut(&mut self, qubit: usize) -> Option<&mut PauliVec> {
        self.frames.get_mut(qubit)
    }

    fn get_two_mut(
        &mut self,
        qubit_a: usize,
        qubit_b: usize,
    ) -> Option<(&mut PauliVec, &mut PauliVec)> {
        self.frames.get_two_mut(qubit_a, qubit_b)
    }

    #[inline(always)]
    fn iter(&self) -> Self::Iter<'_> {
        self.frames.iter().enumerate()
    }

    #[inline(always)]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.frames.iter_mut().enumerate()
    }

    #[inline(always)]
    fn init(num_qubits: usize) -> Self {
        Self {
            frames: vec![PauliVec::new(); num_qubits],
        }
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}
