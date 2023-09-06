use super::CliffordCircuit;

/// A dummy Clifford circuit that does nothing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DummyCircuit {}
impl CliffordCircuit for DummyCircuit {
    type Outcome = ();
    impl_dummy_gates!();
    fn measure(&mut self, _: usize) {}
}
