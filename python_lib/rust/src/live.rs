use pyo3::{
    PyResult,
    Python,
};

use crate::Module;

// Tracker and Init must be in scope for the macro to work.
macro_rules! impl_live {
    ($storage:ty, $gentype:expr) => {
        type LibLive = lib::tracker::live::Live<$storage>;

        #[doc = $gentype]
        #[pyo3::pyclass(subclass)]
        pub struct Live(pub LibLive);

        #[pyo3::pymethods]
        impl Live {
            #[new]
            #[pyo3(signature = (len=0))]
            fn __new__(len: usize) -> Self {
                Self(LibLive::init(len))
            }

            /// Create a new Live tracker.
            ///
            /// Args:
            ///     len (int): The number of qubits to track
            ///
            /// Returns:
            ///     Live:
            #[pyo3(text_signature = "(self, len=0)")]
            fn __init__(&mut self, _len: usize) {}

            /// Create a new qubit in the tracker, returning the old Pauli if the qubit
            /// was already initialized.
            fn new_qubit(&mut self, bit: usize) -> Option<crate::pauli::PauliDense> {
                self.0.new_qubit(bit).map(crate::pauli::PauliDense)
            }

            /// Remove a qubit in the tracker, returning the according Pauli and
            /// erroring if the qubit was not initialized.
            fn measure(
                &mut self,
                bit: usize,
            ) -> pyo3::PyResult<crate::pauli::PauliDense> {
                match self.0.measure(bit) {
                    Ok(p) => Ok(crate::pauli::PauliDense(p)),
                    Err(b) => {
                        Err(pyo3::exceptions::PyValueError::new_err(format!("{b}")))
                    },
                }
            }

            /// Get the Pauli of a qubit in the tracker, returning None if the qubit was
            /// not initialized.
            fn get(&self, bit: usize) -> Option<crate::pauli::PauliDense> {
                self.0.get(bit).map(|p| crate::pauli::PauliDense(p.clone()))
            }
        }

        crate::impl_helper::tracker_impl!(Live);
        crate::impl_helper::serde!(Live);
    };
}

mod map;
mod vec;

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let module = Module::new(py, "live", parent_module.path.clone())?;

    map::add_module(py, &module)?;
    vec::add_module(py, &module)?;

    parent_module.add_submodule(py, module)?;
    Ok(())
}
