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
    Module,
};

type Map<T> = collection::Map<T, BuildHasherDefault<FxHasher>>;

type PauliStack = pauli::PauliStack<BitVec>;
type Storage = Map<PauliStack>;
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
    fn to_py_dict(&self) -> HashMap<usize, (Vec<usize>, Vec<usize>)> {
        self.0
            .clone()
            .into_storage()
            .into_iter()
            .map(|(b, p)| (b, (p.left.into_vec(), p.right.into_vec())))
            .collect()
    }
}

pub fn add_module(py: Python<'_>, parent_module: &Module) -> PyResult<()> {
    let _ = parent_module;
    let module = Module::new(py, "map", parent_module.path.clone())?;

    module.add_class::<Frames>()?;

    parent_module.add_submodule(py, module)?;
    Ok(())
}
