use lib::pauli::{self, Pauli};
use pyo3::{exceptions::PyValueError, PyResult, Python};

use crate::{
    impl_helper::{doc, serialization},
    BitVec, Module,
};

/// `PauliDense
/// <https://docs.rs/pauli_tracker/latest/pauli_tracker/pauli/struct.PauliDense.html>`_
#[pyo3::pyclass(subclass)]
pub struct PauliDense(pub pauli::PauliDense);

#[pyo3::pymethods]
impl PauliDense {
    #[new]
    #[pyo3(signature = (storage=0))]
    fn __new__(storage: u8) -> PyResult<Self> {
        match pauli::PauliDense::try_from(storage) {
            Ok(p) => Ok(PauliDense(p)),
            Err(b) => Err(PyValueError::new_err(format!("{b}"))),
        }
    }

    /// Create a new PauliDense from a tableau encoding.
    ///
    /// Args:
    ///     storage (int): The tableau encoding
    ///
    /// Returns:
    ///     PauliDense:
    #[pyo3(text_signature = "(self, storage=0)")]
    fn __init__(&self, _storage: u8) {}

    fn tableau_encoding(&self) -> u8 {
        self.0.tableau_encoding()
    }
}

/// `PauliTuple
/// <https://docs.rs/pauli_tracker/latest/pauli_tracker/pauli/struct.PauliTuple.html>`_
#[pyo3::pyclass(subclass)]
pub struct PauliTuple(pub pauli::PauliTuple);

#[pyo3::pymethods]
impl PauliTuple {
    #[new]
    #[pyo3(signature = (z=false, x=false))]
    fn __new__(z: bool, x: bool) -> Self {
        PauliTuple(pauli::PauliTuple(z, x))
    }

    /// Create a new PauliTuple from a tableau encoding.
    ///
    /// Args:
    ///     z (bool): The Z component
    ///     x (bool): The X component
    ///
    /// Returns:
    ///     PauliTuple:
    #[pyo3(text_signature = "(self, z=False, x=False)")]
    fn __init__(&self, _x: bool, _z: bool) {}

    fn tableau_encoding(&self) -> u8 {
        self.0.tableau_encoding()
    }

    #[doc = doc::transform!()]
    ///
    /// Returns:
    ///     tuple[bool, bool]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_tuple(&self) -> (bool, bool) {
        (self.0.0, self.0.1)
    }
}

#[pyo3::pyclass(subclass)]
/// `PauliStack
/// <https://docs.rs/pauli_tracker/latest/pauli_tracker/pauli/struct.PauliStack.html>`_\<`BitVec
/// <https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html>`_\>.
///
/// The Pauli Z and X stacks are bitvectors where each chunk consists of 64 bits. In the
/// chunk the bits are ordered from least to most significant. You can use
/// :func:`~pauli_tracker.bitvector_to_boolvector` to convert the bitvector to a list of
/// booleans.
#[derive(Clone)]
pub struct PauliStack(pub pauli::PauliStack<BitVec>);

#[pyo3::pymethods]
impl PauliStack {
    /// **Not defined**
    fn __init__(&self) {}

    #[doc = doc::transform!()]
    ///
    /// Returns:
    ///     tuple[list[int], list[int]]: The Z (left tuple element) and X (right tuple
    ///     element) components
    #[allow(clippy::wrong_self_convention)]
    pub fn into_py_tuple(&self) -> (Vec<u64>, Vec<u64>) {
        (self.0.z.clone().into_vec(), self.0.x.clone().into_vec())
    }

    #[staticmethod]
    fn zeros(len: usize) -> Self {
        PauliStack(pauli::PauliStack::zeros(len))
    }

    pub fn xor_inplace(&mut self, other: &PauliStack) {
        self.0.xor_inplace(&other.0);
    }

    fn get(&self, idx: usize) -> Option<PauliTuple> {
        Some(PauliTuple(self.0.get(idx)?))
    }

    fn sum_up(&self, filter: Vec<bool>) -> PauliTuple {
        PauliTuple(self.0.sum_up(&filter))
    }
}

serialization::serde!(PauliStack);

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let module = Module::new(py, "pauli", parent_module.path.clone())?;
    module.pymodule.add_class::<PauliDense>()?;
    module.pymodule.add_class::<PauliTuple>()?;
    module.pymodule.add_class::<PauliStack>()?;
    parent_module.add_submodule(py, module)?;
    Ok(())
}
