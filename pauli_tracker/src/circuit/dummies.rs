use super::CliffordCircuit;

macro_rules! single_dummy {
    ($($name:ident,)*) => {$(
        fn $name(&mut self, _: usize) {}
    )*};
}
macro_rules! double_dummy {
    ($($name:ident,)*) => {$(
        fn $name(&mut self, _: usize, _: usize) {}
    )*};
}
macro_rules! impl_dummy_gates {
    () => {
        single_dummy!(
            id, x, y, z, s, sdg, sz, szdg, hxy, h, sy, sydg, sh, hs, shs, sx, sxdg, hyz,
        );
        double_dummy!(cz, cx, cy, swap, iswap, iswapdg,);
    };
}

/// A dummy Clifford circuit that does nothing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DummyCircuit {}
impl CliffordCircuit for DummyCircuit {
    type Outcome = ();
    impl_dummy_gates!();
    fn measure(&mut self, _: usize) {}
}

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
