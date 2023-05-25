//! An intuitive description of a circuit consisting only of certain Clifford
//! gates and (unspecified) measurements.
//!
//! Currently, the implementation only captures the gates. It does **not** run any
//! simulations. It's main usage is rather storing the actions.

use std::ops::{
    Deref,
    DerefMut,
};

use super::CliffordCircuit;

/// A circuit description of a Clifford circuit with measurements.
// it is just a newtype wrapper around a Vec, so it makes sense to implement Deref and
// DerefMut since Vec is a smart pointer
#[derive(Debug, Default)]
pub struct SimpleCircuit {
    /// The circuit instructions
    pub gates: Vec<Gate>,
}

impl SimpleCircuit {
    /// Create a new empty [SimpleCircuit]
    pub fn new() -> Self {
        Self { gates: Vec::new() }
    }
}

impl Deref for SimpleCircuit {
    type Target = Vec<Gate>;
    fn deref(&self) -> &Self::Target {
        &self.gates
    }
}

impl DerefMut for SimpleCircuit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gates
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
/// A subset of the Clifford gates + (unspecified) measurements. Each operation stores
/// the qubit position it acts on.
pub enum Gate {
    /// Pauli X
    X(usize),
    /// Pauli Y
    Y(usize),
    /// Pauli Z
    Z(usize),
    /// Hadamard
    H(usize),
    /// Phase
    S(usize),
    /// Control X (Control Not)
    CX(
        /// Control
        usize,
        /// Target
        usize,
    ),
    /// Control Z
    CZ(usize, usize),
    /// Unspecified measurement
    Measure(usize),
}

impl CliffordCircuit for SimpleCircuit {
    #[inline]
    fn x(&mut self, bit: usize) {
        self.gates.push(Gate::X(bit));
    }
    #[inline]
    fn z(&mut self, bit: usize) {
        self.gates.push(Gate::Z(bit));
    }
    #[inline]
    fn y(&mut self, bit: usize) {
        self.gates.push(Gate::Y(bit));
    }
    #[inline]
    fn h(&mut self, bit: usize) {
        self.gates.push(Gate::H(bit));
    }
    #[inline]
    fn s(&mut self, bit: usize) {
        self.gates.push(Gate::S(bit));
    }
    #[inline]
    fn cx(&mut self, control: usize, target: usize) {
        self.gates.push(Gate::CX(control, target));
    }
    #[inline]
    fn cz(&mut self, bit_a: usize, bit_b: usize) {
        self.gates.push(Gate::CX(bit_a, bit_b));
    }
    #[inline]
    fn measure(&mut self, bit: usize) {
        self.gates.push(Gate::Measure(bit));
    }
}
