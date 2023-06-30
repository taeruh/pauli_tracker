/*!
...
*/

use std::{
    collections::HashMap,
    mem,
    slice,
};

use itertools::Itertools;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use crate::analyse::DependencyGraph;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum State {
    #[default]
    Sleeping,
    InMemory,
    Measured,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Node {
    state: State,
    neighbors: Vec<usize>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Graph {
    nodes: Vec<Node>,
    current: usize,
    max: usize,
}

type Deps = HashMap<usize, Vec<usize>>;

impl Graph {
    pub fn new(num: usize, edges: &[(usize, usize)]) -> Self {
        let mut nodes = vec![Node::default(); num];
        for &(left, right) in edges {
            assert!(left != right, "no loops allowed");
            nodes[left].neighbors.push(right);
            nodes[right].neighbors.push(left);
        }
        Self { nodes, current: 0, max: 0 }
    }

    fn initialize(&mut self, bit: usize) -> Result<(), ()> {
        match &mut self.nodes[bit].state {
            state @ State::Sleeping => {
                *state = State::InMemory;
                self.current += 1;
            }
            State::InMemory => (),
            State::Measured => return Err(()),
        }
        Ok(())
    }

    pub fn measure(&mut self, bit: usize) -> Result<(), ()> {
        // take and replace aftwards, instead of cloning (I'm not sure whether there's an
        // valid way to get around the aliasing rules; one could do something like
        // std::slice::from_raw(...).iter() and Miri wouldn't error, but I think this is
        // not valid; of course we could get around it with an UnsafeCell to opt-out of
        // the strict aliasing rules or use raw pointers in initialize); okay, the
        // following works, but then we have index self.nodes in every round
        // for i in 0..self.nodes[bit].neighbors.len() {
        //     let _ = self.initialize(self.nodes[bit].neighbors[i]);
        // }
        // let node = &mut self.nodes[bit];
        let neighbors = mem::take(&mut self.nodes[bit].neighbors);
        for neighbor in neighbors.iter() {
            let _ = self.initialize(*neighbor);
        }
        let node = &mut self.nodes[bit];
        mem::replace(&mut node.neighbors, neighbors);
        match node.state {
            State::Sleeping => {
                self.current += 1;
                node.state = State::Measured;
            }
            State::InMemory => node.state = State::Measured,
            State::Measured => return Err(()),
        }
        if self.current > self.max {
            self.max = self.current;
        }
        self.current -= 1;
        Ok(())
    }

    pub fn min_bits(&mut self, deps: &Deps) -> (usize, Vec<usize>) {
        let len = self.nodes.len();
        let mut max = len + 1; // worst case + 1
        let mut path = (0..len).collect();
        for s in (0..len).permutations(len).filter(|path| valid_path(path, deps)) {
            let mut copy = self.clone();
            for &bit in &s {
                copy.measure(bit);
            }
            if copy.max < max {
                max = copy.max;
                path = s;
            }
        }
        (max, path)
    }

    pub fn mixed(
        &mut self,
        deps_graph: &DependencyGraph,
        lower_bound: usize, // inclusive
        upper_bound: usize, // exclusive
        get_shortest: bool,
    ) -> (HashMap<usize, Vec<Vec<Vec<usize>>>>, Option<Vec<Vec<Vec<usize>>>>) {
        fn schedule(s: Vec<usize>, deps_graph: &DependencyGraph) -> Vec<Vec<usize>> {
            let mut path = Vec::new();
            let mut s = s.into_iter();
            let bit = s.next().unwrap();
            // first bit has to be in the zeroth layer
            let mut last_layer = 0;
            let mut layer = 0;
            path.push(vec![bit]);
            for bit in s {
                let layer = find_layer(bit, deps_graph).unwrap();
                if layer <= last_layer {
                    path[last_layer].push(bit);
                } else {
                    debug_assert_eq!(last_layer + 1, layer);
                    path.push(vec![bit]);
                    last_layer = layer;
                }
            }
            path
        }

        fn compare(present: &mut Vec<Vec<Vec<usize>>>, path: Vec<Vec<usize>>) {
            match present[0].len().cmp(&path.len()) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => present.push(path),
                std::cmp::Ordering::Greater => *present = vec![path],
            }
        }

        let deps = Deps::from_iter(deps_graph.clone().into_iter().flatten());
        let len = self.nodes.len();
        let mut max = len + 1; // worst case + 1
        let mut min_bits_path: Option<Vec<Vec<Vec<usize>>>> = None;
        let mut res: HashMap<usize, Vec<Vec<Vec<usize>>>> = HashMap::new();
        for s in (0..len).permutations(len).filter(|path| valid_path(path, &deps)) {
            let mut copy = self.clone();
            for &bit in &s {
                copy.measure(bit);
            }
            if lower_bound <= copy.max && copy.max < upper_bound {
                let path = schedule(s, deps_graph);
                match res.get_mut(&copy.max) {
                    Some(p) => compare(p, path),
                    None => {
                        let _ = res.insert(copy.max, vec![path]);
                    }
                };
            } else if get_shortest && copy.max <= max {
                max = copy.max;
                let path = schedule(s, deps_graph);
                match min_bits_path {
                    Some(ref mut p) => compare(p, path),
                    None => min_bits_path = Some(vec![path]),
                }
            }
        }
        (res, min_bits_path)
    }
}

fn find_layer(bit: usize, deps: &DependencyGraph) -> Option<usize> {
    for (i, layer) in deps.iter().enumerate() {
        if layer.iter().map(|(i, _)| i).contains(&bit) {
            return Some(i);
        }
    }
    None
}

fn valid_path(path: &[usize], deps: &Deps) -> bool {
    let mut measured = Vec::with_capacity(path.len());
    for node in path {
        for dep in deps.get(node).unwrap() {
            if !measured.contains(dep) {
                return false;
            }
        }
        measured.push(*node);
    }
    true
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;

    #[test]
    fn bar() {
        let deps = Deps::from([(0, vec![]), (1, vec![]), (2, vec![0, 1])]);
        // (0..deps.len())
        //     .permutations(deps.len())
        //     .filter(|path| valid_path(path, &deps))
        //     .for_each(|path| println!("{:?}", path));
        assert_eq!(
            (2, vec![0, 1, 2]),
            Graph::new(3, &[(0, 1), (1, 2)]).min_bits(&deps)
        );
        let deps = Deps::from([(0, vec![1]), (1, vec![]), (2, vec![0, 1])]);
        // (0..deps.len())
        //     .permutations(deps.len())
        //     .filter(|path| valid_path(path, &deps))
        //     .for_each(|path| println!("{:?}", path));
        assert_eq!(
            (3, vec![1, 0, 2]),
            Graph::new(3, &[(0, 1), (1, 2)]).min_bits(&deps)
        );
    }

    #[test]
    fn hey() {
        let deps = vec![vec![(0, vec![]), (1, vec![])], vec![(2, vec![0, 1])]];
        let mut graph = Graph::new(3, &[(0, 1), (1, 2)]);
        println!("{:?}", graph.mixed(&deps, 1, 4, true));
        println!("{:?}", graph.mixed(&deps, 1, 2, true));

        let deps = vec![
            vec![(0, vec![]), (1, vec![]), (6, vec![])],
            vec![(2, vec![3, 1]), (3, vec![0, 1])],
            vec![(4, vec![2, 1]), (5, vec![3, 1])],
        ];
        let mut graph = Graph::new(
            7,
            &[(0, 1), (1, 2), (2, 0), (2, 3), (1, 5), (4, 5), (2, 6), (0, 5)],
        );
        let (res, min) = graph.mixed(&deps, 1, 7, true);
        println!("{:?}\n", min);
        println!("3: {:?}\n", res.get(&3));
        println!("4: {:?}\n", res.get(&4));
        println!("5: {:?}\n", res.get(&5));
        println!("{:?}", graph.mixed(&deps, 1, 3, true));
    }
}

#[allow(unused)]
pub(crate) mod maybe_better;
