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
}
