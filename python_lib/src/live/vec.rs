use lib::{
    collection::{
        Init,
        NaiveVector,
    },
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

use crate::{
    impl_helper::{
        doc,
        links,
    },
    Module,
};

type LiveStorage = NaiveVector<PauliDense>;
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
    fn to_py_array(&self) -> Vec<u8> {
        self.0
            .clone()
            .into()
            .0
            .into_iter()
            .map(|p| p.tableau_encoding())
            .collect()
    }
}

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let _ = parent_module;
    let module = Module::new(py, "vec", parent_module.path.clone())?;

    module.add_class::<Live>()?;

    parent_module.add_submodule(py, module)?;
    Ok(())
}
