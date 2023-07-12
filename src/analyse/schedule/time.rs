/*!
...
*/

use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    mem,
};

use super::tree::{
    self,
    Focus,
    FocusIterator,
};
use crate::analyse::{
    combinatoric::Partition,
    DependencyGraph,
};

type Deps = HashMap<usize, Vec<usize>>;
type Look = Vec<Vec<usize>>;

pub struct LookupBuffer {
    look: Look,
}
impl LookupBuffer {
    pub fn new(num_bits: usize) -> Self {
        Self { look: vec![Vec::new(); num_bits] }
    }
}

#[derive(Debug, Clone)]
pub struct PathGenerator<'l> {
    // one could also put the dependents with the bit into the partition set and in deps
    // have vaules of the form (dependents, dependencies), however, the Partition clones
    // the set multiple times, therefore we don't want the dependents in there (also it
    // makes the from(DependencyGraph) function and the step function simpler if it is
    // separated)
    known: Partition<Vec<usize>>,
    deps: Deps,
    // it would have been slighty more ergnomic to use use an Rc instead of a reference
    // (no need to keep the actual map in an extra variable), however, this would have
    // been slyghtly less performant (Rc has an overhead every time it is cloned or
    // dropped, which happens rather often when sweeping); alternatively one could own
    // the map and always clone it, this would have the benefit that we could remove
    // elements from the map in the step function, however, since we are using a
    // HashMap, this does not really change the lookup time, rather the removing might
    // cause a slight overhead, and also we have the additional time and space overhead
    // when cloning it
    look: &'l Look,
}

impl<'l> PathGenerator<'l> {
    pub fn from_dependency_graph(
        mut graph: DependencyGraph,
        look: &'l mut LookupBuffer,
        bit_mapping: Option<&HashMap<usize, usize>>,
    ) -> Self {
        let look = &mut look.look;

        if graph.is_empty() {
            return Self {
                known: Partition::default(),
                deps: Deps::default(),
                look,
            };
        }

        if let Some(bit_mapping) = bit_mapping {
            for layer in graph.iter_mut() {
                for (bit, deps) in layer {
                    update!(bit; bit_mapping);
                    for dep in deps.iter_mut() {
                        update!(dep; bit_mapping);
                    }
                }
            }
        }

        fn resolve(bit: usize, rest: &[Vec<(usize, Vec<usize>)>], look: &mut Look) {
            let mut dependents = Vec::new();
            for layer in rest {
                for (dep, deps) in layer {
                    if deps.contains(&bit) {
                        dependents.push(*dep);
                    }
                }
            }
            look[bit] = dependents;
        }

        let mut known = Vec::new();
        let mut deps = HashMap::new();

        let mut graph_iter = graph.into_iter();

        let first = graph_iter.next().unwrap();
        let rest = graph_iter.as_ref();
        for (bit, _) in first {
            resolve(bit, rest, look);
            known.push(bit);
        }

        while let Some(layer) = graph_iter.next() {
            let rest = graph_iter.as_ref();
            for (bit, dependency) in layer {
                resolve(bit, rest, look);
                deps.insert(bit, dependency);
            }
        }

        let known = Self::new_partition(known);
        Self { known, deps, look }
    }

    pub fn finished_path(&self) -> bool {
        self.known.set.is_empty()
    }

    fn new(first: Partition<Vec<usize>>, deps: Deps, look: &'l Look) -> Self {
        Self { known: first, deps, look }
    }

    fn new_partition(a: Vec<usize>) -> Partition<Vec<usize>> {
        let first_len = a.len();
        let mut first = Partition::new(a, first_len);
        first.next();
        first
    }

    fn focus_unchecked(
        &mut self,
        measuring: &[usize],
        mut first: Vec<usize>,
    ) -> Result<Self, TimeOrderingViolation> {
        let mut deps = self.deps.clone();
        for known in measuring.iter() {
            let dependents = &self.look[*known];
            for bit in dependents {
                let dependencies = match deps.get_mut(bit) {
                    Some(s) => s,
                    None => {
                        return Err(TimeOrderingViolation::MissingDependent(
                            *known, *bit,
                        ));
                    }
                };
                let pos = dependencies.iter().position(|e| e == known).expect(
                    "bug: the creation of self via from_dependency_graph guarantees \
                     that the known is in dependencies",
                );
                dependencies.swap_remove(pos);
                if dependencies.is_empty() {
                    deps.remove(bit)
                        .expect("bug: we checked it already above with get_mut");
                    first.push(*bit);
                }
            }
        }
        Ok(Self::new(Self::new_partition(first), deps, self.look))
    }
}

impl Focus<(&Vec<usize>, Vec<usize>)> for PathGenerator<'_> {
    type Error = TimeOrderingViolation;
    fn focus(
        &mut self,
        instruction: (&Vec<usize>, Vec<usize>),
    ) -> Result<Self, TimeOrderingViolation>
    where
        Self: Sized,
    {
        let (measuring, first) = instruction;
        for e in measuring.iter() {
            if first.contains(e) {
                return Err(TimeOrderingViolation::Overlap(*e));
            }
            if !self.known.set.contains(e) {
                return Err(TimeOrderingViolation::NotMeasureable(*e));
            }
        }
        self.focus_unchecked(measuring, first)
    }
}

impl FocusIterator for PathGenerator<'_> {
    type IterItem = Vec<usize>;
    type LeafItem = ();

    fn next_and_focus(&mut self) -> Option<(Self, Self::IterItem)>
    where
        Self: Sized,
    {
        let (measuring, new_measureable_set) = self.known.next()?;
        Some((
            // we know that the input is fine, because it comes from self.known.next()
            self.focus_unchecked(&measuring, new_measureable_set).expect(
                "bug: the only way to change self in the API is through this function \
                 here (the pointee behind self.look is already through the borrow \
                 checker system locked since we take a mut ref to when creating a \
                 TimeOrdering in from_dependency_graph, and additionally through the \
                 type system ), so we know that all should be fine",
            ),
            measuring,
        ))
    }

    fn at_leaf(&self) -> Option<Self::LeafItem> {
        self.known.set.is_empty().then_some(())
    }
}

#[derive(Debug, Clone)]
pub enum TimeOrderingViolation {
    MissingDependent(usize, usize),
    NotMeasureable(usize),
    Overlap(usize),
}
impl Display for TimeOrderingViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeOrderingViolation::MissingDependent(measure, dependent) => {
                write!(
                    f,
                    "missing dependent ({dependent}) for bit {measure} is missing; \
                     this likely happended because the bit {measure} is measured \
                     multiple times",
                )
            }
            TimeOrderingViolation::NotMeasureable(bit) => {
                write!(f, "the bit {bit} is not in the measureable set",)
            }
            TimeOrderingViolation::Overlap(bit) => write!(
                f,
                "the measured set and the measureable set contain a shared bit: {bit}",
            ),
        }
    }
}
impl Error for TimeOrderingViolation {}

tree::impl_into_iterator!(PathGenerator with <'l>);

pub(crate) type Step = tree::Step<Vec<usize>, Option<()>>;

#[allow(unused)]
pub(crate) fn split_instructions(
    time: PathGenerator,
    num_tasks: usize,
) -> Vec<Vec<Step>> {
    let mut total_num_paths = 0;
    let mut instructions = time
        .into_iter()
        .inspect(|e| {
            if let Step::Backward(Some(())) = e {
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
            ref step @ Step::Backward(at_leaf) => {
                task.push(step.clone());
                init.pop();
                if at_leaf.is_some() {
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
    fn leck() {
        let time = vec![
            vec![(0, vec![]), (2, vec![])],
            vec![(3, vec![0]), (1, vec![0, 2])],
            vec![(4, vec![0, 3])],
        ];
        let mut look = LookupBuffer::new(5);
        let mut graph = PathGenerator::from_dependency_graph(time, &mut look, None);

        let (m, f) = graph.known.next().unwrap();
        graph.focus((&m, f)).unwrap();
        let _moved = m;
        println!("{:?}", graph);
    }

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
        let mut look = LookupBuffer::new(5);
        let graph = PathGenerator::from_dependency_graph(time, &mut look, None);
        // println!("{:?}", graph);
        // return;

        let mut path = Vec::new();
        let mut results = Vec::new();
        let mut instructions = Vec::new();
        let mut total = 0;
        let mut effective = 0;

        let _splitted = split_instructions(graph.clone(), 5);

        for step in graph {
            instructions.push(step.clone());
            match step {
                Step::Forward(mess) => {
                    path.push(mess);
                    effective += 1;
                }
                Step::Backward(at_leaf) => {
                    if at_leaf.is_some() {
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

// currently, when looping through the partitions in next_and_focus, we do
// let (measuring, new_measureable_set) = self.known.next()?;
// if we want to swap the order, we have to do
// let (mut new_measureable_set, measuring) = self.first.next()?;
// if measuring.is_empty() {
//     return None;
// }
// and in new_partition we have to comment
// first.next();
