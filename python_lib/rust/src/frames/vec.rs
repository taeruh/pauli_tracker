use std::mem;

use lib::{
    collection::{Init, NaiveVector},
    pauli,
    tracker::{Tracker, frames},
};
use pyo3::{PyResult, Python, types::PyModuleMethods};

use crate::{BitVec, Module, impl_helper::links, pauli::PauliStack};

type Storage = NaiveVector<pauli::PauliStack<BitVec>>;
impl_frames!(
    Storage,
    concat!(
        "`Frames <",
        links::frames!(),
        ">`_\\<`NaiveVector <",
        links::naive_vector!(),
        ">`_\\<`PauliStack <",
        links::pauli_stack!(),
        ">`_\\<`BitVec <",
        links::bit_vec!(),
        ">`_\\>\\>\\>."
    )
);

#[pyo3::pymethods]
impl Frames {
    #[doc = crate::transform!()]
    ///
    /// Returns:
    ///     list[PauliStack]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_array(&self) -> Vec<PauliStack> {
        into_py_array(self.0.clone())
    }

    #[doc = crate::take_transform!()]
    ///
    /// Returns:
    ///     list[PauliStack]:
    fn take_into_py_array(&mut self) -> Vec<PauliStack> {
        into_py_array(mem::take(&mut self.0))
    }

    #[doc = crate::transform!()]
    ///
    /// Returns: cf. :obj:`~pauli_tracker.pauli.PauliStack`
    ///     list[tuple[list[int], list[int]]]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_array_recursive(&self) -> Vec<(Vec<u64>, Vec<u64>)> {
        into_py_array_recursive(self.0.clone())
    }

    #[doc = crate::take_transform!()]
    ///
    /// Returns: cf. :obj:`~pauli_tracker.pauli.PauliStack`
    ///     list[tuple[list[int], list[int]]]:
    #[allow(clippy::wrong_self_convention)]
    fn take_into_py_array_recursive(&mut self) -> Vec<(Vec<u64>, Vec<u64>)> {
        into_py_array_recursive(mem::take(&mut self.0))
    }
}

fn into_py_array(frames: frames::Frames<Storage>) -> Vec<PauliStack> {
    frames.into_storage().0.into_iter().map(PauliStack).collect()
}

fn into_py_array_recursive(frames: frames::Frames<Storage>) -> Vec<(Vec<u64>, Vec<u64>)> {
    frames
        .into_storage()
        .0
        .into_iter()
        .map(|p| (p.z.into_vec(), p.x.into_vec()))
        .collect()
}

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let module = Module::new(py, "vec", parent_module.path.clone())?;
    module.pymodule.add_class::<Frames>()?;
    parent_module.add_submodule(py, module)?;
    Ok(())
}
