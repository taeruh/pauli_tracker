Analyse scheduling paths on a [graph state] (or similar) allowed by a [DependencyGraph].

**This module is currently rather experimental. It may be put into a separate crate in
the future.**

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
paths is given by the [ordered Bell number]. Therefore, if the task is to find the
optimal path, under some metric, doing this by checking all possible paths might scale
very, very badly. The [Sweep] iterator in [tree] tries to reduce that scaling by keeping
track of its states to reduce the number of redundant calculations (this comes at the
cost of memory, but this cost is scaling linearly), and a [skipping method] to skip
states that are known to be not interesting. However, the scaling can still be very,
very bad.

Saying that, if you have a high number of qubits, you might not want to use this module,
but it might still be useful for testing purposes.

It is a goal to find better algorithms, but this will probably happen in a separate
project.

# Examples

### Capturing all paths
```rust
# #[cfg_attr(coverage_nightly, coverage(off))]
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
// we could now generate paths with `path_generator` and feed them into `graph`; here
// however, we wrap them into a Scheduler which basically does exactly that for us (if
// one needs more flexibility one can use them separately)
let scheduler = Scheduler::new(path_generator, graph);
// the Partitioner above, as generic parameter, will allow us to iterate over all paths;
// if we would want to create the paths manually, we could use Vec<usize> instead, and
// for example, do something like the following to get the shortest path
//    while !scheduler.time().measurable().is_empty() {
//        let measurable_set = scheduler.time().measurable().clone();
//        scheduler.focus_inplace(&measurable_set)?;
//        println!("{:?}", measurable_set);
//        println!("{:?}", scheduler.space().max_memory());
//    }

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
        (       3,            3,         vec![vec![0], vec![3, 1], vec![2]]),
        (       4,            3,         vec![vec![0], vec![1], vec![3], vec![2]]),
        (       3,            3,         vec![vec![0], vec![3], vec![1, 2]]),
        (       4,            3,         vec![vec![0], vec![3], vec![2], vec![1]]),
        (       4,            3,         vec![vec![0], vec![3], vec![1], vec![2]]),
    ]
);
# }
# #[cfg_attr(coverage_nightly, coverage(off))]
# #[cfg(not(feature = "scheduler"))]
# fn main() {}
```

### Finding the optimal paths
```rust
# #[cfg_attr(coverage_nightly, coverage(off))]
# #[cfg(feature = "scheduler")]
# fn main() {
use std::{cmp, collections::HashMap};
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
    (3, 3, vec![vec![0], vec![3, 1], vec![2]]),
    (4, 3, vec![vec![0], vec![1], vec![3], vec![2]]),
    (3, 3, vec![vec![0], vec![3], vec![1, 2]]), // optimal
    (4, 3, vec![vec![0], vec![3], vec![2], vec![1]]),
    (4, 3, vec![vec![0], vec![3], vec![1], vec![2]]), // optimal
];

//                                       memory  lenght   path
let mut optimal_for_each_length: HashMap<usize, (usize, Vec<Vec<usize>>)> = HashMap::new();
for (len, mem, path) in all_paths {
    if let Some(optimal) = optimal_for_each_length.get_mut(&len) {
        if optimal.0 > mem {
            *optimal = (mem, path);
        }
    } else {
        optimal_for_each_length.insert(len, (mem, path));
    }
}

assert_eq!(
    optimal_for_each_length,
    HashMap::from_iter(vec![
        (3, (3, vec![vec![0], vec![3, 1], vec![2]])),
        (4, (3, vec![vec![0], vec![1], vec![3], vec![2]])),
    ])
);

// however, this required us to first calculate all paths; we can do better by directly
// skipping paths for which we know that they cannot be optimal

let graph_buffer = GraphBuffer::new(&graph_state_edges, num_bits, None, false);
let mut dependency_buffer = DependencyBuffer::new(num_bits);
let scheduler = Scheduler::new(
    PathGenerator::from_dependency_graph(time_ordering, &mut dependency_buffer, None),
    Graph::new(&graph_buffer),
);


let mut skipped_results = HashMap::new();
let mut current_path = Vec::new();

// we keep track of the minimum required memory for a given path length
let mut best_memory = vec![num_bits + 1; num_bits + 1];

let mut scheduler = scheduler.into_iter();

while let Some(step) = scheduler.next() {
    match step {
        Step::Forward(measure) => {
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
            // than the minimal required memory for paths which have the same length or
            // are shorter, we skip this state and with that all paths that could be
            // generated from this state
            if current.space().max_memory() >= best_memory[minimum_path_length] {
                if scheduler.skip_current().is_err() {
                    break;
                }
            } else {
                current_path.push(measure);
            }
        }
        Step::Backward(leaf) => {
            if let Some(mem) = leaf {
                // we update our best_memory, because we have found a path which is
                // optimal so far (it is mem < best_memory[current_path.len()] because
                // otherwise we would have skipped this state above
                best_memory[current_path.len()] = mem;
                // we also update the memory for all longer paths because we don't want
                // a longer path with the same or more memory
                for m in best_memory[current_path.len() + 1..].iter_mut() {
                    *m = cmp::min(*m, mem);
                }
                skipped_results.insert(current_path.len(), (mem, current_path.clone()));
            }
            current_path.pop();
            // no sense in skipping here, because if it we could skip, we would
            // have done it already in the forward step that led to this focused
            // state
        }
    }
}

// we finally filter out all paths that are longer than some other path but don't have
// less memory (in this case that is actually not required for skipper_results, but
// sometimes it might be; anyway, we need it for the filtered_from_all down below)

fn final_filter(
    num_bits: usize,
    paths: HashMap<usize, (usize, Vec<Vec<usize>>)>,
) -> HashMap<usize, (usize, Vec<Vec<usize>>)> {
    let mut best_memory = vec![num_bits + 1; num_bits + 1];
    let mut res: HashMap<usize, (usize, Vec<Vec<usize>>)> = HashMap::new();
    for i in 0..best_memory.len() {
        if let Some((mem, _)) = paths.get(&i) {
            let m = best_memory[i];
            if *mem < m {
                res.insert(i, paths.get(&i).unwrap().clone());
                for m in best_memory[i..].iter_mut() {
                    *m = *mem;
                }
            }
        }
    }
    res
}

let filtered_from_skipped_results = final_filter(num_bits, skipped_results);
let filtered_from_all = final_filter(num_bits, optimal_for_each_length);

assert_eq!(
    filtered_from_skipped_results,
    filtered_from_all,
);
assert_eq!(
    filtered_from_skipped_results,
    HashMap::from_iter([(3, (3, vec![vec![0], vec![3, 1], vec![2]]))]),
);
# }
# #[cfg_attr(coverage_nightly, coverage(off))]
# #[cfg(not(feature = "scheduler"))]
# fn main() {}
```

# Parallelization

We don't provide explicit methods to generate the paths in parallel, however, it can be
done by "simply" splitting the iterations. In case of the [Sweep] iterators, one can,
for example, set different initial states per thread. One can also first calculate all
paths with the [PathGenerator], split them up, and then instruct [Graph] with them. In
the [scheduling-proptest] is some messy code which does exactly that (the
`split_instructions` function and the code after that function call).

[DependencyGraph]: crate::tracker::frames::dependency_graph::DependencyGraph
[graph state]: https://en.wikipedia.org/wiki/Graph_state
[ordered Bell number]: https://en.wikipedia.org/wiki/Ordered_Bell_number
[scheduling-proptest]: https://github.com/taeruh/pauli_tracker/blob/main/pauli_tracker/tests/roundtrips/scheduling.rs
[skipping method]: Sweep::skip_current
