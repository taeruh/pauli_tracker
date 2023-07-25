/*!
Given a [DependencyGraph], there are only specific allowed sequences of measuring
qubits. This module provides a [PathGenerator] that can generate all allowed paths.

The time ordering defined by the [DependencyGraph] is often induced by non-determinism
introduced by quantum measurements, e.g., as in MBQC, and captured by a Pauli tracker
(cf. [README](https://github.com/taeruh/pauli_tracker)).
*/

use std::{
    error::Error,
    fmt::Display,
    hash::BuildHasherDefault,
};

use hashbrown::HashMap;
use rustc_hash::FxHasher;

pub use super::combinatoric::Partition;
use super::tree::{
    Focus,
    FocusIterator,
    Sweep,
};
use crate::tracker::frames::dependency_graph::DependencyGraph;

type DepsCounters = HashMap<usize, usize, BuildHasherDefault<FxHasher>>;
type Lookup = Vec<Vec<usize>>;

/// A buffer that holds the dependency structure implied by a [DependencyGraph].
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyBuffer {
    look: Lookup,
}

impl DependencyBuffer {
    /// Initialize a new [DependencyBuffer] to hold the dependency structure for `len`
    /// qubits.
    pub fn new(len: usize) -> Self {
        Self { look: vec![Vec::new(); len] }
    }
}

type Set = Vec<usize>;

/// An iterator over all partitions of a set of integers.
pub type Partitioner = Partition<Set>;

/// A generator to create a scheduling path - initialization and measuring of qubits -
/// allowed by a [DependencyGraph].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathGenerator<'l, T /* Measurable */> {
    // one could also put the dependents with the bit into the partition set and in deps
    // have values of the form (dependents, dependencies), however, the Partition clones
    // the set multiple times, therefore we don't want the dependents in there (also it
    // makes the from(DependencyGraph) function and the step function simpler if it is
    // separated)
    measurable: T,
    deps: DepsCounters,
    // it would have been slightly more ergnomic to use use an Rc instead of a reference
    // (no need to keep the actual map in an extra variable), however, this would have
    // been slightly less performant (Rc has an overhead every time it is cloned or
    // dropped, which happens rather often when sweeping); alternatively one could own
    // the map and always clone it, this would have the benefit that we could remove
    // elements from the map in the step function, however, since we are using a
    // HashMap, this does not really change the lookup time, rather the removing might
    // cause a slight overhead, and also we have the additional time and space overhead
    // when cloning it
    look: &'l Lookup,
}

impl<'l, T> PathGenerator<'l, T> {
    fn new(measureable: T, deps: DepsCounters, look: &'l Lookup) -> Self {
        Self {
            measurable: measureable,
            deps,
            look,
        }
    }

    /// Get a reference to currently the measurable set of qubits.
    pub fn measureable(&self) -> &T {
        &self.measurable
    }

    /// Check whether there are qubits that cannot be measured yet.
    pub fn has_unmeasureable(&self) -> bool {
        !self.deps.is_empty()
    }
}

impl<'l, T: MeasurableSet<usize>> PathGenerator<'l, T> {
    /// Create a new [PathGenerator] from a [DependencyGraph]. `dependency_buffer` is
    /// going to own the dependency structure implied by the `graph`, so that it can be
    /// reused again.
    ///
    /// # Panics
    /// Panics if the dependency_buffer has a length smaller than the number of qubits
    /// in the `graph`
    pub fn from_dependency_graph(
        mut graph: DependencyGraph,
        dependency_buffer: &'l mut DependencyBuffer,
        bit_mapping: Option<&HashMap<usize, usize>>,
    ) -> Self {
        let look = &mut dependency_buffer.look;

        if graph.is_empty() {
            return Self {
                measurable: T::default(),
                deps: DepsCounters::default(),
                look,
            };
        }

        // one could/should? do some similar macro stuff as in super::space to get rid
        // of one loop run ...
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

        fn resolve(bit: usize, rest: &[Vec<(usize, Vec<usize>)>], look: &mut Lookup) {
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

        let mut measureable = Vec::new();
        let mut deps = HashMap::default();

        let mut graph_iter = graph.into_iter();

        let first = graph_iter.next().unwrap();
        let rest = graph_iter.as_ref();
        for (bit, _) in first {
            resolve(bit, rest, look);
            measureable.push(bit);
        }

        while let Some(layer) = graph_iter.next() {
            let rest = graph_iter.as_ref();
            for (bit, dependency) in layer {
                resolve(bit, rest, look);
                deps.insert(bit, dependency.len());
            }
        }

        let measureable = T::init(measureable);
        Self {
            measurable: measureable,
            deps,
            look,
        }
    }

    // check whether the measure_set is really measurable and return the new
    // measurable set
    fn partition(&self, measure_set: &[usize]) -> Result<Vec<usize>, NotMeasurable> {
        let mut new_measurable_set =
            Vec::with_capacity(self.measurable.set().len() - measure_set.len());
        let mut copy_measure_set = measure_set.to_vec();
        for e in self.measurable.set().iter() {
            if let Some(p) = copy_measure_set.iter().position(|m| m == e) {
                copy_measure_set.swap_remove(p);
            } else {
                new_measurable_set.push(*e);
            }
        }
        if !copy_measure_set.is_empty() {
            return Err(NotMeasurable(copy_measure_set));
        }
        Ok(new_measurable_set)
    }

    // "unchecked" in the sense that it does not check if the measure_set is a subset of
    // self.measurable and does not overlap with new_measurable_set

    // # Panics
    // Panics if measure_set contains a bit with a dependent that is already resolved.
    fn update_unchecked(
        look: &Lookup, // always self.look; don't use self because of borrow problems
        deps: &mut DepsCounters, // might be self.deps
        measure_set: &[usize],
        new_measurable_set: &mut Vec<usize>,
    ) {
        for measure in measure_set.iter() {
            let dependents = &look[*measure];
            for bit in dependents {
                let dependency_count = deps
                    .get_mut(bit)
                    .unwrap_or_else(|| panic!("the {bit} is already resolved"));
                *dependency_count -= 1;
                if *dependency_count == 0 {
                    deps.remove(bit)
                        .expect("bug: already checked with the expect above");
                    new_measurable_set.push(*bit);
                }
            }
        }
    }

    // # Panics
    // Panics if measure_set is not a subset of self.measureable
    fn focus_unchecked(
        &mut self,
        measure_set: &[usize],
        mut new_measureable_set: Vec<usize>,
    ) -> Self {
        let mut deps = self.deps.clone();
        Self::update_unchecked(
            self.look,
            &mut deps,
            measure_set,
            &mut new_measureable_set,
        );
        Self::new(T::init(new_measureable_set), deps, self.look)
    }
}

impl<T: MeasurableSet<usize>> Focus<&[usize]> for PathGenerator<'_, T> {
    type Error = NotMeasurable;

    fn focus(&mut self, measure_set: &[usize]) -> Result<Self, NotMeasurable>
    where
        Self: Sized,
    {
        // self.partition already ensures the input is okay
        Ok(self.focus_unchecked(measure_set, self.partition(measure_set)?))
    }

    fn focus_inplace(&mut self, measure_set: &[usize]) -> Result<(), NotMeasurable> {
        let mut new_measureable_set = self.partition(measure_set)?;
        // self.partition already catches ensures the input is okay
        Self::update_unchecked(
            self.look,
            &mut self.deps,
            measure_set,
            &mut new_measureable_set,
        );
        self.measurable = T::init(new_measureable_set);
        Ok(())
    }
}

impl FocusIterator for PathGenerator<'_, Partitioner> {
    type IterItem = Vec<usize>;
    type LeafItem = ();

    fn next_and_focus(&mut self) -> Option<(Self, Self::IterItem)>
    where
        Self: Sized,
    {
        let (measuring, new_measurable_set) = self.measurable.next()?;
        Some((
            // we know that the input is fine, because it is a partition of
            // self.measurable
            self.focus_unchecked(&measuring, new_measurable_set),
            measuring,
        ))
    }

    fn at_leaf(&self) -> Option<Self::LeafItem> {
        self.measurable.set.is_empty().then_some(())
    }
}

impl<'l> IntoIterator for PathGenerator<'l, Partition<Vec<usize>>> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = Sweep<Self>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

mod sealed {
    use super::Partition;
    pub trait Sealed {}
    impl<T> Sealed for Vec<T> {}
    impl<T> Sealed for Partition<Vec<T>> {}
}

/// A trait for types that describe the set of measurable qubits in [PathGenerator].
///
/// Use [`Vec<usize>`] if you want to manually create the paths, and [Partitioner] if you
/// want to iterate over all paths.
///
/// **This trait is sealed**
pub trait MeasurableSet<T>: sealed::Sealed + Default {
    /// Create a new instance of the type from a set of measurable qubits.
    fn init(set: Vec<T>) -> Self;

    /// Get the set of measurable qubits.
    fn set(&self) -> &[T];
}

impl<T> MeasurableSet<T> for Vec<T> {
    #[inline(always)]
    fn init(set: Vec<T>) -> Self {
        set
    }

    #[inline]
    fn set(&self) -> &[T] {
        self
    }
}

impl<T: Clone> MeasurableSet<T> for Partition<Vec<T>> {
    fn init(set: Vec<T>) -> Self {
        let len = set.len();
        let mut res = Partition::new(set, len);
        res.next();
        res
    }

    #[inline]
    fn set(&self) -> &[T] {
        &self.set
    }
}

/// An error that is returned when trying to measure a qubit that is not measurable yet,
/// i.e., it's dependencies haven't been measured yet.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NotMeasurable(Vec<usize>);
impl Display for NotMeasurable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the bits {:?} are not in the measureable set", self.0)
    }
}
impl Error for NotMeasurable {}

#[cfg(test)]
pub(crate) mod tests {
    use std::panic;

    use coverage_helper::test;

    use super::{
        super::tree::Step,
        *,
    };

    #[cfg_attr(coverage_nightly, no_coverage)]
    pub fn example_ordering() -> DependencyGraph {
        // 0 --- 3 --- 2
        //  \
        //    -- 1
        vec![vec![(0, vec![])], vec![(3, vec![0]), (1, vec![0])], vec![(2, vec![3])]]
    }

    #[cfg_attr(coverage_nightly, no_coverage)]
    fn get_all_paths(
        generator: PathGenerator<'_, Partitioner>,
    ) -> Vec<Vec<Vec<usize>>> {
        let mut results = Vec::new();
        let mut path = Vec::new();
        for step in generator {
            match step {
                Step::Forward(set) => path.push(set),
                Step::Backward(leaf) => {
                    if let Some(()) = leaf {
                        results.push(path.clone());
                    }
                    path.pop();
                }
            }
        }

        results
    }

    #[test]
    fn simple_paths() {
        let mut buffer = DependencyBuffer::new(5);
        let time = PathGenerator::<Partitioner>::from_dependency_graph(
            example_ordering(),
            &mut buffer,
            None,
        );

        assert_eq!(
            get_all_paths(time),
            vec![
                vec![vec![0], vec![3], vec![1], vec![2]],
                vec![vec![0], vec![3], vec![2], vec![1]],
                vec![vec![0], vec![3], vec![1, 2]],
                vec![vec![0], vec![1], vec![3], vec![2]],
                vec![vec![0], vec![3, 1], vec![2]],
            ]
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn max() {
        const ORDERED_BELL_NUMBERS: [usize; 6] = [
            1, 1, 3, 13, 75, 541, // 4683, 47293, 545835,  7087261, 102247563
        ];

        let mut buffer = DependencyBuffer::new(10);

        for (n, &result) in ORDERED_BELL_NUMBERS.iter().enumerate() {
            let time = PathGenerator::<Partitioner>::from_dependency_graph(
                if n == 0 {
                    vec![]
                } else {
                    vec![(0..n).map(|i| (i, vec![])).collect()]
                },
                &mut buffer,
                None,
            );
            assert_eq!(get_all_paths(time).len(), result);
        }
    }

    #[test]
    fn wrong_instructions() {
        let dependency_graph = vec![
            vec![(5, vec![])],
            vec![(2, vec![5]), (8, vec![5])],
            vec![(4, vec![8, 5])],
        ];
        let map = [5, 8, 2, 4];

        let mut buffer = DependencyBuffer::new(5);

        assert!(
            panic::catch_unwind(|| {
                let mut buffer = buffer.clone();
                let _ = PathGenerator::<Partitioner>::from_dependency_graph(
                    dependency_graph.clone(),
                    &mut buffer,
                    None,
                );
            })
            .is_err()
        );

        let map = map
            .iter()
            .enumerate()
            .map(|(i, &e)| (e, i))
            .collect::<HashMap<_, _>>();

        let mut time = PathGenerator::<Partitioner>::from_dependency_graph(
            dependency_graph,
            &mut buffer,
            Some(&map),
        );

        assert_eq!(time.focus_inplace(&[5]).unwrap_err(), NotMeasurable(vec![5]));

        assert_eq!(
            time.focus_inplace(&[map[&8]]).unwrap_err(),
            NotMeasurable(vec![map[&8]])
        );

        time.focus_inplace(&[map[&5]]).unwrap();

        assert_eq!(
            time.focus_inplace(&[map[&5]]).unwrap_err(),
            NotMeasurable(vec![map[&5]])
        );
    }
}

// currently, when looping through the partitions in next_and_focus, we do
// let (measuring, new_measureable_set) = self.measureable.next()?;
// if we want to swap the order, we have to do
// let (mut new_measureable_set, measuring) = self.measureable.next()?;
// if measuring.is_empty() {
//     return None;
// }
// and in new_partition we have to comment
// measureable.next();
