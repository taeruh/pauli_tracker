use lib::{
    collection::{
        Init,
        NaiveVector,
    },
    pauli::{
        self,
        Pauli,
    },
    tracker::Tracker,
};
use pyo3::{
    PyResult,
    Python,
};

use crate::{
    impl_helper::{
        doc,
        links,
    },
    pauli::PauliDense,
    Module,
};

type LiveStorage = NaiveVector<pauli::PauliDense>;
impl_live!(
    LiveStorage,
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
    #[doc = doc::transform!()]
    ///
    /// Returns:
    ///     list[PauliDense]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_array(&self) -> Vec<PauliDense> {
        self.0
            .clone()
            .into_storage()
            .0
            .into_iter()
            .map(PauliDense)
            .collect()
    }

    #[doc = doc::transform!()]
    ///
    /// Returns:
    ///     list[int]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_array_recursive(&self) -> Vec<u8> {
        self.0
            .clone()
            .into_storage()
            .0
            .into_iter()
            .map(|p| p.tableau_encoding())
            .collect()
    }
}

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let module = Module::new(py, "vec", parent_module.path.clone())?;

    module.add_class::<Live>()?;

    parent_module.add_submodule(py, module)?;
    Ok(())
}
