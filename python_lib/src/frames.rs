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
        pub struct Frames(LibFrames);

        #[pyo3::pymethods]
        impl Frames {
            #[new]
            fn init(len: usize) -> Self {
                Self(LibFrames::init(len))
            }

            /// Create a new qubit in the tracker, returning the old Pauli stack if the
            /// qubit was already initialized.
            fn new_qubit(&mut self, bit: usize) -> Option<(Vec<usize>, Vec<usize>)> {
                self.0
                    .new_qubit(bit)
                    .map(|p| (p.left.into_vec(), p.right.into_vec()))
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
