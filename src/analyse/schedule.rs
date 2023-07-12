/*!
...
*/

use std::fmt::Display;

use self::{
    space::{
        AlreadyMeasured,
        Graph,
    },
    time::{
        PathGenerator,
        TimeOrderingViolation,
    },
    tree::{
        Focus,
        FocusIterator,
    },
};

macro_rules! update {
    ($bit:expr, $map:expr) => {
        $map.get($bit)
            .unwrap_or_else(|| panic!("no bit mapping for bit {}", $bit))
    };
    ($bit:expr; $map:expr) => {
        *$bit = *update!($bit, $map);
    };
}
pub mod space;
pub mod time;
pub mod tree;

pub struct Scheduler<'l> {
    time: PathGenerator<'l>,
    space: Graph,
}

impl<'l> Scheduler<'l> {
    pub fn new(time: PathGenerator<'l>, space: Graph) -> Self {
        Self { time, space }
    }
}

// just for seeing whether it works as expected while developing
pub(crate) static mut COUNT: usize = 0;

impl Focus<(&Vec<usize>, Vec<usize>)> for Scheduler<'_> {
    type Error = InstructionError;
    fn focus(
        &mut self,
        instruction: (&Vec<usize>, Vec<usize>),
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let new_space = self.space.focus(instruction.0)?;
        let new_time = self.time.focus(instruction)?;
        Ok(Self { time: new_time, space: new_space })
    }
}

impl FocusIterator for Scheduler<'_> {
    type IterItem = Vec<usize>;
    type LeafItem = usize;

    fn next_and_focus(&mut self) -> Option<(Self, Self::IterItem)>
    where
        Self: Sized,
    {
        let (new_time, mess) = self.time.next_and_focus()?;
        unsafe { COUNT += 1 };
        let new_space = self.space.focus(&mess).unwrap();
        Some((Self { time: new_time, space: new_space }, mess))
    }

    fn at_leaf(&self) -> Option<Self::LeafItem> {
        self.time.finished_path().then_some(self.space.max_memory())
    }
}

#[derive(Debug, Clone)]
pub enum InstructionError {
    TimeOrderingViolation(TimeOrderingViolation),
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
impl From<TimeOrderingViolation> for InstructionError {
    fn from(error: TimeOrderingViolation) -> Self {
        Self::TimeOrderingViolation(error)
    }
}
impl From<AlreadyMeasured> for InstructionError {
    fn from(error: AlreadyMeasured) -> Self {
        Self::AlreadyMeasured(error)
    }
}

tree::impl_into_iterator!(Scheduler with <'l>);

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;
    use crate::analyse::schedule::tree::Step;

    #[test]
    fn scheduler() {
        let (space, time, _) = input();

        let num = space.len();
        // just for playing around, unnecessary in real test; num < nodes_in_time would
        // definitely panic
        assert_eq!(num, {
            // nodes in time
            let mut len = 0;
            time.iter().for_each(|e: &Vec<_>| len += e.len());
            len
        });

        let mut look = time::LookupBuffer::new(5);

        let scheduler = Scheduler::new(
            PathGenerator::from_dependency_graph(time, &mut look, None),
            Graph::new(num, &space, None),
        );

        let mut path = Vec::new();
        let mut results = Vec::new();

        for step in scheduler {
            match step {
                Step::Forward(mess) => {
                    path.push(mess);
                }
                Step::Backward(at_end) => {
                    if let Some(max_memory) = at_end {
                        println!("{}; {}; {:?}", path.len(), max_memory, path);
                        results.push(path.clone());
                    }
                    path.pop();
                }
            }
        }
        println!("result: {:?}", results.len());
        println!("count: {:?}", unsafe { COUNT });
    }

    #[test]
    fn skipper() {
        let (space, time, num_nodes) = input();

        let mut predicates = vec![space.len() + 1; num_nodes + 1];

        let mut path = Vec::new();
        let mut results = Vec::new();

        let mut look = time::LookupBuffer::new(5);
        let mut scheduler = Scheduler::new(
            PathGenerator::from_dependency_graph(time, &mut look, None),
            Graph::new(space.len(), &space, None),
        )
        .into_iter();

        while let Some(step) = scheduler.next() {
            match step {
                Step::Forward(mess) => {
                    let current = scheduler.current();
                    let minimum_time = if current.time.finished_path() {
                        path.len() + 1
                    } else {
                        path.len() + 2
                    };
                    // unsafe { COUNT += 1 };
                    if current.space.max_memory() >= predicates[minimum_time] {
                        if scheduler.skip_focus().is_err() {
                            break;
                        }
                    } else {
                        path.push(mess);
                    }
                }
                Step::Backward(at_end) => {
                    if let Some(max_memory) = at_end {
                        predicates[path.len()] = max_memory;
                        println!("{}; {}; {:?}", path.len(), max_memory, path);
                        results.push(path.clone());
                    }
                    path.pop();
                    let current = scheduler.current();
                    // we are never at an end-node after a backward step, so the set is
                    // never empty and we can skip the check
                    // let minimum_time = if current.time.first.set.is_empty() {
                    //     path.len()
                    // } else {
                    //     path.len() + 1
                    // };
                    let minimum_time = path.len() + 1;
                    // unsafe { COUNT += 1 };
                    if current.space.max_memory() >= predicates[minimum_time] {
                        path.pop();
                        if scheduler.skip_focus().is_err() {
                            break;
                        }
                    }
                }
            }
        }
        println!("result: {:?}", results.len());
        println!("count: {:?}", unsafe { COUNT });
    }

    #[allow(clippy::type_complexity)]
    fn input() -> (Vec<(usize, usize)>, Vec<Vec<(usize, Vec<usize>)>>, usize) {
        //         2
        //       /  \
        // 0 - 1     4
        //       \  /
        //         3
        //
        //    -------
        //  /         \
        // 0 --- 3 --- 4
        //  \
        //    -
        //      \
        // 2 --- 1
        let space = vec![(0, 1), (1, 2), (1, 3), (2, 4), (4, 3)];
        let time = vec![
            vec![(0, vec![]), (2, vec![])],
            vec![(3, vec![0]), (1, vec![0, 2])],
            vec![(4, vec![0, 3])],
        ];
        // let time =
        //     vec![vec![(0, vec![]), (1, vec![]), (2, vec![]), (3, vec![]), (4, vec![])]];

        // let space = [];
        // let time = vec![vec![(0, vec![])]];
        // let time = vec![];
        (space, time, 5)
    }
}
