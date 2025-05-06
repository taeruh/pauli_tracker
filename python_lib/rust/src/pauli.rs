use std::mem;

use lib::pauli::{self, Pauli};
use pyo3::{PyResult, Python, exceptions::PyValueError, types::PyModuleMethods};

use crate::{BitVec, Module, impl_helper::serialization};

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

    /// Get the according Python tuple.
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
/// chunk the bits are ordered from least to most significant.
#[derive(Clone)]
pub struct PauliStack(pub pauli::PauliStack<BitVec>);

#[pyo3::pymethods]
impl PauliStack {
    /// **Not defined**
    fn __init__(&self) {}

    #[doc = crate::transform!()]
    /// You can use :func:`~pauli_tracker.bitvector_to_boolvector` to convert the
    /// bitvector to a list of booleans, or directly use :func:`into_py_bool_tuple`.
    ///
    /// Returns:
    ///     tuple[list[int], list[int]]: The Z (left tuple element) and X (right tuple
    ///     element) components
    #[allow(clippy::wrong_self_convention)]
    pub fn into_py_tuple(&self) -> (Vec<u64>, Vec<u64>) {
        stack_into_py_tuple(self.0.clone())
    }

    #[doc = crate::take_transform!()]
    /// You can use :func:`~pauli_tracker.bitvector_to_boolvector` to convert the
    /// bitvector to a list of booleans, or directly use :func:`take_into_py_bool_tuple`.
    ///
    /// Returns:
    ///     tuple[list[int], list[int]]: The Z (left tuple element) and X (right tuple
    ///     element) components
    fn take_into_py_tuple(&mut self) -> (Vec<u64>, Vec<u64>) {
        stack_into_py_tuple(mem::take(&mut self.0))
    }

    /// Transform the internal Rust data structure into the according Python tuple of
    /// lists of booleans.  If you do this mutiple times consider using the according
    /// `take_` method to avoid an additional clone, however, be aware that the internal
    /// data is replaced with its default value.
    ///
    /// Returns:
    ///     tuple[list[bool], list[bool]]: The Z (left tuple element) and X (right tuple
    ///     element) components
    #[allow(clippy::wrong_self_convention)]
    pub fn into_py_bool_tuple(&self) -> (Vec<bool>, Vec<bool>) {
        stack_into_py_bool_tuple(self.0.clone())
    }

    /// Transform the internal Rust data structure into the according Python tuple of
    /// list of booleans, replacing the internal data with its default value.
    ///
    /// Returns:
    ///     tuple[list[bool], list[bool]]: The Z (left tuple element) and X (right tuple
    ///     element) components
    fn take_into_py_bool_tuple(&mut self) -> (Vec<bool>, Vec<bool>) {
        stack_into_py_bool_tuple(mem::take(&mut self.0))
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

fn stack_into_py_tuple(stack: pauli::PauliStack<BitVec>) -> (Vec<u64>, Vec<u64>) {
    (stack.z.into_vec(), stack.x.into_vec())
}

fn stack_into_py_bool_tuple(stack: pauli::PauliStack<BitVec>) -> (Vec<bool>, Vec<bool>) {
    (stack.z.into_iter().collect(), stack.x.into_iter().collect())
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
