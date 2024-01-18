/*!
The main content of this module is the [get_order] function that can be
used to define a time ordering induced by the tracked frames.
*/

use crate::{
    boolean_vector::BooleanVector,
    pauli::PauliStack,
};

/// A layered graph, describing the partial (time) ordering of the qubits.
///
/// Each layer l_i = PartialOrderGraph\[i\] consist of a vector of tuples, where the
/// first tuple element is the node qubit and the second tuple element contains the
/// qubits which are lower ordered than the node qubit (the dependencies of the node
/// qubit). However, transitivity is skipped, e.g., if we have 0 > 1 > 2, then we list
/// only 1 as dependency of 0, since the information 0 > 2 is already covered in 0 > 1 and
/// 1 > 2. The layering gives some global information: A node in layer l_i has at least
/// one dependency in layer l_{i-1}.
pub type PartialOrderGraph = Vec<Vec<(usize, Vec<usize>)>>;

/// Sort the `frames_storage`'s qubits according to the induced dependencies by the
/// frames (row through the PauliStacks).
///
/// Each frame in `frames_storage` maps to a qubit number in `map`; frame(i) ->
/// `map`\[i\]. If a qubit's Pauli stack has non-zero elements in a frame(i), the qubit
/// is assumed to depend on `map`\[i\].
///
/// Note that while the sorting is deterministic, up to `frames_storage`'s Iterator
/// implementation, the output might not be sorted as expected, since nodes are swapped
/// around for better efficiency.
///
/// # Panics
/// The input has to make "sense", i.e., the `map` must not be empty, there, shouldn't
/// be dependency cycles, etc. The algorithm loops through the qubits, searching for
/// qubits whose dependecies are already in the graph. If there are no such qubits, it
/// panics.
///
/// # Examples
/// ```
/// # #[cfg_attr(coverage_nightly, coverage(off))]
/// # fn main() {
/// use pauli_tracker::{
///     collection::BufferedVector,
///     pauli::PauliStack,
///     tracker::frames::induced_order,
/// };
/// let storage = BufferedVector::from(vec![
///     PauliStack::<Vec<bool>>::try_from_str("", "").unwrap(),
///     PauliStack::<Vec<bool>>::try_from_str("10", "00").unwrap(),
///     PauliStack::<Vec<bool>>::try_from_str("01", "10").unwrap(),
///     PauliStack::<Vec<bool>>::try_from_str("1", "0").unwrap(),
/// ]);
/// let map = vec![0, 3];
/// assert_eq!(
///     induced_order::get_order(&storage, &map),
///     vec![
///         vec![(0, vec![])],
///         vec![(3, vec![0]), (1, vec![0])],
///         vec![(2, vec![3])], // note that the redundent dependency on 0 is removed
///     ]
/// );
/// # }
/// ```
pub fn get_order<'l, I, B>(frames_storage: I, map: &[usize]) -> PartialOrderGraph
where
    I: IntoIterator<Item = (usize, &'l PauliStack<B>)>,
    B: BooleanVector + 'l,
{
    let mut graph: Vec<Vec<(usize, Vec<usize>)>> = vec![Vec::new()];
    let mut remaining: Vec<(usize, Vec<usize>, Vec<usize>)> = Vec::new();

    assert!(!map.is_empty(), "map must not be empty");

    // the first loop filters the dependencies and searches for qubits with no
    // dependencies
    for (bit, stack) in frames_storage {
        let mut deps: Vec<usize> = Vec::new();

        let max = stack.z.len().max(stack.x.len());
        let mut z = stack.z.clone();
        z.resize(max, false);
        let mut x = stack.x.clone();
        x.resize(max, false);
        z.or_inplace(&x);

        for (dep, flag) in z.iter_vals().enumerate() {
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
        "couldn't find any independent qubit; maybe the storage was empty?"
    );

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
                            |(i, dep)| {
                                if deps.contains(dep) { Some(i) } else { None }
                            },
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
            "couldn't find qubit with resolved dependencies in layer {}",
            layer_idx + 1
        );

        graph.push(new_layer);
        layer_idx += 1;
    }

    graph
}

/// Sort the nodes in a layer of the `graph` according to their qubit number.
///
/// # Examples
/// ```
/// # #[cfg_attr(coverage_nightly, coverage(off))]
/// # fn main() {
/// # use pauli_tracker::tracker::frames::induced_order::sort_layers_by_bits;
/// let mut graph = vec![vec![(0, vec![])], vec![(3, vec![0]), (1, vec![0])]];
/// sort_layers_by_bits(&mut graph);
///
/// assert_eq!(graph, vec![vec![(0, vec![])], vec![(1, vec![0]), (3, vec![0])],]);
/// # }
/// ```
pub fn sort_layers_by_bits(graph: &mut PartialOrderGraph) {
    for layer in graph {
        layer.sort_by_key(|(bit, _)| *bit)
    }
}
