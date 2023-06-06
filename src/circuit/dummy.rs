use super::CliffordCircuit;

/// A dummy Clifford circuit that does nothing.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct DummyCircuit {}
impl CliffordCircuit for DummyCircuit {
    type Outcome = ();

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
    fn measure(&mut self, _: usize) {}
}
