/*!
Analyse scheduling paths on a [graph state] (or similar) allowed by a [DependencyGraph].

**This module is currently rather experimental.**

*The module is rather independent of of Pauli tracking. In general, one just needs a
time ordering on the qubits, like a [DependencyGraph] (or have no time ordering at
all). Also, it is not that general and rather specific to certain tasks, but if one
needs more flexibility, it might be still useful as a template.*

Realizing [graph state]s on a quantum computer can be done sequentially (cf. [space]),
however, this is often restricted by a time ordering induced by non-determinism (cf.
[time]).

This module provides a [Scheduler] that combines [PathGenerator] and [Graph]. It can be
used to analyze allowed scheduling paths - the process of initializing and measuring
qubits - regarding the required quantum memory and the number of required measurement
steps.

# Performance

There are lots of possible scheduling paths, in the worst case the number of possible
paths is given by the
[ordered Bell number](https://en.wikipedia.org/wiki/Ordered_Bell_number). Therefore,
if the task is to find the optimal path, under some metric, doing this by checking all
possible paths might scale very, very badly. The [Sweep] iterator in [tree] tries to
reduce that scaling by keeping track of its states to reduce the number of redundant
calculations (this comes at the cost of memory, but this cost is scaling linearly), and
a [skipping method](Sweep::skip_current) to skip states that are known to be not
interesting. However, the scaling can still be very, very bad.

Saying that, if you have a high number of qubits, you might not want to use this module,
but it might still be useful for testing purposes.

It is a goal to find better algorithms, but this will probably happen in a separate
project.

# Examples

### Capturing all paths
```
# #[cfg_attr(coverage_nightly, no_coverage)]
# #[cfg(feature = "scheduler")]
# fn main() {
# #[rustfmt::skip]
use pauli_tracker::tracker::frames::dependency_graph::DependencyGraph;
# #[rustfmt::skip]
use pauli_tracker::scheduler::{
    Scheduler,
    space::{Graph, GraphBuffer},
    time::{PathGenerator, DependencyBuffer, Partitioner},
    tree::Step,
};

// consider the following spacial graph state
//     1
//   /  \
// 0     3
//   \  /
//     2
// and this time ordering defined a DependencyGraph
// 0 --> 3 --> 2
//  \
//    -> 1
let graph_state_edges = [(0, 1), (0, 2), (1, 3), (3, 2)];
let time_ordering: DependencyGraph = vec![
    vec![(0, vec![])],
    vec![(3, vec![0]), (1, vec![0])],
    vec![(2, vec![3])]
];
let num_bits = 4;

// we want to find all paths, their lenghts, and the required quantum memory

// lets create a path generator, which generates all allowed paths, and a spacial graph
// that captures the required quantum memory:

// to reduce cloning overhead, some parts are owned by separate buffers
let graph_buffer = GraphBuffer::new(&graph_state_edges, num_bits, None, false);
let mut dependency_buffer = DependencyBuffer::new(num_bits);

let graph = Graph::new(&graph_buffer);
let path_generator: PathGenerator::<Partitioner> =
    PathGenerator::from_dependency_graph(time_ordering, &mut dependency_buffer, None);
// the Partitioner above as generic parameter will allow us to iterate over all paths;
// if we would want to create the paths manually, we could use Vec<usize> instead

// we could now generate paths with `path_generator` and feed them into `graph`; we'll do
// this in a later example, here, we wrap them into a Scheduler which basically does
// exactly that for us
let scheduler = Scheduler::new(path_generator, graph);

// by iterating over `scheduler` we can generate all results

let mut results = Vec::new();
let mut current_path = Vec::new();

// Compare the documentation of the tree module; we are traversing through a tree; a
// forward step means we measure a new set; a backward step means we go back to the
// previous state of in the tree; if we took a step back from a leaf, we now that we
// have reached the end of a full path, so we can extract the maximum required memory
for step in scheduler {
    match step {
        Step::Forward(measure_set) => current_path.push(measure_set),
        Step::Backward(leaf) => {
            if let Some(max_memory) = leaf {
                results.push((current_path.len(), max_memory, current_path.clone()));
            }
            current_path.pop();
        }
    }
}

assert_eq!(
    results,
    vec![// path len,  required memory,  path
        (       4,            3,         vec![vec![0], vec![3], vec![1], vec![2]]),
        (       4,            3,         vec![vec![0], vec![3], vec![2], vec![1]]),
        (       3,            3,         vec![vec![0], vec![3], vec![1, 2]]),
        (       4,            3,         vec![vec![0], vec![1], vec![3], vec![2]]),
        (       3,            3,         vec![vec![0], vec![3, 1], vec![2]]),
    ]
);
# }
# #[cfg_attr(coverage_nightly, no_coverage)]
# #[cfg(not(feature = "scheduler"))]
# fn main() {}
```

### Finding the optimal paths
```
# #[cfg_attr(coverage_nightly, no_coverage)]
# #[cfg(feature = "scheduler")]
# fn main() {
use std::collections::HashMap;
# #[rustfmt::skip]
use pauli_tracker::tracker::frames::dependency_graph::DependencyGraph;
# #[rustfmt::skip]
use pauli_tracker::scheduler::{
    Scheduler,
    space::{Graph, GraphBuffer},
    time::{PathGenerator, DependencyBuffer},
    tree::{Step, FocusIterator},
};

// we consider the same example as above
let graph_state_edges = [(0, 1), (0, 2), (1, 3), (3, 2)];
let time_ordering: DependencyGraph = vec![
    vec![(0, vec![])],
    vec![(3, vec![0]), (1, vec![0])],
    vec![(2, vec![3])]
];
let num_bits = 4;

// now we want to find the optimal paths in the sense that for a given path length
// we want to find the path that requires the least amount of quantum memory

// taking the previous results, one could simply filter them:

let all_paths = vec![
    (4, 3, vec![vec![0], vec![3], vec![1], vec![2]]), // optimal
    (4, 3, vec![vec![0], vec![3], vec![2], vec![1]]),
    (3, 3, vec![vec![0], vec![3], vec![1, 2]]), // optimal
    (4, 3, vec![vec![0], vec![1], vec![3], vec![2]]),
    (3, 3, vec![vec![0], vec![3, 1], vec![2]]),
];

//                             memory  lenght   path
let mut optimal_paths: HashMap<usize, (usize, Vec<Vec<usize>>)> = HashMap::new();
for (len, mem, path) in all_paths {
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

// however, this required us to first calculate all paths; we can do better by filtering
// out paths for which we know that they cannot be optimal


let graph_buffer = GraphBuffer::new(&graph_state_edges, num_bits, None, false);
let mut dependency_buffer = DependencyBuffer::new(num_bits);
let scheduler = Scheduler::new(
    PathGenerator::from_dependency_graph(time_ordering, &mut dependency_buffer, None),
    Graph::new(&graph_buffer),
);


let mut results = Vec::new();
let mut current_path = Vec::new();

// we keep track of the minimum required memory for a given path length
let mut predicates = vec![num_bits + 1; num_bits + 1];

let mut scheduler = scheduler.into_iter();

while let Some(step) = scheduler.next() {
    match step {
        Step::Forward(mess) => {
            // we get the current state of the scheduler, so that we can make an
            // estimate about the minimum path length for all paths that can be
            // generated from this state
            let current = scheduler.current();
            let time = current.time();
            let minimum_path_length = if time.at_leaf().is_some() {
                current_path.len() + 1 // captured path plus the current step
            } else if time.has_unmeasureable() {
                // capture path
                //     plus current step
                //     plus at least one more because there are some measurable (since
                //          we are not at a leaf)
                //     plus at least one more because there are some not-measurable bits
                //          left that cannot be measured in the next step
                current_path.len() + 3
            } else {
                current_path.len() + 2 // similar as above
            };
            // if the already maximal required memory for the current subpath is bigger
            // than minimal required memory for paths which have the same length or are
            // shorted, we skip this state and with that all paths that could be
            // generated from this state
            if current.space().max_memory() >= predicates[minimum_path_length] {
                if scheduler.skip_current().is_err() {
                    break;
                }
            } else {
                current_path.push(mess);
            }
        }
        Step::Backward(leaf) => {
            if let Some(mem) = leaf {
                // we update our predicates, because we have found a path which is
                // optimal so far
                predicates[current_path.len()] = mem;
                results.push((current_path.len(), mem, current_path.clone()));
            }
            current_path.pop();
            // no sense in skipping here, because if it we could skip, we would
            // have done it already in the forward step that led to this focused
            // state
        }
    }
}

assert_eq!(
    results,
    vec![
        (4, 3, vec![vec![0], vec![3], vec![1], vec![2]]),
        (3, 3, vec![vec![0], vec![3], vec![1, 2]]),
    ]
);

// Note that while in this example, we directly get the two optimal paths, in general,
// one would need to filter the `results`, as we had filtered `all_results`, because we
// are keeping paths in `results` even if we find a with the same length but less
// memory. This can, of course, also be done directly in the loop above.
# }
# #[cfg_attr(coverage_nightly, no_coverage)]
# #[cfg(not(feature = "scheduler"))]
# fn main() {}
```

### Paralyzing some parts
```
# #[cfg_attr(coverage_nightly, no_coverage)]
# #[cfg(feature = "scheduler")]
# fn main() {
use std::collections::HashMap;
# #[rustfmt::skip]
use pauli_tracker::tracker::frames::dependency_graph::DependencyGraph;
# #[rustfmt::skip]
use pauli_tracker::scheduler::{
    Scheduler,
    space::{Graph, GraphBuffer},
    time::{PathGenerator, DependencyBuffer},
    tree::{Step, FocusIterator},
};

// we consider the same example as above
let graph_state_edges = [(0, 1), (0, 2), (1, 3), (3, 2)];
let time_ordering: DependencyGraph = vec![
    vec![(0, vec![])],
    vec![(3, vec![0]), (1, vec![0])],
    vec![(2, vec![3])]
];
let num_bits = 4;
# }
# #[cfg_attr(coverage_nightly, no_coverage)]
# #[cfg(not(feature = "scheduler"))]
# fn main() {}
```

[graph state]: https://en.wikipedia.org/wiki/Graph_state
[DependencyGraph]: crate::tracker::frames::dependency_graph::DependencyGraph
*/

mod combinatoric;

pub use combinatoric::Partition;
use time::Partitioner;

use self::{
    space::{
        AlreadyMeasured,
        Graph,
    },
    time::{
        MeasurableSet,
        NotMeasurable,
        PathGenerator,
    },
    tree::{
        Focus,
        FocusIterator,
        Step,
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

/// A scheduler to generate allowed paths scheduling paths, capturing the required
/// quantum memory. Compare the [module documentation](crate::scheduler).
#[derive(Debug, Clone)]
pub struct Scheduler<'l, T> {
    time: PathGenerator<'l, T>,
    space: Graph<'l>,
}

impl<'l, T> Scheduler<'l, T> {
    /// Create a new scheduler.
    pub fn new(time: PathGenerator<'l, T>, space: Graph<'l>) -> Self {
        Self { time, space }
    }

    /// Get a reference to the underlying [PathGenerator].
    pub fn time(&self) -> &PathGenerator<'l, T> {
        &self.time
    }

    /// Get a reference to the underlying [Graph].
    pub fn space(&self) -> &Graph {
        &self.space
    }
}

// just for seeing whether it works as expected while developing
// pub(crate) static mut COUNT: usize = 0;

impl<T: MeasurableSet<usize>> Focus<&[usize]> for Scheduler<'_, T> {
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

impl FocusIterator for Scheduler<'_, Partitioner> {
    type IterItem = Vec<usize>;
    type LeafItem = usize;

    fn next_and_focus(&mut self) -> Option<(Self, Self::IterItem)>
    where
        Self: Sized,
    {
        let (new_time, mess) = self.time.next_and_focus()?;
        // unsafe { COUNT += 1 };
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

/// An error that can happen when instructing the [Scheduler].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum InstructionError {
    /// See [NotMeasurable].
    #[error(transparent)]
    NotMeasurable(#[from] NotMeasurable),
    /// See [AlreadyMeasured].
    #[error(transparent)]
    AlreadyMeasured(#[from] AlreadyMeasured),
}

#[doc = non_semantic_default!()]
impl Default for InstructionError {
    fn default() -> Self {
        Self::NotMeasurable(NotMeasurable::default())
    }
}

impl<'l> IntoIterator for Scheduler<'l, Partitioner> {
    type Item = Step<Vec<usize>, Option<usize>>;
    type IntoIter = Sweep<Self>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;
    use hashbrown::HashMap;

    use super::{
        time::DependencyBuffer,
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

        let mut lookup_buffer = DependencyBuffer::new(num);
        let graph = Graph::new(&graph_buffer);
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
                        if scheduler.skip_current().is_err() {
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
