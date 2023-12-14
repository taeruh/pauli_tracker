use std::collections::HashMap;

use bitvec::vec::BitVec;
use lib::{
    collection::Init,
    pauli::{
        self,
    },
    tracker::{
        frames,
        Tracker,
    },
};

use super::Map;

type PauliStack = pauli::PauliStack<BitVec>;
type Storage = Map<PauliStack>;
type LibFrames = frames::Frames<Storage>;

#[pyo3::pyclass]
pub struct Frames(LibFrames);

#[pyo3::pymethods]
impl Frames {
    #[new]
    fn init(len: usize) -> Self {
        Self(LibFrames::init(len))
    }

    fn new_qubit(&mut self, bit: usize) {
        self.0.new_qubit(bit);
    }

    fn to_py_dict(&self) -> HashMap<usize, (Vec<usize>, Vec<usize>)> {
        self.0
            .clone()
            .into_storage()
            .into_iter()
            .map(|(b, p)| (b, (p.left.into_vec(), p.right.into_vec())))
            .collect()
    }
}

single_pass!(
    Frames, track_x, track_y, track_z, id, x, y, z, s, sdg, sz, szdg, hxy, h, sh, hs,
    shs, sx, sxdg, hyz,
);
double_pass!(Frames, cz, swap, iswap, iswapdg,);
double_pass_named_bits!(
    Frames,
    (cx, control, target),
    (cy, control, target),
    (move_z_to_z, source, destination),
    (move_z_to_x, source, destination),
    (move_x_to_z, source, destination),
    (move_x_to_x, source, destination),
);
