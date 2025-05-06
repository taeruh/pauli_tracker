use std::{collections::HashMap, hash::BuildHasherDefault, mem};

use lib::{
    collection::{self, Init},
    pauli::{self, Pauli},
    tracker::{Tracker, live},
};
use pyo3::{PyResult, Python, types::PyModuleMethods};
use rustc_hash::FxHasher;

use crate::{Module, impl_helper::links, pauli::PauliDense};

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
    #[doc = crate::transform!()]
    ///
    /// Returns:
    ///     dict[int, PauliDense]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_dict(&self) -> HashMap<usize, PauliDense> {
        into_py_dict(self.0.clone())
    }

    #[doc = crate::take_transform!()]
    ///
    /// Returns:
    ///     dict[int, PauliDense]:
    fn take_into_py_dict(&mut self) -> HashMap<usize, PauliDense> {
        into_py_dict(mem::take(&mut self.0))
    }

    #[doc = crate::transform!()]
    ///
    /// Returns:
    ///     dict[int, int]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_dict_recursive(&self) -> HashMap<usize, u8> {
        into_py_dict_recursive(self.0.clone())
    }

    #[doc = crate::take_transform!()]
    ///
    /// Returns:
    ///     dict[int, int]:
    fn take_into_py_dict_recursive(&mut self) -> HashMap<usize, u8> {
        into_py_dict_recursive(mem::take(&mut self.0))
    }
}

fn into_py_dict(live: live::Live<Storage>) -> HashMap<usize, PauliDense> {
    live.into_storage()
        .into_iter()
        .map(|(b, p)| (b, PauliDense(p)))
        .collect()
}

fn into_py_dict_recursive(live: live::Live<Storage>) -> HashMap<usize, u8> {
    live.into_storage()
        .into_iter()
        .map(|(b, p)| (b, p.tableau_encoding()))
        .collect()
}

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let module = Module::new(py, "map", parent_module.path.clone())?;
    module.pymodule.add_class::<Live>()?;
    parent_module.add_submodule(py, module)?;
    Ok(())
}
