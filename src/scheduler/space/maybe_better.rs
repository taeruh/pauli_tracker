use std::{
    mem,
    slice,
};

use itertools::Itertools;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum State {
    #[default]
    Sleeping,
    InMemory(usize),
    Measured(usize),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    state: State,
    neighbors: Vec<usize>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    current: usize,
    max: usize,
    // the updating of current_max is not bijective, we need to keep the history
    max_history: Vec<usize>,
}

impl Graph {
    pub fn new(num: usize, edges: &[(usize, usize)]) -> Self {
        let mut nodes = vec![Node::default(); num];
        for &(left, right) in edges {
            assert!(left != right, "no loops allowed");
            nodes[left].neighbors.push(right);
            nodes[right].neighbors.push(left);
        }
        Self {
            nodes,
            current: 0,
            max: 0,
            max_history: Vec::with_capacity(num),
        }
    }

    fn initialize(&mut self, bit: usize) -> Result<(), ()> {
        match &mut self.nodes[bit].state {
            state @ State::Sleeping => {
                *state = State::InMemory(1);
                self.current += 1;
            }
            State::InMemory(i) => *i += 1,
            State::Measured(_) => return Err(()),
        }
        Ok(())
    }

    fn reverse_initialization(&mut self, bit: usize) -> Result<(), ()> {
        match &mut self.nodes[bit].state {
            State::Sleeping => return Err(()),
            state @ State::InMemory(1) => {
                *state = State::Sleeping;
                self.current -= 1;
            }
            State::InMemory(i) => *i -= 1,
            State::Measured(_) => return Err(()),
        }
        Ok(())
    }

    pub fn measure(&mut self, bit: usize) -> Result<(), ()> {
        let neighbors = mem::take(&mut self.nodes[bit].neighbors);
        for neighbor in neighbors.iter() {
            let _ = self.initialize(*neighbor);
        }
        let node = &mut self.nodes[bit];
        mem::replace(&mut node.neighbors, neighbors);
        match node.state {
            State::Sleeping => {
                self.current += 1;
                node.state = State::Measured(0)
            }
            State::InMemory(i) => node.state = State::Measured(i),
            State::Measured(i) => return Err(()),
        }
        self.max_history.push(self.max);
        if self.current > self.max {
            self.max = self.current;
        }
        self.current -= 1;
        Ok(())
    }

    pub fn reverse_measure(&mut self, bit: usize) -> Result<(), ()> {
        let node = &mut self.nodes[bit];
        let inits = match node.state {
            State::Sleeping => return Err(()),
            State::InMemory(_) => return Err(()),
            State::Measured(i) => i,
        };
        let neighbors = mem::take(&mut node.neighbors);
        for neighbor in neighbors.iter() {
            let _ = self.reverse_initialization(*neighbor);
        }
        let node = &mut self.nodes[bit];
        mem::replace(&mut node.neighbors, neighbors);
        self.max = self.max_history.pop().unwrap();
        if inits == 0 {
            node.state = State::Sleeping;
        } else {
            node.state = State::InMemory(inits);
            self.current += 1;
        }
        Ok(())
    }

    pub fn shortest(&mut self) -> usize {
        let len = self.nodes.len();
        let mut shortest = len; // worst case
        for s in (0..len).permutations(len) {
            let mut copy = self.clone();
            for bit in s {
                copy.measure(bit);
            }
            if copy.max < shortest {
                shortest = copy.max
            }
        }
        shortest
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;

    #[test]
    fn foo() {
        let mut graph = Graph::new(3, &[(0, 1), (1, 2)]);
        let mut copy = graph.clone();
        copy.measure(0);
        copy.measure(1);
        copy.measure(2);
        assert_eq!(copy.max, 2);
        let mut copy = graph.clone();
        copy.measure(1);
        copy.measure(0);
        copy.measure(2);
        assert_eq!(copy.max, 3);
        graph.measure(0);
        assert_eq!(graph.current, 1);
        assert_eq!(graph.max, 2);
        graph.measure(1);
        assert_eq!(graph.current, 1);
        assert_eq!(graph.max, 2);
        graph.measure(2);
        assert_eq!(graph.current, 0);
        assert_eq!(graph.max, 2);
        graph.reverse_measure(2);
        assert_eq!(graph.current, 1);
        assert_eq!(graph.max, 2);
        graph.reverse_measure(1);
        assert_eq!(graph.current, 1);
        assert_eq!(graph.max, 2);
        graph.reverse_measure(0);
        assert_eq!(graph.current, 0);
        assert_eq!(graph.max, 0);
    }

    #[test]
    fn bir() {
        assert_eq!(2, Graph::new(3, &[(0, 1), (1, 2)]).shortest());
        assert_eq!(3, Graph::new(3, &[(0, 1), (1, 2), (2, 0)]).shortest());
        assert_eq!(
            3,
            Graph::new(5, &[(0, 1), (1, 2), (2, 0), (0, 3), (0, 4)]).shortest()
        );
        // assert_eq!(
        //     3,
        //     Graph::new(
        //         10,
        //         &[
        //             // (10, 9),
        //             (9, 8),
        //             (8, 7),
        //             (7, 6),
        //             (6, 5),
        //             (5, 0),
        //             (0, 1),
        //             (1, 2),
        //             (2, 0),
        //             (0, 3),
        //             (0, 4)
        //         ]
        //     )
        //     .shortest()
        // );
    }
}
