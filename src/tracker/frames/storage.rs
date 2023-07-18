/*!
The underlining storage types for [Frames](super::Frames) and some functionality to
analyze the storage.
*/

use crate::{
    boolean_vector::BooleanVector,
    pauli::PauliVec,
};

/// This trait describes the functionality that a storage of [PauliVec]s must provide to
/// be used as storage for [Frames](super::Frames).
// instead of requiring that &T and &mut T implement IntoIterator, we have the iter and
// iter_mut methods, respectively; the reason is that having the additional bounds would
// either need an annoying lifetime or HRTBs, the latter would limit the use cases of
// the trait (for <'l> &'l T implies T: 'static); implementors of this type should
// probably still implement IntoIterator for its references
pub trait StackStorage:
    IntoIterator<Item = (usize, PauliVec<Self::BoolVec>)>
    + FromIterator<(usize, PauliVec<Self::BoolVec>)>
{
    /// The storage type used for [PauliVec].
    type BoolVec: BooleanVector;

    /// An iterator over the storage. The items are tuples of the qubits with references
    /// to their corresponding [PauliVec] stack.
    type Iter<'l>: Iterator<Item = (usize, &'l PauliVec<Self::BoolVec>)>
    where
        Self: 'l;

    /// An iterator over the storage. The items are tuples of the qubits with mutalbe
    /// references to their corresponding [PauliVec] stack.
    type IterMut<'l>: Iterator<Item = (usize, &'l mut PauliVec<Self::BoolVec>)>
    where
        Self: 'l;

    /// Initialize the storage to keep `num_bits` Pauli stacks, numbered from 0 to
    /// `num_bits` - 1.
    ///
    /// # Example
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// use pauli_tracker::{
    ///     pauli::PauliVec,
    ///     tracker::frames::storage::{
    ///         Map,
    ///         StackStorage,
    ///     },
    /// };
    /// let storage = Map::<Vec<bool>>::init(2);
    /// assert_eq!(storage.get(&0), Some(&PauliVec::<Vec<bool>>::new()));
    /// assert_eq!(storage.get(&1), Some(&PauliVec::<Vec<bool>>::new()));
    /// assert_eq!(storage.get(&2), None);
    /// # }
    /// ```
    fn init(num_bits: usize) -> Self;

    /// Check whether the storage is empty.
    fn is_empty(&self) -> bool;

    /// Insert a `pauli` stack for qu`bit`. If the qu`bit` is already present,
    /// its stack is overwritten an the old value, [Some](Some)(\<old stack\>), is
    /// returned.
    fn insert_pauli_stack(
        &mut self,
        bit: usize,
        pauli: PauliVec<Self::BoolVec>,
    ) -> Option<PauliVec<Self::BoolVec>>;

    /// Remove a qu`bit` and its stack from the storage. If the qubit is present, its
    /// stack is returneddd, otherwise [None].
    fn remove_pauli_stack(&mut self, bit: usize) -> Option<PauliVec<Self::BoolVec>>;

    /// Get a references to qu`bit`s stack, if present, otherwise return [None].
    fn get(&self, bit: usize) -> Option<&PauliVec<Self::BoolVec>>;

    /// Get a mutalbe references to qu`bit`s stack, if present, otherwise return [None].
    fn get_mut(&mut self, bit: usize) -> Option<&mut PauliVec<Self::BoolVec>>;
    #[allow(clippy::type_complexity)]

    /// Get two mutable references to distinct elements.
    ///
    /// # Panics
    /// Panics if the two references point to the same object, i.e., if `bit_a` =
    /// `bit_b`.
    fn get_two_mut(
        &mut self,
        bit_a: usize,
        bit_b: usize,
    ) -> Option<(&mut PauliVec<Self::BoolVec>, &mut PauliVec<Self::BoolVec>)>;

    /// Get an [Iterator] over the tuples of qubits and references of the corresponding
    /// Pauli stacks.
    fn iter(&self) -> Self::Iter<'_>;

    /// Get an [Iterator] over the tuples of qubits and mutable references of the
    /// corresponding Pauli stacks.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    /// Sort the `storage` according to the qubits numbers.
    fn sort_by_bit(&self) -> Vec<(usize, &PauliVec<Self::BoolVec>)> {
        let mut ret = self.iter().collect::<Vec<(usize, &PauliVec<Self::BoolVec>)>>();
        ret.sort_by_key(|(i, _)| *i);
        ret
    }

    /// Convert the `storage` into a sorted array according to the qubits numbers.
    fn into_sorted_by_bit(self) -> Vec<(usize, PauliVec<Self::BoolVec>)> {
        let mut ret =
            self.into_iter().collect::<Vec<(usize, PauliVec<Self::BoolVec>)>>();
        ret.sort_by_key(|(i, _)| *i);
        ret
    }

    /// Sort the `storage` according to the induced dependencies by the frames (row through
    /// the PauliVecs). Each frame in `storage` maps to a qubit number in `map`; frame (i)
    /// -> `map`\[i\]. If a qubit's Pauli stack has non-zero elements in a frame (i), the
    /// qubit is assumed to depend on `map`\[i\]
    ///
    /// Dependencies that are already covered by later dependencies, i.e., dependencies that
    /// are in a higher layer, are removed. For example if 0 depends on 1 and 2 but 1 also
    /// depends on 2, then 2 is not listed in the dependencies of 0.
    ///
    /// Note that while the sorting is deterministic, up to `storage`'s Iterator
    /// implementation, the output might not be sorted as expected, since nodes are swapped
    /// around for better efficiency.
    ///
    /// # Panics
    /// May panic if `map`.len() < number of frames in storage (out of bounds).
    ///
    /// # Examples
    /// ```
    /// # #[cfg_attr(coverage_nightly, no_coverage)]
    /// # fn main() {
    /// use pauli_tracker::{
    ///     pauli::PauliVec,
    ///     tracker::frames::storage::{
    ///         StackStorage,
    ///         Vector,
    ///     },
    /// };
    /// let storage = Vector {
    ///     frames: vec![
    ///         PauliVec::<Vec<bool>>::try_from_str("", "").unwrap(),
    ///         PauliVec::<Vec<bool>>::try_from_str("10", "00").unwrap(),
    ///         PauliVec::<Vec<bool>>::try_from_str("01", "10").unwrap(),
    ///         PauliVec::<Vec<bool>>::try_from_str("1", "0").unwrap(),
    ///     ],
    /// };
    /// let map = vec![0, 3];
    /// assert_eq!(
    ///     storage.create_dependency_graph(&map),
    ///     vec![
    ///         vec![(0, vec![])],
    ///         vec![(3, vec![0]), (1, vec![0])],
    ///         vec![(2, vec![3])], // note that the redundent dependency on 0 is removed
    ///     ]
    /// );
    /// # }
    /// ```
    fn create_dependency_graph(&self, map: &[usize]) -> DependencyGraph {
        let mut graph: Vec<Vec<(usize, Vec<usize>)>> = vec![Vec::new()];
        let mut remaining: Vec<(usize, Vec<usize>, Vec<usize>)> = Vec::new();

        assert!(!map.is_empty(), "map must not be empty");

        // the first loop filters the dependencies and searches for qubits with no
        // dependencies
        for (bit, stack) in self.iter() {
            let mut deps: Vec<usize> = Vec::new();

            let max = stack.left.len().max(stack.right.len());
            let mut left = stack.left.clone();
            left.resize(max, false);
            let mut right = stack.right.clone();
            right.resize(max, false);
            left.or_inplace(&right);

            for (dep, flag) in left.iter_vals().enumerate() {
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

        assert!(
            !graph[0].is_empty(),
            "couldn't find any independent qubit; maybe the
            storage was empty?"
        );

        let mut layer_idx = 0;

        while !remaining.is_empty() {
            let mut new_layer = Vec::new();
            for (known, deps) in graph.get(layer_idx).unwrap().iter() {
                let mut register = Vec::new();
                for (bit, (_, resolved, open)) in remaining.iter_mut().enumerate() {
                    if let Some(resolved_idx) =
                        open.iter().position(|&dep| dep == *known)
                    {
                        let redundent_deps: Vec<usize> =
                            resolved
                                .iter()
                                .enumerate()
                                .filter_map(|(i, dep)| {
                                    if deps.contains(dep) { Some(i) } else { None }
                                })
                                .collect();
                        // want to remove the redundent deps; the swap_remove works, because
                        // redundent_deps is sorted with increasing order
                        for redundent in redundent_deps.iter().rev() {
                            resolved.swap_remove(*redundent);
                        }
                        resolved.push(open.swap_remove(resolved_idx));
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

            assert!(
                !new_layer.is_empty(),
                "couldn't find qubit with resolved deps in layer {}",
                layer_idx + 1
            );

            graph.push(new_layer);
            layer_idx += 1;
        }

        graph
    }
}

/// A layered graph, describing the how the qubits depend on each other. Each layer l =
/// DependencyGraph\[i\] consist of an vector of tuples, where the first tuple element is
/// the node qubits and the second tuple element contains all qubits on which the node
/// qubit depends.
pub type DependencyGraph = Vec<Vec<(usize, Vec<usize>)>>;

/// Sort the nodes in a layer of `graph` according to their qubit number.
///
/// # Examples
/// ```
/// # #[cfg_attr(coverage_nightly, no_coverage)]
/// # fn main() {
/// # use pauli_tracker::tracker::frames::storage::sort_layers_by_bits;
/// let mut graph = vec![vec![(0, vec![])], vec![(3, vec![0]), (1, vec![0])]];
/// sort_layers_by_bits(&mut graph);
///
/// assert_eq!(graph, vec![vec![(0, vec![])], vec![(1, vec![0]), (3, vec![0])],]);
/// # }
/// ```
pub fn sort_layers_by_bits(graph: &mut DependencyGraph) {
    for layer in graph {
        layer.sort_by_key(|(bit, _)| *bit)
    }
}

mod vector;
pub use vector::Vector;

mod map;
pub use map::Map;

mod mapped_vector;
#[allow(unused)] // we're using it in some tests
pub(crate) use mapped_vector::MappedVector;

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;
    use crate::tracker::frames::storage::{
        StackStorage,
        Vector,
    };

    // // First we test the methods of [FullMap] that are not just simple redirections.
    // // Then we use [FullMap] to as reference to test the other storages

    // #[test]
    // fn full_map() {
    //     /* all trivial */
    // }

    // #[test]
    // fn mapped_vec() {
    //     // do some fuzzing using dispatch_storage_operation_comparison below
    // }

    #[test]
    #[should_panic]
    fn graph_no_first_layer() {
        let storage = Vector {
            frames: vec![PauliVec::<Vec<bool>>::try_from_str("1", "0").unwrap()],
        };
        let map = vec![42];
        storage.create_dependency_graph(&map);
    }

    #[test]
    #[should_panic]
    fn graph_no_new_layer() {
        let storage = Vector {
            frames: vec![
                PauliVec::<Vec<bool>>::try_from_str("", "").unwrap(),
                PauliVec::<Vec<bool>>::try_from_str("10", "00").unwrap(),
                PauliVec::<Vec<bool>>::try_from_str("01", "00").unwrap(),
            ],
        };
        let map = vec![1, 2];
        storage.create_dependency_graph(&map);
    }
}

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
