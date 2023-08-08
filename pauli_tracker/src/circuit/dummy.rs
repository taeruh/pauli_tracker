use super::CliffordCircuit;

/// A dummy Clifford circuit that does nothing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DummyCircuit {}
impl CliffordCircuit for DummyCircuit {
    type Outcome = ();

    fn x(&mut self, _: usize) {}
    fn y(&mut self, _: usize) {}
    fn z(&mut self, _: usize) {}
    fn h(&mut self, _: usize) {}
    fn s(&mut self, _: usize) {}
    fn cx(&mut self, _: usize, _: usize) {}
    fn cz(&mut self, _: usize, _: usize) {}
    fn measure(&mut self, _: usize) {}
}
