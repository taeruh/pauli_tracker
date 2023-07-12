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

use super::{
    time::Step,
    tree::Focus,
};

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
    space: Nodes,
    current_memory: usize,
    max_memory: usize,
}

impl Graph {
    pub fn new(
        num_bits: usize,
        edges: &[(usize, usize)],
        bit_mapping: Option<&HashMap<usize, usize>>,
    ) -> Self {
        let mut nodes = vec![Node::default(); num_bits];

        if let Some(bit_mapping) = bit_mapping {
            run(
                edges.iter().map(|(left, right)| {
                    (*update!(left, bit_mapping), *update!(right, bit_mapping))
                }),
                &mut nodes,
            )
        } else {
            run(edges.iter().copied(), &mut nodes);
        };
        fn run<T: Iterator<Item = (usize, usize)>>(edges: T, nodes: &mut Nodes) {
            for (left, right) in edges {
                if left == right {
                    continue;
                }
                nodes[left].1.push(right);
                nodes[right].1.push(left);
            }
        }

        Self {
            space: nodes,
            current_memory: 0,
            max_memory: 0,
        }
    }

    pub fn update(&mut self, bits: &[usize]) -> Result<(), AlreadyMeasured> {
        for bit in bits {
            self.measure(*bit)?;
        }
        if self.current_memory > self.max_memory {
            self.max_memory = self.current_memory;
        }
        self.current_memory -= bits.len();
        Ok(())
    }

    pub fn max_memory(&self) -> usize {
        self.max_memory
    }

    fn initialize(&mut self, bit: usize) -> Result<(), AlreadyMeasured> {
        match &mut self.space[bit].0 {
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
        let neighbors = mem::take(&mut self.space[bit].1);
        for neighbor in neighbors.iter() {
            let _ = self.initialize(*neighbor); // Err is okay
        }
        let node = &mut self.space[bit];
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
        new.update(instruction)?;
        Ok(new)
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

#[allow(unused)]
impl Graph {
    fn into_instructed_iterator<T: IntoIterator<Item = Step>>(
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

struct Sweep<T> {
    current: Graph,
    stack: Vec<Graph>,
    instructions: T,
}

enum Next {
    Mess(Vec<usize>),
    Mem(Option<usize>),
}

impl<T: Iterator<Item = Step>> Iterator for Sweep<T> {
    type Item = Next;
    fn next(&mut self) -> Option<Self::Item> {
        match self.instructions.next()? {
            Step::Forward(mess) => {
                let new = self.current.focus(&mess).unwrap();
                self.stack.push(mem::replace(&mut self.current, new));
                Some(Next::Mess(mess))
            }
            Step::Backward(at_leaf) => {
                let res = at_leaf.and(Some(self.current.max_memory));
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
    use crate::analyse::schedule::time::PathGenerator;

    #[test]
    fn tada() {
        let time = vec![
            vec![(0, vec![]), (2, vec![])],
            vec![(3, vec![0]), (1, vec![0, 2])],
            vec![(4, vec![0, 3])],
        ];
        let mut look = super::super::time::LookupBuffer::new(5);
        let time_graph = PathGenerator::from_dependency_graph(time, &mut look, None);
        let space = Graph::new(5, &[(0, 1), (1, 2), (1, 3), (2, 4), (4, 3)], None);

        // let mut path = Vec::new();
        // let mut results = Vec::new();

        // let splitted = super::super::time::split_instructions(graph.clone(), 5);
        let splitted = super::super::time::split_instructions(time_graph, 5);

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
