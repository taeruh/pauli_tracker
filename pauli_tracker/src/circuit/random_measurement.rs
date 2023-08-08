use super::CliffordCircuit;

/// A circuit where the gates do nothing, but the measurements return random bools.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RandomMeasurementCircuit {}

impl CliffordCircuit for RandomMeasurementCircuit {
    type Outcome = bool;

    fn x(&mut self, _: usize) {}
    fn y(&mut self, _: usize) {}
    fn z(&mut self, _: usize) {}
    fn h(&mut self, _: usize) {}
    fn s(&mut self, _: usize) {}
    fn cx(&mut self, _: usize, _: usize) {}
    fn cz(&mut self, _: usize, _: usize) {}

    fn measure(&mut self, _: usize) -> bool {
        rand::random::<bool>()
        // true
    }
}
