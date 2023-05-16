//! An intuitive, canonical description of a circuit consisting only of certain Clifford
//! gates and (unspecified) measurements.

use std::ops::{
    Deref,
    DerefMut,
};

/// A circuit consisting of Clifford gates and measurements.
// it is just a newtype wrapper around a Vec, so it makes sense to implement Deref and
// DerefMut since Vec is a smart pointer
#[derive(Debug, Default)]
pub struct Circuit {
    /// The circuit instructions
    pub gates: Vec<Gate>,
}

#[derive(Debug)]
/// A subset of the Clifford gates + (unspecified) measurements. Each operation stores
/// the qubit position it acts on.
pub enum Gate {
    /// Pauli X
    X(usize),
    /// Pauli Z
    Z(usize),
    /// Hadamard
    H(usize),
    /// Phase
    S(usize),
    /// Control Not
    Cnot(
        /// Control
        usize,
        /// Target
        usize,
    ),
    /// Unspecified measurement
    Measure(usize),
}

impl Circuit {
    /// Create a new empty [Circuit]
    pub fn new() -> Self {
        Self { gates: Vec::new() }
    }

    // convenience methods to build the circuit (could also be down directly since
    // DerefMut is implemented to point to self.gates)

    /// Push [Gate::X]\(`bit`\)
    pub fn x(&mut self, bit: usize) {
        self.gates.push(Gate::X(bit));
    }

    // ...
}

impl Deref for Circuit {
    type Target = Vec<Gate>;
    fn deref(&self) -> &Self::Target {
        &self.gates
    }
}

impl DerefMut for Circuit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gates
    }
}

// TODO finish implementation; maybe with some derive macros to reduce some boilerplate
#[cfg(any(feature = "dense-circuit", doc))]
#[cfg_attr(
    all(feature = "doc-build", not(feature = "not-nightly")),
    doc(cfg(feature = "dense-circuit"))
)]
pub mod dense;
