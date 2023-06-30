/*!
... tools to analyse the tracking results ...
*/

#[allow(unused)]
pub(crate) mod space;

use std::collections::HashMap;

use crate::{
    boolean_vector::BooleanVector,
    pauli::PauliVec,
};

/// A layered graph, describing the how the qubits depend on each other. Each layer l =
/// DependencyGraph\[i\] consist of an vector of tuples, where the first tuple element is
/// the node qubits and the second tuple element contains all qubits on which the node
/// qubit depends.
pub type DependencyGraph = Vec<Vec<(usize, Vec<usize>)>>;

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
/// # use pauli_tracker::analyse::create_dependency_graph;
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
///     create_dependency_graph(storage.iter(), &map),
///     vec![
///         vec![(0, vec![])],
///         vec![(3, vec![0]), (1, vec![0])],
///         vec![(2, vec![3])], // note that the redundent dependency on 0 is removed
///     ]
/// );
/// # }
/// ```
pub fn create_dependency_graph<'l, BoolVec, Storage>(
    storage: Storage,
    map: &[usize],
) -> DependencyGraph
where
    BoolVec: BooleanVector + 'l,
    Storage: IntoIterator<Item = (usize, &'l PauliVec<BoolVec>)>,
{
    let mut graph: Vec<Vec<(usize, Vec<usize>)>> = vec![Vec::new()];
    let mut remaining: Vec<(usize, Vec<usize>, Vec<usize>)> = Vec::new();

    // the first loop filters the dependencies and searches for qubits with no
    // dependencies
    for (bit, stack) in storage {
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

    assert!(!graph[0].is_empty(), "couldn't find any independent qubit");

    let mut layer_idx = 0;

    while !remaining.is_empty() {
        let mut new_layer = Vec::new();
        for (known, deps) in graph.get(layer_idx).unwrap().iter() {
            let mut register = Vec::new();
            for (bit, (_, resolved, open)) in remaining.iter_mut().enumerate() {
                if let Some(resolved_idx) = open.iter().position(|&dep| dep == *known) {
                    let redundent_deps: Vec<usize> = resolved
                        .iter()
                        .enumerate()
                        .filter_map(
                            |(i, dep)| if deps.contains(dep) { Some(i) } else { None },
                        )
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

/// Sort the nodes in a layer of `graph` according to their qubit number.
///
/// # Examples
/// ```
/// # #[cfg_attr(coverage_nightly, no_coverage)]
/// # fn main() {
/// # use pauli_tracker::analyse::sort_layers_by_bits;
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

/// graph into hashmap
pub fn into_hashmap(graph: DependencyGraph) -> HashMap<usize, Vec<usize>> {
    HashMap::from_iter(graph.into_iter().flatten())
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;
    use crate::tracker::frames::storage::{
        StackStorage,
        Vector,
    };

    #[test]
    #[should_panic]
    fn graph_no_first_layer() {
        let storage = Vector {
            frames: vec![PauliVec::<Vec<bool>>::try_from_str("1", "0").unwrap()],
        };
        let map = vec![42];
        create_dependency_graph(storage.iter(), &map);
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
        create_dependency_graph(storage.iter(), &map);
    }
}
