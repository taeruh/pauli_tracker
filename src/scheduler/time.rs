/*!
...
*/

use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    // mem,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    combinatoric::Partition,
    tree::{
        Focus,
        FocusIterator,
        Sweep,
    },
};
use crate::tracker::frames::dependency_graph::DependencyGraph;

type Deps = HashMap<usize, Vec<usize>>;
type Look = Vec<Vec<usize>>;

#[derive(Clone, Debug)]
pub struct LookupBuffer {
    look: Look,
}

impl LookupBuffer {
    pub fn new(num_bits: usize) -> Self {
        Self { look: vec![Vec::new(); num_bits] }
    }
}

pub type Set = Vec<usize>;
pub type Partitioner = Partition<Set>;
#[derive(Debug, Clone)]
pub struct PathGenerator<'l, T /* : Init<I> */> {
    // one could also put the dependents with the bit into the partition set and in deps
    // have vaules of the form (dependents, dependencies), however, the Partition clones
    // the set multiple times, therefore we don't want the dependents in there (also it
    // makes the from(DependencyGraph) function and the step function simpler if it is
    // separated)
    measureable: T,
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

impl<'l, T> PathGenerator<'l, T> {
    fn new(measureable: T, deps: Deps, look: &'l Look) -> Self {
        Self { measureable, deps, look }
    }

    pub fn measureable(&self) -> &T {
        &self.measureable
    }

    pub fn has_unmeasureable(&self) -> bool {
        !self.deps.is_empty()
    }
}

impl<'l, T: Init<usize>> PathGenerator<'l, T> {
    pub fn from_dependency_graph(
        mut graph: DependencyGraph,
        look: &'l mut LookupBuffer,
        bit_mapping: Option<&HashMap<usize, usize>>,
    ) -> Self {
        let look = &mut look.look;

        if graph.is_empty() {
            return Self {
                measureable: T::default(),
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

        let mut measureable = Vec::new();
        let mut deps = HashMap::new();

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
                deps.insert(bit, dependency);
            }
        }

        let measureable = T::init(measureable);
        Self { measureable, deps, look }
    }

    fn partition(
        &self,
        measure_set: &[usize],
    ) -> Result<Vec<usize>, TimeOrderingViolation> {
        let mut new_measureable_set =
            Vec::with_capacity(self.measureable.set().len() - measure_set.len());
        let mut copy_measure_set = measure_set.to_vec();
        for e in self.measureable.set().iter() {
            if let Some(p) = copy_measure_set.iter().position(|m| m == e) {
                copy_measure_set.swap_remove(p);
            } else {
                new_measureable_set.push(*e);
            }
        }
        if !copy_measure_set.is_empty() {
            return Err(TimeOrderingViolation::NotMeasureable(copy_measure_set));
        }
        Ok(new_measureable_set)
    }

    // "unchecked" in the sense that it does not check if the measure_set is a subset of
    // self.measureable and does not overlap with new_measureable_set

    fn update_unchecked(
        look: &Look, // always self.look; don't use self because of borrow problems
        deps: &mut Deps, // might be self.deps
        measure_set: &[usize],
        new_measureable_set: &mut Vec<usize>,
    ) -> Result<(), TimeOrderingViolation> {
        for measure in measure_set.iter() {
            let dependents = &look[*measure];
            for bit in dependents {
                let dependencies = match deps.get_mut(bit) {
                    Some(s) => s,
                    None => {
                        return Err(TimeOrderingViolation::MissingDependent(
                            *measure, *bit,
                        ));
                    }
                };
                let pos = dependencies.iter().position(|e| e == measure).expect(
                    "bug: the creation of self via from_dependency_graph guarantees \
                     that the measureable bit is in dependencies",
                );
                dependencies.swap_remove(pos);
                if dependencies.is_empty() {
                    deps.remove(bit)
                        .expect("bug: we checked it already above with get_mut");
                    new_measureable_set.push(*bit);
                }
            }
        }
        Ok(())
    }

    fn focus_unchecked(
        &mut self,
        measure_set: &[usize],
        mut new_measureable_set: Vec<usize>,
    ) -> Result<Self, TimeOrderingViolation> {
        let mut deps = self.deps.clone();
        Self::update_unchecked(
            self.look,
            &mut deps,
            measure_set,
            &mut new_measureable_set,
        )?;
        Ok(Self::new(T::init(new_measureable_set), deps, self.look))
    }
}

impl<T: Init<usize>> Focus<&[usize]> for PathGenerator<'_, T> {
    type Error = TimeOrderingViolation;

    fn focus(&mut self, measure_set: &[usize]) -> Result<Self, TimeOrderingViolation>
    where
        Self: Sized,
    {
        self.focus_unchecked(measure_set, self.partition(measure_set)?)
    }

    fn focus_inplace(
        &mut self,
        measure_set: &[usize],
    ) -> Result<(), TimeOrderingViolation> {
        let mut new_measureable_set = self.partition(measure_set)?;
        Self::update_unchecked(
            self.look,
            &mut self.deps,
            measure_set,
            &mut new_measureable_set,
        )?;
        self.measureable = T::init(new_measureable_set);
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
        let (measuring, new_measureable_set) = self.measureable.next()?;
        Some((
            // we know that the input is fine, because it comes from
            // self.measureable.next()
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
        self.measureable.set.is_empty().then_some(())
    }
}

impl<'l> IntoIterator for PathGenerator<'l, Partition<Vec<usize>>> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = Sweep<Self>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self, Vec::new())
    }
}

mod sealed {
    use super::Partition;
    pub trait Sealed {}
    impl<T> Sealed for Vec<T> {}
    impl<T> Sealed for Partition<Vec<T>> {}
}

pub trait Init<T>: sealed::Sealed + Default {
    fn init(set: Vec<T>) -> Self;
    fn set(&self) -> &[T];
}
impl<T> Init<T> for Vec<T> {
    #[inline(always)]
    fn init(set: Vec<T>) -> Self {
        set
    }

    #[inline]
    fn set(&self) -> &[T] {
        self
    }
}

impl<T: Clone> Init<T> for Partition<Vec<T>> {
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TimeOrderingViolation {
    // with the current API, this error can actually never happen
    MissingDependent(usize, usize),
    NotMeasureable(Vec<usize>),
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
            TimeOrderingViolation::NotMeasureable(set) => {
                write!(f, "the bits {set:?} are not in the measureable set",)
            }
        }
    }
}

impl Error for TimeOrderingViolation {}

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
        let mut buffer = LookupBuffer::new(5);
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

        let mut buffer = LookupBuffer::new(10);

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

        let mut buffer = LookupBuffer::new(5);

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

        assert_eq!(
            time.focus_inplace(&[5]).unwrap_err(),
            TimeOrderingViolation::NotMeasureable(vec![5])
        );

        assert_eq!(
            time.focus_inplace(&[map[&8]]).unwrap_err(),
            TimeOrderingViolation::NotMeasureable(vec![map[&8]])
        );

        time.focus_inplace(&[map[&5]]).unwrap();

        assert_eq!(
            time.focus_inplace(&[map[&5]]).unwrap_err(),
            TimeOrderingViolation::NotMeasureable(vec![map[&5]])
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
