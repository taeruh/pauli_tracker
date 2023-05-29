use std::mem;

use bit_vec::BitVec;

use crate::pauli::Pauli;

/// Multiple encoded Paulis compressed into two [BitVec]s.
// each Pauli can be described by two bits (neglecting phases)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct PauliVec {
    // the bit representing the left qubit on the left-hand side in the tableau
    // representation, i.e., X
    pub left: BitVec,
    // right-hand side, i.e., Z
    pub right: BitVec,
}

impl TryFrom<(&str, &str)> for PauliVec {
    type Error = String;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        fn to_bool(c: char) -> Result<bool, String> {
            match c.to_digit(2) {
                Some(d) => Ok(d == 1),
                None => Err(format!("{} is not a valid binary", c)),
            }
        }

        Ok(PauliVec {
            left: value.0.chars().flat_map(to_bool).collect(),
            right: value.1.chars().flat_map(to_bool).collect(),
        })
    }
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

    pub fn push(&mut self, pauli: Pauli) {
        self.left.push(pauli.get_x());
        self.right.push(pauli.get_z());
    }

    pub fn pop_or_false(&mut self) -> Pauli {
        let l = self.left.pop().unwrap_or(false);
        let r = self.right.pop().unwrap_or(false);
        Pauli::new(l, r)
    }

    // pub fn reset(&mut self) {
    //     self.left.clear();
    //     self.right.clear();
    // }

    // we can define the action of local gates

    // Pauli gates don't do anything; we just include them for completeness and since it
    // might be more convenient to have them on the caller side
    /// Apply Pauli X, note that it is just the identity
    #[inline(always)]
    pub fn x(&self) {}
    /// Apply Pauli Z, note that it is just the identity
    #[inline(always)]
    pub fn z(&self) {}
    /// Apply Pauli Y, note that it is just the identity
    #[inline(always)]
    pub fn y(&self) {}

    /// Apply Hadamard
    #[inline]
    pub fn h(&mut self) {
        mem::swap(
            // Safety:
            // we don't do anything with the storage itself, so we should be good
            unsafe { self.left.storage_mut() },
            unsafe { self.right.storage_mut() },
        );
    }

    /// Apply Phase S
    #[inline]
    pub fn s(&mut self) {
        self.right.xor(&self.left);
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

/// This trait describes the functionality that a storage of [PauliVec]s must provide to
/// be used as storage for [Frames].
pub trait StackStorage: IntoIterator<Item = (usize, PauliVec)> {
    type IterMut<'a>: Iterator<Item = (usize, &'a mut PauliVec)>
    where
        Self: 'a;

    type Iter<'a>: Iterator<Item = (usize, &'a PauliVec)>
    where
        Self: 'a;

    fn insert_pauli(&mut self, qubit: usize, pauli: PauliVec) -> Option<PauliVec>;
    fn remove_pauli(&mut self, qubit: usize) -> Option<PauliVec>;
    fn get(&self, qubit: usize) -> Option<&PauliVec>;
    fn get_mut(&mut self, qubit: usize) -> Option<&mut PauliVec>;
    fn get_two_mut(
        &mut self,
        qubit_a: usize,
        qubit_b: usize,
    ) -> Option<(&mut PauliVec, &mut PauliVec)>;
    fn iter(&self) -> Self::Iter<'_>;
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
    fn init(num_qubits: usize) -> Self;
    fn is_empty(&self) -> bool;
}

pub fn sort_pauli_storage(storage: &impl StackStorage) -> Vec<(usize, &PauliVec)> {
    let mut ret = storage.iter().collect::<Vec<(usize, &PauliVec)>>();
    ret.sort_by_key(|(i, _)| *i);
    ret
}

pub fn into_sorted_pauli_storage(storage: impl StackStorage) -> Vec<(usize, PauliVec)> {
    let mut ret = storage.into_iter().collect::<Vec<(usize, PauliVec)>>();
    ret.sort_by_key(|(i, _)| *i);
    ret
}

pub fn layered_graph(
    storage: &impl StackStorage,
    map: &[usize],
) -> Vec<Vec<(usize, Vec<usize>)>> {
    let mut graph: Vec<Vec<(usize, Vec<usize>)>> = vec![Vec::new()];
    let mut remaining: Vec<(usize, Vec<usize>, Vec<usize>)> = Vec::new();

    // the first loop filters the dependencies and searches for qubits with no
    // dependencies
    for (bit, stack) in storage.iter() {
        let mut deps: Vec<usize> = Vec::new();
        for (dep, flag) in stack.left.iter().enumerate() {
            if flag {
                deps.push(map[dep]);
            }
        }
        for (dep, flag) in stack.right.iter().enumerate() {
            if flag {
                deps.push(map[dep]);
            }
        }
        if deps.is_empty() {
            graph[0].push((bit, deps));
        } else {
            remaining.push((bit, Vec::new(), deps));
        }
    }

    let mut register: Vec<usize> = Vec::new();
    let mut layer_idx = 0;

    while !remaining.is_empty() {
        let layer = graph.get(layer_idx).unwrap();
        let mut new_layer = Vec::new();
        for (known, deps) in layer.iter() {
            register.clear();
            for (bit, (_, resolved, open)) in remaining.iter_mut().enumerate() {
                if let Some(p) = open.iter().position(|&dep| dep == *known) {
                    let mut duplicates = Vec::new();
                    for (i, dep) in resolved.iter().enumerate() {
                        if deps.contains(dep) {
                            duplicates.push(i);
                        }
                    }
                    // want to remove the duplicates; this here should work, because
                    // duplicates is sorted with increasing order
                    for duplicate in duplicates.iter().rev() {
                        resolved.swap_remove(*duplicate);
                    }
                    resolved.push(open.swap_remove(p));
                    if open.is_empty() {
                        register.push(bit);
                    }
                }
            }
            for fully_resolved in register.iter().rev() {
                let (bit, deps, _) = remaining.swap_remove(*fully_resolved);
                new_layer.push((bit, deps));
            }
        }
        graph.push(new_layer);
        layer_idx += 1;
    }

    graph
}

mod fixed_vector;
pub use fixed_vector::FixedVector;

mod full_map;
pub use full_map::FullMap;

mod mapped_vector;
pub use mapped_vector::MappedVector;

// #[cfg(test)]
// mod tests {
//     // use super::*;

//     // First we test the methods of [FullMap] that are not just simple redirections.
//     // Then we use [FullMap] to as reference to test the other storages

//     #[test]
//     fn full_map() {
//         /* all trivial */
//     }

//     #[test]
//     fn mapped_vec() {
//         // do some fuzzing using dispatch_storage_operation_comparison below
//     }
// }

// #[cfg(test)]
// fn dispatch_storage_operation_comparison(
//     storage: &mut (impl PauliStorage + PartialEq + Clone),
//     other: &mut FullMap,
//     operation: u8,
//     bit: usize,
// ) {
//     let operation = operation % 3;
//     match operation {
//         0 => {
//             assert_eq!(
//                 storage.insert_pauli(bit, PauliVec::new()),
//                 other.insert_pauli(bit, PauliVec::new())
//             );
//         }
//         1 => {
//             assert_eq!(storage.remove_pauli(bit), other.remove_pauli(bit));
//         }
//         2 => {
//             assert_eq!(storage.get(bit), other.get(&bit));
//         }
//         _ => {}
//     }
//     let compare = FullMap::from_iter(storage.clone().into_iter());
// }
