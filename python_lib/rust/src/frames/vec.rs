use lib::{
    collection::{
        Init,
        NaiveVector,
    },
    pauli,
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
    pauli::PauliStack,
    BitVec,
    Module,
};

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
    #[doc = doc::transform!()]
    ///
    /// Returns:
    ///     list[PauliStack]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_array(&self) -> Vec<PauliStack> {
        self.0.clone().into_storage().0.into_iter().map(PauliStack).collect()
    }

    #[doc = doc::transform!()]
    ///
    /// Returns: cf. :obj:`~pauli_tracker.pauli.PauliStack`
    ///     list[tuple[list[int], list[int]]]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_array_recursive(&self) -> Vec<(Vec<u64>, Vec<u64>)> {
        self.0
            .clone()
            .into_storage()
            .0
            .into_iter()
            .map(|p| (p.z.into_vec(), p.x.into_vec()))
            .collect()
    }
}
pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let module = Module::new(py, "vec", parent_module.path.clone())?;
    module.add_class::<Frames>()?;
    parent_module.add_submodule(py, module)?;
    Ok(())
}
