use pyo3::{
    PyResult,
    Python,
};

use crate::Module;

// Tracker and Init must be in scope for the macro to work.
macro_rules! impl_frames {
    ($storage:ty, $gentype:expr) => {
        type LibFrames = lib::tracker::frames::Frames<$storage>;

        #[doc = $gentype]
        #[pyo3::pyclass(subclass)]
        pub struct Frames(pub LibFrames);

        #[pyo3::pymethods]
        impl Frames {
            #[new]
            #[pyo3(signature = (len=0))]
            fn __new__(len: usize) -> Self {
                Self(LibFrames::init(len))
            }

            /// Create a new Frames tracker.
            ///
            /// Args:
            ///     len (int): The number of qubits to track
            ///
            /// Returns:
            ///     Frames: The new Frames tracker
            #[pyo3(text_signature = "(self, len=0)")]
            fn __init__(&mut self, _len: usize) {}

            /// Create a new qubit in the tracker, returning the old Pauli stack if the
            /// qubit was already initialized.
            fn new_qubit(&mut self, bit: usize) -> Option<crate::pauli::PauliStack> {
                self.0.new_qubit(bit).map(crate::pauli::PauliStack)
            }

            /// Remove a qubit in the tracker, returning the according Pauli stack and
            /// erroring if the qubit was not initialized.
            fn measure(
                &mut self,
                bit: usize,
            ) -> pyo3::PyResult<crate::pauli::PauliStack> {
                match self.0.measure(bit) {
                    Ok(p) => Ok(crate::pauli::PauliStack(p)),
                    Err(b) => {
                        Err(pyo3::exceptions::PyValueError::new_err(format!("{b}")))
                    },
                }
            }

            /// Get the Pauli stack of a qubit in the tracker, returning None if the
            /// qubit was not initialized. Note that this clones the data.
            fn get(&self, bit: usize) -> Option<crate::pauli::PauliStack> {
                self.0.get(bit).map(|p| crate::pauli::PauliStack(p.clone()))
            }
        }

        crate::impl_helper::impl_passes!(Frames);
    };
}

pub mod map;
pub mod vec;

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let _ = parent_module;
    let module = Module::new(py, "frames", parent_module.path.clone())?;

    map::add_module(py, &module)?;
    vec::add_module(py, &module)?;

    parent_module.add_submodule(py, module)?;
    Ok(())
}
