use std::mem;

use lib::{
    collection::{Init, NaiveVector},
    pauli::{self, Pauli},
    tracker::{live, Tracker},
};
use pyo3::{PyResult, Python};

use crate::{impl_helper::links, pauli::PauliDense, Module};

type Storage = NaiveVector<pauli::PauliDense>;
impl_live!(
    Storage,
    concat!(
        "`Live <",
        links::live!(),
        ">`_\\<`NaiveVector <",
        links::naive_vector!(),
        ">`_\\<`PauliDense <",
        links::pauli_dense!(),
        ">`_\\>\\>."
    )
);

#[pyo3::pymethods]
impl Live {
    #[doc = crate::transform!()]
    ///
    /// Returns:
    ///     list[PauliDense]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_array(&self) -> Vec<PauliDense> {
        into_py_array(self.0.clone())
    }

    #[doc = crate::take_transform!()]
    ///
    /// Returns:
    ///     list[PauliDense]:
    fn take_into_py_array(&mut self) -> Vec<PauliDense> {
        into_py_array(mem::take(&mut self.0))
    }

    #[doc = crate::transform!()]
    ///
    /// Returns:
    ///     list[int]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_array_recursive(&self) -> Vec<u8> {
        into_py_array_recursive(self.0.clone())
    }

    #[doc = crate::take_transform!()]
    ///
    /// Returns:
    ///     list[int]:
    fn take_into_py_array_recursive(&mut self) -> Vec<u8> {
        into_py_array_recursive(mem::take(&mut self.0))
    }
}

fn into_py_array(live: live::Live<Storage>) -> Vec<PauliDense> {
    live.into_storage().0.into_iter().map(PauliDense).collect()
}

fn into_py_array_recursive(live: live::Live<Storage>) -> Vec<u8> {
    live.into_storage()
        .0
        .into_iter()
        .map(|p| p.tableau_encoding())
        .collect()
}

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let module = Module::new(py, "vec", parent_module.path.clone())?;
    module.pymodule.add_class::<Live>()?;
    parent_module.add_submodule(py, module)?;
    Ok(())
}
