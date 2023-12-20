use bitvec::vec::BitVec;
use lib::pauli::{
    self,
    Pauli,
};
use pyo3::{
    exceptions::PyValueError,
    PyResult,
    Python,
};

use crate::{
    impl_helper::doc,
    Module,
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
    fn __init__(&mut self, _storage: u8) {}

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
    #[pyo3(signature = (x=false, z=false))]
    fn __new__(x: bool, z: bool) -> Self {
        PauliTuple(pauli::PauliTuple(x, z))
    }

    /// Create a new PauliTuple from a tableau encoding.
    ///
    /// Args:
    ///     x (bool): The X component
    ///     z (bool): The Z component
    ///
    /// Returns:
    ///     PauliTuple:
    #[pyo3(text_signature = "(self, x=False, z=False)")]
    fn __init__(&mut self, _x: bool, _z: bool) {}

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

/// `PauliStack
/// <https://docs.rs/pauli_tracker/latest/pauli_tracker/pauli/struct.PauliStack.html>`_
#[pyo3::pyclass(subclass)]
pub struct PauliStack(pub pauli::PauliStack<BitVec>);

#[pyo3::pymethods]
impl PauliStack {
    #[doc = doc::transform!()]
    ///
    /// Returns:
    ///     tuple[list[int], list[int]]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_tuple(&self) -> (Vec<usize>, Vec<usize>) {
        (self.0.left.clone().into_vec(), self.0.right.clone().into_vec())
    }

    fn sum_up(&self, filter: Vec<bool>) -> PauliTuple {
        PauliTuple(self.0.sum_up(&filter))
    }
}

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let _ = parent_module;
    let module = Module::new(py, "pauli", parent_module.path.clone())?;

    module.add_class::<PauliDense>()?;
    module.add_class::<PauliTuple>()?;
    module.add_class::<PauliStack>()?;

    parent_module.add_submodule(py, module)?;
    Ok(())
}
