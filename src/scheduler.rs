/*!
... tools to analyse the tracking results ...
*/

pub(crate) mod combinatoric;

use std::fmt::Display;

use combinatoric::Partition;

use self::{
    space::{
        AlreadyMeasured,
        Graph,
    },
    time::{
        Init,
        NotMeasurable,
        PathGenerator,
    },
    tree::{
        Focus,
        FocusIterator,
        Sweep,
    },
};

macro_rules! update {
    ($bit:expr, $map:expr) => {
        $map.get($bit).unwrap_or($bit)
    };
    ($bit:expr; $map:expr) => {
        *$bit = *update!($bit, $map);
    };
}
pub mod space;
pub mod time;
pub mod tree;

#[derive(Debug, Clone)]
pub struct Scheduler<'l, T> {
    time: PathGenerator<'l, T>,
    space: Graph<'l>,
}

impl<'l, T> Scheduler<'l, T> {
    pub fn new(time: PathGenerator<'l, T>, space: Graph<'l>) -> Self {
        Self { time, space }
    }

    pub fn time(&self) -> &PathGenerator<'l, T> {
        &self.time
    }

    pub fn space(&self) -> &Graph {
        &self.space
    }
}

// just for seeing whether it works as expected while developing
pub(crate) static mut COUNT: usize = 0;

impl<T: Init<usize>> Focus<&[usize]> for Scheduler<'_, T> {
    type Error = InstructionError;

    fn focus_inplace(&mut self, measure_set: &[usize]) -> Result<(), Self::Error> {
        self.time.focus_inplace(measure_set)?;
        #[cfg(debug_assertions)]
        self.space.focus_inplace(measure_set)?;
        #[cfg(not(debug_assertions))]
        self.space.focus_inplace_unchecked(measure_set);
        Ok(())
    }

    fn focus(&mut self, measure_set: &[usize]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let new_time = self.time.focus(measure_set)?;
        #[cfg(debug_assertions)]
        let new_space = self.space.focus(measure_set)?;
        #[cfg(not(debug_assertions))]
        let new_space = self.space.focus_unchecked(measure_set);
        Ok(Self { time: new_time, space: new_space })
    }
}

impl FocusIterator for Scheduler<'_, Partition<Vec<usize>>> {
    type IterItem = Vec<usize>;
    type LeafItem = usize;

    fn next_and_focus(&mut self) -> Option<(Self, Self::IterItem)>
    where
        Self: Sized,
    {
        let (new_time, mess) = self.time.next_and_focus()?;
        unsafe { COUNT += 1 };
        #[cfg(debug_assertions)]
        let new_space = self.space.focus(&mess).unwrap();
        #[cfg(not(debug_assertions))]
        let new_space = self.space.focus_unchecked(&mess);
        Some((Self { time: new_time, space: new_space }, mess))
    }

    fn at_leaf(&self) -> Option<Self::LeafItem> {
        self.time
            .measureable()
            .set()
            .is_empty()
            .then_some(self.space.max_memory())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InstructionError {
    TimeOrderingViolation(NotMeasurable),
    AlreadyMeasured(AlreadyMeasured),
}
impl Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionError::TimeOrderingViolation(e) => {
                write!(f, "time ordering violation: {}", e)
            }
            InstructionError::AlreadyMeasured(e) => {
                write!(f, "bit already measured: {}", e)
            }
        }
    }
}
impl std::error::Error for InstructionError {}
impl From<NotMeasurable> for InstructionError {
    fn from(error: NotMeasurable) -> Self {
        Self::TimeOrderingViolation(error)
    }
}
impl From<AlreadyMeasured> for InstructionError {
    fn from(error: AlreadyMeasured) -> Self {
        Self::AlreadyMeasured(error)
    }
}
#[doc = non_semantic_default!()]
impl Default for InstructionError {
    fn default() -> Self {
        Self::TimeOrderingViolation(NotMeasurable::default())
    }
}

impl<'l> IntoIterator for Scheduler<'l, Partition<Vec<usize>>> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = Sweep<Self>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self, Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use coverage_helper::test;

    use super::{
        time::LookupBuffer,
        tree::Step,
        *,
    };

    #[test]
    fn simple_paths() {
        //     1
        //   /  \
        // 0     3
        //   \  /
        //     2
        //
        // 0 --- 3 --- 2
        //  \
        //    -- 1
        let graph_buffer = space::tests::example_graph();
        let ordering = time::tests::example_ordering();
        let num = 4;
        let max = 4;

        let mut lookup_buffer = LookupBuffer::new(num);
        let graph = Graph::from_graph_buffer(&graph_buffer);
        let scheduler = Scheduler::new(
            PathGenerator::from_dependency_graph(ordering, &mut lookup_buffer, None),
            graph,
        );

        let mut results = Vec::new();
        let mut path = Vec::new();

        for step in scheduler.clone() {
            match step {
                Step::Forward(set) => path.push(set),
                Step::Backward(leaf) => {
                    if let Some(mem) = leaf {
                        results.push((path.len(), mem, path.clone()));
                    }
                    path.pop();
                }
            }
        }

        assert_eq!(
            results,
            vec![
                (4, 3, vec![vec![0], vec![3], vec![1], vec![2]]),
                (4, 3, vec![vec![0], vec![3], vec![2], vec![1]]),
                (3, 3, vec![vec![0], vec![3], vec![1, 2]]),
                (4, 3, vec![vec![0], vec![1], vec![3], vec![2]]),
                (3, 3, vec![vec![0], vec![3, 1], vec![2]]),
            ]
        );

        let mut optimal_paths: HashMap<usize, (usize, Vec<Vec<usize>>)> =
            HashMap::new();
        for (len, mem, path) in results {
            if let Some(optimal) = optimal_paths.get_mut(&len) {
                if optimal.0 > mem {
                    *optimal = (mem, path);
                }
            } else {
                optimal_paths.insert(len, (mem, path));
            }
        }

        assert_eq!(
            optimal_paths,
            HashMap::from_iter(vec![
                (4, (3, vec![vec![0], vec![3], vec![1], vec![2]])),
                (3, (3, vec![vec![0], vec![3], vec![1, 2]])),
            ])
        );

        let mut results = Vec::new();
        let mut path = Vec::new();
        let mut predicates = vec![max + 1; num + 1];
        let mut scheduler = scheduler.into_iter();

        while let Some(step) = scheduler.next() {
            match step {
                Step::Forward(mess) => {
                    let current = scheduler.current();
                    let minimum_time = if current.time.at_leaf().is_some() {
                        path.len() + 1 // plus the current step
                    } else if current.time.has_unmeasureable() {
                        // current step; at least one more because there are some
                        // measureable bits left; at least one more because there
                        // are some unmeasureable bits left that cannot be measured
                        // in the next step
                        path.len() + 3
                    } else {
                        path.len() + 2 // ...
                    };
                    if current.space.max_memory() >= predicates[minimum_time] {
                        if scheduler.skip_focus().is_err() {
                            break;
                        }
                    } else {
                        path.push(mess);
                    }
                }
                Step::Backward(leaf) => {
                    if let Some(mem) = leaf {
                        predicates[path.len()] = mem;
                        results.push((path.len(), mem, path.clone()));
                    }
                    path.pop();
                    // no sense in skipping here, because if it we could skip, we would
                    // have done it already in the forward step that led to this focused
                    // state
                }
            }
        }

        assert_eq!(
            HashMap::from_iter(
                results.into_iter().map(|(len, mem, path)| (len, (mem, path)))
            ),
            optimal_paths
        );
    }
}
