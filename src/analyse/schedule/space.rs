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

use super::time::Step;
use crate::analyse::DependencyGraph;
type SpaceGraph = Vec<Node>;

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

#[derive(Clone, PartialEq, Eq, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Graph {
    space: SpaceGraph,
    current_memory: usize,
    pub max_memory: usize,
}

type Deps = HashMap<usize, Vec<usize>>;

type Schedule = Vec<Vec<usize>>;

impl Graph {
    pub fn new(num: usize, edges: &[(usize, usize)]) -> Self {
        let mut nodes = vec![Node::default(); num];
        for &(left, right) in edges {
            // assert!(left != right, "no loops allowed");
            if left == right {
                continue;
            }
            nodes[left].neighbors.push(right);
            nodes[right].neighbors.push(left);
        }
        Self {
            space: nodes,
            current_memory: 0,
            max_memory: 0,
        }
    }

    fn initialize(&mut self, bit: usize) -> Result<(), ()> {
        match &mut self.space[bit].state {
            state @ State::Sleeping => {
                *state = State::InMemory;
                self.current_memory += 1;
            }
            State::InMemory => (),
            State::Measured => return Err(()),
        }
        Ok(())
    }

    fn measure(&mut self, bit: usize) -> Result<(), ()> {
        let neighbors = mem::take(&mut self.space[bit].neighbors);
        for neighbor in neighbors.iter() {
            let _ = self.initialize(*neighbor); // Err is okay
        }
        let node = &mut self.space[bit];
        mem::replace(&mut node.neighbors, neighbors);
        match node.state {
            State::Sleeping => {
                self.current_memory += 1;
                node.state = State::Measured;
            }
            State::InMemory => node.state = State::Measured,
            State::Measured => return Err(()),
        }
        Ok(())
    }

    pub fn step(&mut self, bits: &[usize]) -> Option<Self> {
        let mut new = self.clone();
        for bit in bits {
            // new.measure(*bit)?;
            new.measure(*bit).unwrap();
        }
        if new.current_memory > new.max_memory {
            new.max_memory = new.current_memory;
        }
        new.current_memory -= bits.len();
        Some(new)
    }

    pub fn into_instructed_iterator<T: IntoIterator<Item = Step>>(
        self,
        instructions: T,
    ) -> Sweep<T::IntoIter> {
        Sweep {
            current: self,
            stack: Vec::new(),
            instructions: instructions.into_iter(),
        }
    }
}

pub struct Sweep<T> {
    current: Graph,
    stack: Vec<Graph>,
    instructions: T,
}

pub enum Next {
    Mess(Vec<usize>),
    Mem(Option<usize>),
}

impl<T: Iterator<Item = Step>> Iterator for Sweep<T> {
    type Item = Next;
    fn next(&mut self) -> Option<Self::Item> {
        match self.instructions.next()? {
            Step::Forward(mess) => {
                let new = self.current.step(&mess).unwrap();
                self.stack.push(mem::replace(&mut self.current, new));
                Some(Next::Mess(mess))
            }
            Step::Backward(at_end) => {
                let res = at_end.then_some(self.current.max_memory);
                self.current = self.stack.pop().unwrap();
                Some(Next::Mem(res))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;
    use crate::analyse::schedule::time::TimeGraph;

    #[test]
    fn tada() {
        let time = vec![
            vec![(0, vec![]), (2, vec![])],
            vec![(3, vec![0]), (1, vec![0, 2])],
            vec![(4, vec![0, 3])],
        ];
        let graph = TimeGraph::from(time);
        let space = Graph::new(5, &[(0, 1), (1, 2), (1, 3), (2, 4), (4, 3)]);

        // let mut path = Vec::new();
        // let mut results = Vec::new();

        // let splitted = super::super::time::split_instructions(graph.clone(), 5);
        let splitted = super::super::time::split_instructions(graph, 5);

        // for step in graph {
        //     match step {
        //         Step::Forward(mess) => {
        //             path.push(mess);
        //         }
        //         Step::Backward(at_end) => {
        //             if at_end {
        //                 println!("{}; {:?}", path.len(), path);
        //                 results.push(path.clone());
        //             }
        //             path.pop();
        //         }
        //     }
        // }

        // println!();

        for split in splitted {
            let mut path = Vec::new();
            let mut results = Vec::new();
            for step in space.clone().into_instructed_iterator(split) {
                match step {
                    Next::Mess(mess) => {
                        path.push(mess);
                    }
                    Next::Mem(mem) => {
                        if let Some(mem) = mem {
                            println!("{}; {}; {:?}", path.len(), mem, path);
                            results.push(path.clone());
                        }
                        path.pop();
                    }
                }
            }
        }
    }

    // #[test]
    // fn bar() {
    //     let deps = Deps::from([(0, vec![]), (1, vec![]), (2, vec![0, 1])]);
    //     // (0..deps.len())
    //     //     .permutations(deps.len())
    //     //     .filter(|path| valid_path(path, &deps))
    //     //     .for_each(|path| println!("{:?}", path));
    //     assert_eq!(
    //         (2, vec![0, 1, 2]),
    //         Graph::new(3, &[(0, 1), (1, 2)]).min_bits(&deps)
    //     );
    //     let deps = Deps::from([(0, vec![1]), (1, vec![]), (2, vec![0, 1])]);
    //     // (0..deps.len())
    //     //     .permutations(deps.len())
    //     //     .filter(|path| valid_path(path, &deps))
    //     //     .for_each(|path| println!("{:?}", path));
    //     assert_eq!(
    //         (3, vec![1, 0, 2]),
    //         Graph::new(3, &[(0, 1), (1, 2)]).min_bits(&deps)
    //     );
    // }

    // #[test]
    // fn hey() {
    //     let deps = vec![vec![(0, vec![]), (1, vec![])], vec![(2, vec![0, 1])]];
    //     let mut graph = Graph::new(3, &[(0, 1), (1, 2)]);
    //     println!("{:?}", graph.mixed(&deps, 1, 4, true));
    //     println!("{:?}", graph.mixed(&deps, 1, 2, true));

    //     let deps = vec![
    //         vec![(0, vec![]), (1, vec![]), (6, vec![])],
    //         vec![(2, vec![3, 1]), (3, vec![0, 1])],
    //         vec![(4, vec![2, 1]), (5, vec![3, 1])],
    //     ];
    //     let mut graph = Graph::new(
    //         7,
    //         &[(0, 1), (1, 2), (2, 0), (2, 3), (1, 5), (4, 5), (2, 6), (0, 5)],
    //     );
    //     let (res, min) = graph.mixed(&deps, 1, 7, true);
    //     println!("{:?}\n", min);
    //     println!("3: {:?}\n", res.get(&3));
    //     println!("4: {:?}\n", res.get(&4));
    //     println!("5: {:?}\n", res.get(&5));
    //     println!("{:?}", graph.mixed(&deps, 1, 3, true));
    // }
}

#[allow(unused)]
pub(crate) mod maybe_better;

// pub fn mixed(
//     &mut self,
//     deps_graph: &DependencyGraph,
//     lower_bound: usize, // inclusive
//     upper_bound: usize, // exclusive
//     get_shortest: bool,
// ) -> (HashMap<usize, Vec<Schedule>>, Option<Vec<Schedule>>) {
//     fn schedule(s: Vec<usize>, deps_graph: &DependencyGraph) -> Schedule {
//         let mut path = Vec::new();
//         let mut s = s.into_iter();
//         let bit = s.next().unwrap();
//         // first bit has to be in the zeroth layer
//         let mut last_layer = 0;
//         let mut layer = 0;
//         path.push(vec![bit]);
//         for bit in s {
//             let layer = find_layer(bit, deps_graph).unwrap();
//             if layer <= last_layer {
//                 path[last_layer].push(bit);
//             } else {
//                 debug_assert_eq!(last_layer + 1, layer);
//                 path.push(vec![bit]);
//                 last_layer = layer;
//             }
//         }
//         path
//     }

//     fn compare(present: &mut Vec<Schedule>, path: Schedule) {
//         match present[0].len().cmp(&path.len()) {
//             std::cmp::Ordering::Less => (),
//             std::cmp::Ordering::Equal => present.push(path),
//             std::cmp::Ordering::Greater => *present = vec![path],
//         }
//     }

//     let deps = Deps::from_iter(deps_graph.clone().into_iter().flatten());
//     let len = self.nodes.len();
//     let mut max = len + 1; // worst case + 1
//     let mut min_bits_path: Option<Vec<Schedule>> = None;
//     let mut res: HashMap<usize, Vec<Schedule>> = HashMap::new();
//     for s in (0..len).permutations(len).filter(|path| valid_path(path, &deps)) {
//         let mut copy = self.clone();
//         for &bit in &s {
//             copy.measure_set(&[bit]);
//         }
//         if lower_bound <= copy.max && copy.max < upper_bound {
//             let path = schedule(s, deps_graph);
//             match res.get_mut(&copy.max) {
//                 Some(p) => compare(p, path),
//                 None => {
//                     let _ = res.insert(copy.max, vec![path]);
//                 }
//             };
//         } else if get_shortest && copy.max <= max {
//             max = copy.max;
//             let path = schedule(s, deps_graph);
//             match min_bits_path {
//                 Some(ref mut p) => compare(p, path),
//                 None => min_bits_path = Some(vec![path]),
//             }
//         }
//     }
//     (res, min_bits_path)
// }

// fn find_layer(bit: usize, deps: &DependencyGraph) -> Option<usize> {
//     for (i, layer) in deps.iter().enumerate() {
//         if layer.iter().map(|(i, _)| i).contains(&bit) {
//             return Some(i);
//         }
//     }
//     None
// }
// fn valid_path(path: &[usize], deps: &Deps) -> bool {
//     let mut measured = Vec::with_capacity(path.len());
//     for node in path {
//         for dep in deps.get(node).unwrap() {
//             if !measured.contains(dep) {
//                 return false;
//             }
//         }
//         measured.push(*node);
//     }
//     true
// }

// pub fn min_bits(&mut self, deps: &Deps) -> (usize, Vec<usize>) {
//     let len = self.nodes.len();
//     let mut max = len + 1; // worst case + 1
//     let mut path = (0..len).collect();
//     for s in (0..len).permutations(len).filter(|path| valid_path(path, deps)) {
//         let mut copy = self.clone();
//         for &bit in &s {
//             copy.measure_set(&[bit]);
//         }
//         if copy.max < max {
//             max = copy.max;
//             path = s;
//         }
//     }
//     (max, path)
// }
