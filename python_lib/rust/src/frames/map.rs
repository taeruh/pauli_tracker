use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
};

use bitvec::vec::BitVec;
use lib::{
    collection,
    collection::Init,
    pauli,
    tracker::Tracker,
};
use pyo3::{
    PyResult,
    Python,
};
use rustc_hash::FxHasher;

use crate::{
    impl_helper::{
        doc,
        links,
    },
    pauli::PauliStack,
    Module,
};

type Map<T> = collection::Map<T, BuildHasherDefault<FxHasher>>;

type Storage = Map<pauli::PauliStack<BitVec>>;
impl_frames!(
    Storage,
    concat!(
        "`Frames <",
        links::frames!(),
        ">`_\\<`Map <",
        links::map!(),
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
    ///     dict[int, PauliStack]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_dict(&self) -> HashMap<usize, PauliStack> {
        self.0
            .clone()
            .into_storage()
            .into_iter()
            .map(|(b, p)| (b, PauliStack(p)))
            .collect()
    }

    #[doc = doc::transform!()]
    ///
    /// Returns:
    ///     dict[int, tuple[list[int], list[int]]]:
    #[allow(clippy::wrong_self_convention)]
    fn into_py_dict_recursive(&self) -> HashMap<usize, (Vec<usize>, Vec<usize>)> {
        self.0
            .clone()
            .into_storage()
            .into_iter()
            .map(|(b, p)| (b, (p.left.into_vec(), p.right.into_vec())))
            .collect()
    }
}

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let module = Module::new(py, "map", parent_module.path.clone())?;

    module.add_class::<Frames>()?;

    parent_module.add_submodule(py, module)?;
    Ok(())
}
