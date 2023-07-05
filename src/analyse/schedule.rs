use std::mem;

use self::{
    space::Graph,
    time::TimeGraph,
};

pub mod space;
pub mod time;

struct Scheduler {
    time: TimeGraph,
    space: Graph,
}

impl Scheduler {
    fn new(time: TimeGraph, space: Graph) -> Self {
        Self { time, space }
    }

    fn step(&mut self) -> Option<(Self, Vec<usize>)> {
        let (new_time, mess) = self.time.step()?;
        let new_space = self.space.step(&mess).unwrap();
        Some((Self { time: new_time, space: new_space }, mess))
    }

    fn sweep(self) {
        let mut stack = Vec::new();
        let mut this = self;
        let mut path = Vec::new();
        let mut num = 0;
        loop {
            match this.step() {
                Some((that, mess)) => {
                    path.push(mess);
                    stack.push(mem::replace(&mut this, that));
                }
                None => {
                    if this.time.first.set.is_empty() {
                        println!(
                            "{}; {}; {:?}",
                            path.len(),
                            this.space.max_memory,
                            path
                        );
                        num += 1;
                    }
                    this = match stack.pop() {
                        Some(old) => {
                            path.pop();
                            old
                        }
                        None => {
                            println!("\n{num}");
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;

    #[test]
    fn scheduler() {
        //         2
        //       /  \
        // 0 - 1     4
        //       \  /
        //         3
        let space = [(0, 1), (1, 2), (1, 3), (2, 4), (4, 3)];
        let time = vec![
            vec![(0, vec![]), (2, vec![])],
            vec![(3, vec![0]), (1, vec![0, 2])],
            vec![(4, vec![0, 3])],
        ];
        // let time =
        //     vec![vec![(0, vec![]), (2, vec![]), (3, vec![]), (1, vec![]), (4, vec![])]];
        let mut scheduler =
            Scheduler::new(TimeGraph::from(time), Graph::new(5, &space));
        scheduler.sweep();
    }
}
