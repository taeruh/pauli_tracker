use std::{
    cmp::Ordering,
    fmt::Debug,
    iter::{
        self,
        Enumerate,
    },
    mem,
    slice,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    super::StackStorage,
    PauliVec,
};
use crate::{
    boolean_vector::BooleanVector,
    slice_extension::GetTwoMutSlice,
};

/// A newtype vector of [PauliVec]s. Restricted, since we don't have the flexibility of
/// a hashmap, but if that is no problem, and the type is used correctly, it is more
/// efficient than [Map](super::map::Map).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector<B> {
    /// The inner storage.
    pub frames: Vec<PauliVec<B>>,
}

impl<B> IntoIterator for Vector<B> {
    type Item = (usize, PauliVec<B>);

    type IntoIter = Enumerate<<Vec<PauliVec<B>> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.frames.into_iter().enumerate()
    }
}

impl<B: BooleanVector> FromIterator<(usize, PauliVec<B>)> for Vector<B> {
    fn from_iter<T: IntoIterator<Item = (usize, PauliVec<B>)>>(iter: T) -> Self {
        let mut res = Vector::init(0);
        for (bit, pauli) in iter {
            res.insert_pauli_stack(bit, pauli);
        }
        res
    }
}

/// Note that [Vector] is essentially a [Vec]. Therefore, we can basically only remove
/// Pauli stacks at the end without screwing things up. When inserting Pauli stacks at
/// qubits above the length, buffer stacks are added.
impl<B: BooleanVector> StackStorage for Vector<B> {
    type BoolVec = B;
    type IterMut<'a> = Enumerate<slice::IterMut<'a, PauliVec<B>>>
    where
        Self: 'a;
    type Iter<'a> = Enumerate<slice::Iter<'a, PauliVec<B>>>
    where
        Self: 'a;

    fn insert_pauli_stack(
        &mut self,
        bit: usize,
        pauli: PauliVec<B>,
    ) -> Option<PauliVec<B>> {
        let len = self.frames.len();
        match bit.cmp(&len) {
            Ordering::Less => Some(mem::replace(
                self.frames
                    .get_mut(bit)
                    .expect("can't be out of bounds in this match arm"),
                pauli,
            )),
            Ordering::Equal => {
                self.frames.push(pauli);
                None
            }
            Ordering::Greater => {
                let diff = bit - len;
                self.frames.try_reserve(diff).unwrap_or_else(|e| {
                    panic!("error when trying to reserve enough memory: {e}")
                });
                self.frames
                    .extend(iter::repeat(PauliVec::<B>::default()).take(diff));
                self.frames.push(pauli);
                None
            }
        }
    }

    fn remove_pauli_stack(&mut self, bit: usize) -> Option<PauliVec<B>> {
        match bit.cmp(&(self.frames.len().checked_sub(1)?)) {
            Ordering::Less => panic!(
                "this type, Vector, only allows removing elements consecutively from \
                 the end"
            ),
            Ordering::Equal => Some(
                self.frames
                    .pop()
                    .expect("that's an implementation bug; please report"),
            ),
            Ordering::Greater => None,
        }
    }

    #[inline(always)]
    fn get(&self, qubit: usize) -> Option<&PauliVec<B>> {
        self.frames.get(qubit)
    }

    #[inline(always)]
    fn get_mut(&mut self, qubit: usize) -> Option<&mut PauliVec<B>> {
        self.frames.get_mut(qubit)
    }

    fn get_two_mut(
        &mut self,
        qubit_a: usize,
        qubit_b: usize,
    ) -> Option<(&mut PauliVec<B>, &mut PauliVec<B>)> {
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

#[cfg(test)]
mod tests {
    use std::panic;

    use coverage_helper::test;

    use super::*;
    use crate::pauli::{
        Pauli,
        PauliDense,
    };

    #[test]
    fn remove_and_insert() {
        type B = Vec<bool>;
        let mut pauli = PauliVec::<B>::zeros(2);
        pauli.push(PauliDense::new_x());
        let mut storage = Vector::<B>::init(1);
        assert_eq!(
            storage.insert_pauli_stack(0, pauli.clone()),
            Some(PauliVec::<B>::new())
        );
        assert_eq!(storage.insert_pauli_stack(1, pauli.clone()), None);
        assert_eq!(storage.insert_pauli_stack(3, pauli.clone()), None);
        assert!(
            panic::catch_unwind(|| {
                let mut storage = storage.clone();
                storage.remove_pauli_stack(1);
            })
            .is_err()
        );
        assert_eq!(storage.remove_pauli_stack(3), Some(pauli.clone()));
        assert_eq!(storage.remove_pauli_stack(3), None);
        assert_eq!(storage.remove_pauli_stack(2), Some(PauliVec::<B>::zeros(3)));
        assert_eq!(storage.remove_pauli_stack(2), None);
        assert_eq!(storage.remove_pauli_stack(1), Some(pauli.clone()));
        assert_eq!(storage.remove_pauli_stack(1), None);
        assert_eq!(storage.remove_pauli_stack(0), Some(pauli.clone()));
        assert!(storage.is_empty());
    }
}
