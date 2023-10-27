#![doc = include_str!("../xdocs/scheduler.md")]

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

impl<T: MeasurableSet> Focus<&[usize]> for Scheduler<'_, T> {
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
            .measurable()
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
