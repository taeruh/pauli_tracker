/*!
This module provides a [Graph] to describe the initialization and measuring process of
qubits in a [graph state].

When realizing a [graph state] one does not necessarily need to initialize - having
a qubit in quantum memory - all qubits at once. Instead, one can initialize them one by
one, and if they are measured, reuse the freed quantum memory. However, to measure a
node, all its neighbors need to be initialized, otherwise it would have been impossible
to create the edges.

[graph state]: https://en.wikipedia.org/wiki/Graph_state
*/

use hashbrown::HashMap;
use thiserror::Error;

use super::tree::Focus;

/// A single node, containing the state of the qubit and the edges to other qubits. The
/// edges are usually owned by a [GraphBuffer].
pub type Node<'l> = (State, &'l Vec<usize>);

/// Multiple nodes.
pub type Nodes<'l> = Vec<Node<'l>>;

/// Possible states of a qubit node in the graph.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum State {
    /// Not initialized, i.e., not in quantum memory.
    #[default]
    Sleeping,
    /// Initialized, i.e., in node quantum, and can be measured.
    InMemory,
    /// Measured!
    Measured,
}

/// A buffer that holds the edges of a graph.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphBuffer {
    inner: Vec<Vec<usize>>,
}

/// A graph that holds the information about the states of the qubits and the edges
/// between them.
///
/// While initializing and measuring qubits, the graph keeps track of the required
/// quantum memory.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Graph<'l> {
    nodes: Nodes<'l>,
    current_memory: usize,
    max_memory: usize,
}

// not sure how idiomatic it is to do this macro stuff here; a proc macro might be
// cleaner, but it is not necessary and probably wouldn't lead to less code; does
// someone knows a better way (without copy paste or more runtime operations)?

macro_rules! new_loop {
    ($inner:expr, $edges:expr, $bit_mapping:expr, $check:tt) => {
        if let Some(bit_mapping) = $bit_mapping {
            for (left, right) in $edges.iter() {
                let left = *update!(left, bit_mapping);
                let right = *update!(right, bit_mapping);
                new_body!(left, right, $inner, $check);
            }
        } else {
            for (left, right) in $edges.iter() {
                new_body!(*left, *right, $inner, $check);
            }
        }
    };
}
macro_rules! new_body {
    ($left:expr, $right:expr, $inner:expr,checked) => {
        if $left == $right {
            continue;
        }
        if $inner[$left].contains(&$right) {
            continue;
        }
        new_body!($left, $right, $inner, unchecked);
    };
    ($left:expr, $right:expr, $inner:expr,unchecked) => {
        $inner[$left].push($right);
        $inner[$right].push($left);
    };
}

impl GraphBuffer {
    /// Create a new buffer for our graph.
    ///
    /// `edges` is a list of the edges and `num_nodes` holds the number of nodes in the
    /// graph. The [Graph], and its buffer, require that the nodes are numbered from 0
    /// to `num_nodes - 1`! To accomplish this, if needed, `bit_mapping` can be used,
    /// whose keys are the original node ids and whose values are the new node ids.
    /// Loops and Multi-edges are not allowed. If `check_loop_multi_edge` is true, then
    /// the function will check for those and skip them.
    pub fn new(
        edges: &[(usize, usize)],
        num_nodes: usize,
        bit_mapping: Option<&HashMap<usize, usize>>,
        check_loop_multi_edge: bool,
    ) -> Self {
        let mut inner = vec![Vec::new(); num_nodes];
        if check_loop_multi_edge {
            new_loop!(inner, edges, bit_mapping, checked);
        } else {
            new_loop!(inner, edges, bit_mapping, unchecked);
        }
        Self { inner }
    }

    /// Create a new buffer from a sparse representation of the graph.
    ///
    /// The same requirements as for [GraphBuffer::new] apply!
    pub fn from_sparse(value: Vec<Vec<usize>>) -> Self {
        Self { inner: value }
    }
}

impl<'l> Graph<'l> {
    /// Create a freshly initialized graph from a `graph_buffer`.
    pub fn new(graph_buffer: &'l GraphBuffer) -> Self {
        Self {
            nodes: graph_buffer
                .inner
                .iter()
                .map(|neighbors| (State::Sleeping, neighbors))
                .collect(),
            current_memory: 0,
            max_memory: 0,
        }
    }

    /// Get a reference to the nodes of the graph.
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    /// Get the number of qubits which are currently in quantum memory.
    pub fn current_memory(&self) -> usize {
        self.current_memory
    }

    /// Get the, so far, maximum required quantum memory.
    pub fn max_memory(&self) -> usize {
        self.max_memory
    }

    fn initialize(&mut self, bit: usize) {
        match &mut self.nodes[bit].0 {
            state @ State::Sleeping => {
                *state = State::InMemory;
                self.current_memory += 1;
            },
            State::InMemory => (),
            State::Measured => {},
        }
    }
}

macro_rules! impl_measure {
    ($name:ident, $check:tt) => {
        fn $name(&mut self, bit: usize) -> return_type!($check) {
            let node = &mut self.nodes[bit];
            match node.0 {
                State::Sleeping => {
                    // corrected later on in self.update_memory
                    self.current_memory += 1;
                    node.0 = State::Measured;
                },
                State::InMemory => node.0 = State::Measured,
                State::Measured => {
                    return return_error!($check, bit);
                },
            }
            for neighbor in node.1.iter() {
                self.initialize(*neighbor);
            }
            return_ok!($check)
        }
    };
}
macro_rules! return_type {
    (checked) => {
        Result<(), AlreadyMeasured>
    };
    (unchecked) => {
        ()
    };
}
macro_rules! return_error {
    (checked, $bit:expr) => {
        Err(AlreadyMeasured($bit))
    };
    (unchecked, $bit:expr) => {
        ()
    };
}
macro_rules! return_ok {
    (checked) => {
        Ok(())
    };
    (unchecked) => {
        ()
    };
}

impl<'l> Graph<'l> {
    impl_measure!(measure, checked);

    fn update_memory(&mut self, len: usize) {
        if self.current_memory > self.max_memory {
            self.max_memory = self.current_memory;
        }
        self.current_memory -= len; // correct ...
    }
}

impl<'l> Focus<&[usize]> for Graph<'l> {
    type Error = AlreadyMeasured;
    fn focus(&mut self, instruction: &[usize]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let mut new = self.clone();
        new.focus_inplace(instruction)?;
        Ok(new)
    }
    fn focus_inplace(&mut self, measure_set: &[usize]) -> Result<(), Self::Error> {
        for bit in measure_set {
            self.measure(*bit)?;
        }
        self.update_memory(measure_set.len());
        Ok(())
    }
}

#[cfg(not(debug_assertions))]
impl<'l> Graph<'l> {
    impl_measure!(measure_unchecked, unchecked);
    pub(super) fn focus_inplace_unchecked(&mut self, measure_set: &[usize]) {
        for bit in measure_set {
            self.measure_unchecked(*bit);
        }
        self.update_memory(measure_set.len());
    }
    pub(super) fn focus_unchecked(&self, measure_set: &[usize]) -> Self {
        let mut new = self.clone();
        new.focus_inplace_unchecked(measure_set);
        new
    }
}

/// Error type when a bit is measured multiple times.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("bit \"{0}\" has been already measured")]
pub struct AlreadyMeasured(pub usize);

#[cfg(test)]
mod tests {
    use coverage_helper::test;
    use State::*;

    use super::*;
    // use crate::analyse::schedule::time::{
    //     Partitioner,
    //     PathGenerator,
    // };

    const NUM: usize = 4;
    const GN: [(usize, usize); 4] = [(8, 1), (8, 5), (1, 3), (3, 5)];
    const GM: [(usize, usize); 4] = [(0, 1), (0, 2), (1, 3), (3, 2)];
    const GNW: [(usize, usize); 6] = [(8, 1), (8, 5), (8, 1), (1, 3), (3, 5), (5, 5)];
    const GMW: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 1), (1, 3), (3, 2), (2, 2)];

    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn example_graph() -> GraphBuffer {
        //     1
        //   /  \
        // 0     3
        //   \  /
        //     2
        GraphBuffer::new(&GM, 5, None, false)
    }

    #[test]
    fn creation() {
        let mp = HashMap::from([(8, 0), (5, 2)]);
        let graph_buffer = GraphBuffer::new(&GN, NUM, Some(&mp), false);
        let mapped_buffer = GraphBuffer::new(&GM, NUM, None, false);
        let graph_checked_buffer = GraphBuffer::new(&GNW, NUM, Some(&mp), true);
        let mapped_checked_buffer = GraphBuffer::new(&GMW, NUM, None, true);
        let graph = Graph::new(&graph_buffer);
        assert_eq!(graph, Graph::new(&mapped_buffer));
        assert_eq!(graph, Graph::new(&graph_checked_buffer));
        assert_eq!(graph, Graph::new(&mapped_checked_buffer));
        assert_eq!(
            graph,
            Graph {
                nodes: vec![
                    (Sleeping, &vec![1, 2]),
                    (Sleeping, &vec![0, 3]),
                    (Sleeping, &vec![0, 3]),
                    (Sleeping, &vec![1, 2]),
                ],
                current_memory: 0,
                max_memory: 0,
            }
        );
    }

    #[test]
    fn updating() {
        let init_buffer = example_graph();
        let init_graph = Graph::new(&init_buffer);
        let mut graph = init_graph.clone();
        let new = graph.focus(&[2, 3]).unwrap();
        graph.focus_inplace(&[2, 3]).unwrap();
        assert_eq!(graph, new);
        let mut manually = init_graph.clone();
        manually.nodes[2].0 = Measured;
        manually.nodes[3].0 = Measured;
        manually.nodes[0].0 = InMemory;
        manually.nodes[1].0 = InMemory;
        manually.current_memory = 2; // 4 -> 2
        manually.max_memory = 4;
        let mut graph = init_graph;
        graph.focus_inplace(&[2]).unwrap();
        graph.focus_inplace(&[3]).unwrap();
        manually.max_memory = 3; // current_memory: 3 -> 2 -> 3 -> 2
        assert_eq!(graph, manually);
    }
}
