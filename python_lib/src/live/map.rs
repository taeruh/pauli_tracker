use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
};

use lib::{
    collection,
    collection::Init,
    pauli::{
        Pauli,
        PauliDense,
    },
    tracker::Tracker,
};
use pyo3::{
    PyResult,
    Python,
};
use rustc_hash::FxHasher;

use crate::{
    impl_helper::links,
    Module,
};

type Map<T> = collection::Map<T, BuildHasherDefault<FxHasher>>;

type Storage = Map<PauliDense>;
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
    fn to_py_dict(&self) -> HashMap<usize, u8> {
        self.0
            .clone()
            .into()
            .into_iter()
            .map(|(k, v)| (k, v.tableau_encoding()))
            .collect()
    }
}

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let _ = parent_module;
    let module = Module::new(py, "map", parent_module.path.clone())?;

    module.add_class::<Live>()?;

    parent_module.add_submodule(py, module)?;
    Ok(())
}
