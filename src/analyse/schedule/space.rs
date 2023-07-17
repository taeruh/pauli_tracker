/*!
...
*/

use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    mem,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::tree::Focus;

type Node = (State, Vec<usize>);
type Nodes = Vec<Node>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum State {
    #[default]
    Sleeping,
    InMemory,
    Measured,
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Graph {
    nodes: Nodes,
    current_memory: usize,
    max_memory: usize,
}

macro_rules! new_loop {
    ($nodes:expr, $edges:expr, $bit_mapping:expr, $check:tt) => {
        if let Some(bit_mapping) = $bit_mapping {
            for (left, right) in $edges.iter() {
                let left = *update!(left, bit_mapping);
                let right = *update!(right, bit_mapping);
                new_body!(left, right, $nodes, $check);
            }
        } else {
            for (left, right) in $edges.iter() {
                new_body!(*left, *right, $nodes, $check);
            }
        }
    };
}

macro_rules! new_body {
    ($left:expr, $right:expr, $nodes:expr,checked) => {
        if $left == $right {
            continue;
        }
        if $nodes[$left].1.contains(&$right) {
            continue;
        }
        new_body!($left, $right, $nodes, unchecked);
    };
    ($left:expr, $right:expr, $nodes:expr,unchecked) => {
        $nodes[$left].1.push($right);
        $nodes[$right].1.push($left);
    };
}

impl Graph {
    pub fn new(
        num_bits: usize,
        edges: &[(usize, usize)],
        bit_mapping: Option<&HashMap<usize, usize>>,
        check_duplicates: bool,
    ) -> Self {
        let mut nodes = vec![Node::default(); num_bits];

        if check_duplicates {
            new_loop!(nodes, edges, bit_mapping, checked);
        } else {
            new_loop!(nodes, edges, bit_mapping, unchecked);
        }

        Self {
            nodes,
            current_memory: 0,
            max_memory: 0,
        }
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn current_memory(&self) -> usize {
        self.current_memory
    }

    pub fn max_memory(&self) -> usize {
        self.max_memory
    }

    fn initialize(&mut self, bit: usize) -> Result<(), AlreadyMeasured> {
        match &mut self.nodes[bit].0 {
            state @ State::Sleeping => {
                *state = State::InMemory;
                self.current_memory += 1;
            }
            State::InMemory => (),
            State::Measured => {
                return Err(AlreadyMeasured {
                    bit,
                    operation: Operation::Initialize,
                });
            }
        }
        Ok(())
    }

    fn measure(&mut self, bit: usize) -> Result<(), AlreadyMeasured> {
        let neighbors = mem::take(&mut self.nodes[bit].1);
        for neighbor in neighbors.iter() {
            let _ = self.initialize(*neighbor); // Err is okay
        }
        let node = &mut self.nodes[bit];
        let _ = mem::replace(&mut node.1, neighbors);
        match node.0 {
            State::Sleeping => {
                self.current_memory += 1;
                node.0 = State::Measured;
            }
            State::InMemory => node.0 = State::Measured,
            State::Measured => {
                return Err(AlreadyMeasured {
                    bit,
                    operation: Operation::Measure,
                });
            }
        }
        Ok(())
    }
}

impl Focus<&[usize]> for Graph {
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
        if self.current_memory > self.max_memory {
            self.max_memory = self.current_memory;
        }
        self.current_memory -= measure_set.len();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AlreadyMeasured {
    bit: usize,
    operation: Operation,
}
#[derive(Debug, Clone)]
pub enum Operation {
    Initialize,
    Measure,
}
impl Display for AlreadyMeasured {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cannot perform operation \"{}\" on already measured bit {}",
            match self.operation {
                Operation::Initialize => "initialize",
                Operation::Measure => "measure",
            },
            self.bit
        )
    }
}
impl Error for AlreadyMeasured {}

#[cfg(test)]
pub(crate) mod tests {
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

    #[cfg_attr(coverage_nightly, no_coverage)]
    pub fn example_graph() -> Graph {
        //     1
        //   /  \
        // 0     3
        //   \  /
        //     2
        Graph::new(5, &GM, None, false)
    }

    #[test]
    fn creation() {
        let mp = HashMap::from([(8, 0), (5, 2)]);
        let graph = Graph::new(NUM, &GN, Some(&mp), false);
        let mapped = Graph::new(NUM, &GM, None, false);
        let graph_checked = Graph::new(NUM, &GNW, Some(&mp), true);
        let mapped_checked = Graph::new(NUM, &GMW, None, true);
        assert_eq!(graph, mapped);
        assert_eq!(graph, graph_checked);
        assert_eq!(graph, mapped_checked);
        assert_eq!(
            graph,
            Graph {
                nodes: vec![
                    (Sleeping, vec![1, 2]),
                    (Sleeping, vec![0, 3]),
                    (Sleeping, vec![0, 3]),
                    (Sleeping, vec![1, 2]),
                ],
                current_memory: 0,
                max_memory: 0,
            }
        );
    }

    #[test]
    fn updating() {
        let init_graph = example_graph();
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
