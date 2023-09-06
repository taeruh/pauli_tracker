use super::CliffordCircuit;

/// A circuit where the gates do nothing, but the measurements return random bools.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RandomMeasurementCircuit {}

impl CliffordCircuit for RandomMeasurementCircuit {
    type Outcome = bool;
    impl_dummy_gates!();
    fn measure(&mut self, _: usize) -> bool {
        rand::random::<bool>()
    }
}
