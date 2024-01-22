use std::{collections::HashMap, hash::BuildHasherDefault};

use lib::{
    collection,
    collection::Init,
    pauli::{self, Pauli},
    tracker::Tracker,
};
use pyo3::{PyResult, Python};
use rustc_hash::FxHasher;

use crate::{impl_helper::links, pauli::PauliDense, Module};

type Map<T> = collection::Map<T, BuildHasherDefault<FxHasher>>;

type Storage = Map<pauli::PauliDense>;
impl_live!(
    Storage,
    concat!(
        "`Live <",
        links::live!(),
        ">`_\\<`Map <",
        links::map!(),
        ">`_\\<`PauliDense <",
        links::pauli_dense!(),
        ">`_\\>\\>."
    )
);

#[pyo3::pymethods]
impl Live {
    #[doc = crate::impl_helper::doc::transform!()]
    ///
    /// Returns:
    ///     dict[int, PauliDense]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_dict(&self) -> HashMap<usize, PauliDense> {
        self.0
            .clone()
            .into_storage()
            .into_iter()
            .map(|(b, p)| (b, PauliDense(p)))
            .collect()
    }

    #[doc = crate::impl_helper::doc::transform!()]
    ///
    /// Returns:
    ///     dict[int, int]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_dict_recursive(&self) -> HashMap<usize, u8> {
        self.0
            .clone()
            .into_storage()
            .into_iter()
            .map(|(b, p)| (b, p.tableau_encoding()))
            .collect()
    }
}

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let module = Module::new(py, "map", parent_module.path.clone())?;
    module.add_class::<Live>()?;
    parent_module.add_submodule(py, module)?;
    Ok(())
}
