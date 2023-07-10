use std::mem;

use self::{
    space::{
        Graph,
        Sweep,
    },
    sweep::FocusNext,
    time::TimeGraph,
};

pub mod space;
pub mod sweep;
pub mod time;

struct Scheduler {
    time: TimeGraph,
    space: Graph,
}

impl Scheduler {
    fn new(time: TimeGraph, space: Graph) -> Self {
        Self { time, space }
    }

    //
}

// just for seeing whether it works as expected while developing
static mut COUNT: usize = 0;

impl FocusNext for Scheduler {
    type Outcome = Vec<usize>;
    type EndOutcome = Option<usize>;

    fn step(&mut self) -> Option<(Self, Self::Outcome)>
    where
        Self: Sized,
    {
        let (new_time, mess) = self.time.step()?;
        unsafe { COUNT += 1 };
        let new_space = self.space.step(&mess).unwrap();
        Some((Self { time: new_time, space: new_space }, mess))
    }

    fn check_end(&self) -> Self::EndOutcome {
        self.time.known.set.is_empty().then_some(self.space.max_memory)
    }
}

sweep::impl_into_iterator!(Scheduler);

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;
    use crate::analyse::schedule::sweep::Step;

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

        let mut scheduler =
            Scheduler::new(TimeGraph::from(time), Graph::new(num, &space));

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

        let mut scheduler =
            Scheduler::new(TimeGraph::from(time), Graph::new(space.len(), &space))
                .into_iter();

        while let Some(step) = scheduler.next() {
            match step {
                Step::Forward(mess) => {
                    let current = scheduler.current();
                    let minimum_time = if current.time.known.set.is_empty() {
                        path.len() + 1
                    } else {
                        path.len() + 2
                    };
                    // unsafe { COUNT += 1 };
                    if current.space.max_memory >= predicates[minimum_time] {
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
                    if current.space.max_memory >= predicates[minimum_time] {
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
