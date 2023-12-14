use std::collections::HashMap;

use lib::{
    collection::Init,
    pauli::PauliDense,
    tracker::{
        live,
        Tracker,
    },
};
use pyo3::{
    exceptions::PyValueError,
    PyResult,
};

use super::Map;

type Storage = Map<PauliDense>;
type LibLive = live::Live<Storage>;

#[pyo3::pyclass]
pub struct Live(LibLive);

#[pyo3::pymethods]
impl Live {
    #[new]
    fn init(len: usize) -> Self {
        Self(LibLive::init(len))
    }

    fn new_qubit(&mut self, bit: usize) -> Option<u8> {
        self.0.new_qubit(bit).map(|p| p.into())
    }

    fn to_py_dict(&self) -> HashMap<usize, u8> {
        self.0
            .clone()
            .into()
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect()
    }

    fn measure(&mut self, bit: usize) -> PyResult<u8> {
        match self.0.measure(bit) {
            Ok(p) => Ok(p.into()),
            Err(b) => Err(PyValueError::new_err(format!("{b}"))),
        }
    }

    fn get(&self, bit: usize) -> Option<u8> {
        self.0.get(bit).map(|p| p.storage())
    }
}

single_pass!(
    Live, track_x, track_y, track_z, id, x, y, z, s, sdg, sz, szdg, hxy, h, sh, hs,
    shs, sx, sxdg, hyz,
);
double_pass!(Live, cz, swap, iswap, iswapdg,);
double_pass_named_bits!(
    Live,
    (cx, control, target),
    (cy, control, target),
    (move_z_to_z, source, destination),
    (move_z_to_x, source, destination),
    (move_x_to_z, source, destination),
    (move_x_to_x, source, destination),
);
