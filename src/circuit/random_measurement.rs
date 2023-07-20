use super::CliffordCircuit;

/// A circuit where the gates do nothing, but the measurements return random bools.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RandomMeasurementCircuit {}

impl CliffordCircuit for RandomMeasurementCircuit {
    type Outcome = bool;

    #[inline(always)]
    fn x(&mut self, _: usize) {}
    #[inline(always)]
    fn y(&mut self, _: usize) {}
    #[inline(always)]
    fn z(&mut self, _: usize) {}
    #[inline(always)]
    fn h(&mut self, _: usize) {}
    #[inline(always)]
    fn s(&mut self, _: usize) {}
    #[inline(always)]
    fn cx(&mut self, _: usize, _: usize) {}
    #[inline(always)]
    fn cz(&mut self, _: usize, _: usize) {}
    #[inline(always)]

    fn measure(&mut self, _: usize) -> bool {
        rand::random::<bool>()
        // true
    }
}
