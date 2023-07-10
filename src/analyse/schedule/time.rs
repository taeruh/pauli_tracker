/*!
...
*/

use std::{
    collections::HashMap,
    iter::Enumerate,
    mem,
    slice::Iter,
};

use super::sweep::{
    self,
    FocusNext,
    Sweep,
};
use crate::analyse::{
    combinatoric::Partition,
    DependencyGraph,
};

type Map = HashMap<usize, Vec<usize>>;

#[derive(Debug, Clone, Default)]
pub struct TimeGraph {
    // one could also put the dependents with the bit into the partition set and in deps
    // have vaules of the form (dependents, dependencies), however, the Partition clones
    // the set multiple times, therefore we don't want the dependents in there (also it
    // makes the from(DependencyGraph) function and the step function simpler if it is
    // separated)
    pub known: Partition<Vec<usize>>,
    pub deps: Map,
    look: Map,
}

impl From<DependencyGraph> for TimeGraph {
    fn from(mut graph: DependencyGraph) -> Self {
        if graph.is_empty() {
            return Self::default();
        }

        fn resolve(bit: usize, rest: &[Vec<(usize, Vec<usize>)>], look: &mut Map) {
            let mut dependents = Vec::new();
            for layer in rest {
                for (dep, deps) in layer {
                    if let Some(position) = deps.iter().position(|b| *b == bit) {
                        dependents.push(*dep);
                    }
                }
            }
            look.insert(bit, dependents);
        }

        let mut known = Vec::new();
        let mut look = HashMap::new();
        let mut deps = HashMap::new();

        let mut graph_iter = graph.into_iter();

        let first = graph_iter.next().unwrap();
        let rest = graph_iter.as_ref();
        for (bit, _) in first {
            resolve(bit, rest, &mut look);
            known.push(bit);
        }

        while let Some(layer) = graph_iter.next() {
            let rest = graph_iter.as_ref();
            for (bit, dependency) in layer {
                resolve(bit, rest, &mut look);
                deps.insert(bit, dependency);
            }
        }

        let known = Self::new_partition(known);
        Self { known, deps, look }
    }
}

impl TimeGraph {
    fn new(first: Partition<Vec<usize>>, deps: Map, look: Map) -> Self {
        Self { known: first, deps, look }
    }

    fn new_partition(a: Vec<usize>) -> Partition<Vec<usize>> {
        let first_len = a.len();
        let mut first = Partition::new(a, first_len);
        first.next(); // comments this when switching the order
        first
    }
}

impl FocusNext for TimeGraph {
    type Outcome = Vec<usize>;
    type EndOutcome = bool;

    fn step(&mut self) -> Option<(Self, Self::Outcome)>
    where
        Self: Sized,
    {
        // switch comments here to switch order
        let (measuring, mut first) = self.known.next()?;
        // let (mut first, measuring) = self.first.next()?;
        // if measuring.is_empty() {
        //     return None;
        // }
        let mut deps = self.deps.clone();
        let mut look = self.look.clone();
        for known in measuring.iter() {
            let dependents = look.remove(known).unwrap();
            for bit in dependents {
                let dependencies = deps.get_mut(&bit).unwrap();
                let pos = dependencies.iter().position(|e| e == known).unwrap();
                dependencies.swap_remove(pos);
                if dependencies.is_empty() {
                    deps.remove(&bit).unwrap();
                    first.push(bit);
                }
            }
        }
        Some((Self::new(Self::new_partition(first), deps, look), measuring))
    }

    fn check_end(&self) -> Self::EndOutcome {
        self.known.set.is_empty()
    }
}

sweep::impl_into_iterator!(TimeGraph);

pub type Step = sweep::Step<Vec<usize>, bool>;

pub fn split_instructions(time: TimeGraph, num_tasks: usize) -> Vec<Vec<Step>> {
    let mut total_num_paths = 0;
    let mut instructions = time
        .into_iter()
        .inspect(|e| {
            if let Step::Backward(true) = e {
                total_num_paths += 1;
            };
        })
        .collect::<Vec<Step>>()
        .into_iter();
    let mut paths_per_job = total_num_paths / num_tasks;
    let num_normal_tasks = num_tasks - (total_num_paths - num_tasks * paths_per_job);

    let mut res = Vec::new();
    let mut task = Vec::new();
    let mut paths_in_task = 0;
    let mut init = Vec::new();
    let mut num_done_tasks = 0;

    while let Some(step) = instructions.next() {
        match step {
            ref step @ Step::Backward(at_end) => {
                task.push(step.clone());
                init.pop();
                if at_end {
                    paths_in_task += 1;
                    if paths_in_task == paths_per_job {
                        paths_in_task = 0;
                        for step in instructions.by_ref() {
                            match step {
                                Step::Backward(_) => {
                                    init.pop();
                                }
                                step => {
                                    init.push(step);
                                    break;
                                }
                            }
                        }
                        res.push(mem::replace(&mut task, init.clone()));
                        num_done_tasks += 1;
                        if num_done_tasks == num_normal_tasks {
                            paths_per_job += 1;
                        }
                    }
                }
            }
            step => {
                task.push(step.clone());
                init.push(step);
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;

    #[test]
    fn invert_graph() {
        let time = vec![
            vec![(0, vec![]), (2, vec![])],
            vec![(3, vec![0]), (1, vec![0, 2])],
            vec![(4, vec![0, 3])],
        ];
        // let time = vec![vec![
        //     (0, vec![]),
        //     (1, vec![]),
        //     (2, vec![]),
        //     (3, vec![]),
        //     (4, vec![]),
        //     (5, vec![]),
        //     (6, vec![]),
        //     // (7, vec![]),
        //     // (8, vec![]),
        // ]];
        let graph = TimeGraph::from(time);
        println!("{:?}", graph);
        return;

        let mut path = Vec::new();
        let mut results = Vec::new();
        let mut instructions = Vec::new();
        let mut total = 0;
        let mut effective = 0;

        let splitted = split_instructions(graph.clone(), 5);

        for step in graph {
            instructions.push(step.clone());
            match step {
                Step::Forward(mess) => {
                    path.push(mess);
                    effective += 1;
                }
                Step::Backward(at_end) => {
                    if at_end {
                        total += path.len();
                        println!("{}; {:?}", path.len(), path);
                        results.push(path.clone());
                    }
                    path.pop();
                }
            }
        }
        println!("total: {:?}", total);
        println!("effec: {:?}", effective);
        println!("resul: {:?}", results.len());
        println!("instr: {:?}", instructions.len());
        // println!("{:?}", instructions);
    }
}
