#![allow(non_camel_case_types)]

use pauli_tracker::{
    boolean_vector::BooleanVector,
    collection::{Init, IterableBase},
    pauli::PauliStack,
    tracker::{frames::Frames, Tracker},
};

use crate::{
    boolean_vector::{BitVec, Vec_b},
    collection::{
        BufferedVector_psbv, BufferedVector_psvb, Map_psbvfx, Map_psvbfx,
        MappedVector_psbvfx, MappedVector_psvbfx, RawVec_psbv, RawVec_psvb,
    },
    pauli::{PauliStack_bv, PauliStack_vb, PauliTuple},
};

pub type Frames_hmpsvbfx = Frames<Map_psvbfx>;
pub type Frames_hmpsbvfx = Frames<Map_psbvfx>;
pub type Frames_bvpsvb = Frames<BufferedVector_psvb>;
pub type Frames_bvpsbv = Frames<BufferedVector_psbv>;
pub type Frames_mvpsvbfx = Frames<MappedVector_psvbfx>;
pub type Frames_mvpsbvfx = Frames<MappedVector_psbvfx>;

pub type Vec_psvb = Vec<PauliStack_vb>;
pub type Vec_psbv = Vec<PauliStack_bv>;

pub extern "C" fn show_frames(frames: &Frames_hmpsbvfx) {
    println!("{:?}", frames);
}

macro_rules! boilerplate {
    ($(($typ:ty, $pre:tt, $pauli:ty, $stack:ty, $storage:ty),)*) => {$(
        impl_api::basic!($typ, $pre);
        impl_api::init!($typ, $pre);
        impl_api::tracker!($typ, $pre, $stack, is_frames);
        impl_api::frames!($typ, $pre, $pauli, $storage);
    )*};
}

macro_rules! boilerplate_measure_vb {
    ($(($typ:ty, $pre:tt),)*) => {$(
        impl_api::frames_measure!($typ, $pre, Map_psvbfx, _hmfx);
        impl_api::frames_measure!($typ, $pre, BufferedVector_psvb, _bv);
        impl_api::frames_measure!($typ, $pre, MappedVector_psvbfx, _mvfx);
    )*};
}
macro_rules! boilerplate_measure_bv {
    ($(($typ:ty, $pre:tt),)*) => {$(
        impl_api::frames_measure!($typ, $pre, Map_psbvfx, _hmfx);
        impl_api::frames_measure!($typ, $pre, BufferedVector_psbv, _bv);
        impl_api::frames_measure!($typ, $pre, MappedVector_psbvfx, _mvfx);
    )*};
}

macro_rules! boilerplate_vecs {
    ($(($typ:ty, $pre:tt, $raw_typ:ty),)*) => {$(
        impl_api::basic!($typ, $pre);
        impl_api::raw_vec!($typ, $pre, $raw_typ);
    )*};
}

// actually, one should also include the storage abbreviation in the name, but since we
// always use Map_* as storage, I'm omitting it here (for now)
boilerplate!(
    (Frames_hmpsvbfx, frames_hmpsvbfx_, PauliTuple, PauliStack_vb, Map_psvbfx),
    (Frames_hmpsbvfx, frames_hmpsbvfx_, PauliTuple, PauliStack_bv, Map_psbvfx),
    (
        Frames_bvpsvb,
        frames_bvpsvb_,
        PauliTuple,
        PauliStack_vb,
        BufferedVector_psvb
    ),
    (
        Frames_bvpsbv,
        frames_bvpsbv_,
        PauliTuple,
        PauliStack_bv,
        BufferedVector_psbv
    ),
    (
        Frames_mvpsvbfx,
        frames_mvpsvb_,
        PauliTuple,
        PauliStack_vb,
        MappedVector_psvbfx
    ),
    (
        Frames_mvpsbvfx,
        frames_mvpsbv_,
        PauliTuple,
        PauliStack_bv,
        MappedVector_psbvfx
    ),
);

boilerplate_measure_vb!(
    (Frames_hmpsvbfx, frames_hmpsvbfx_),
    (Frames_bvpsvb, frames_bvpsvb_),
    (Frames_mvpsvbfx, frames_mvpsvb_),
);
boilerplate_measure_bv!(
    (Frames_hmpsbvfx, frames_hmpsbvfx_),
    (Frames_bvpsbv, frames_bvpsbv_),
    (Frames_mvpsbvfx, frames_mvpsbv_),
);

boilerplate_vecs!((Vec_psvb, vec_psvb_, RawVec_psvb), (Vec_psbv, vec_psbv_, RawVec_psbv),);

pub trait Remove {
    fn remove(&mut self, row: usize);
}

impl Remove for Vec_b {
    fn remove(&mut self, row: usize) {
        self.remove(row);
    }
}

impl Remove for BitVec {
    fn remove(&mut self, row: usize) {
        self.remove(row);
    }
}

/// drops the orginal Frames; the new returned one, must be dropped
pub fn remove_row<S, B>(frames: Frames<S>, row: usize) -> Frames<S>
where
    S: IterableBase<T = PauliStack<B>>,
    B: BooleanVector + Remove,
{
    let frames_num = frames.frames_num();
    let mut storage = frames.into_storage();
    for (_, stack) in storage.iter_pairs_mut() {
        stack.z.remove(row);
        stack.x.remove(row);
    }
    Frames::new_unchecked(storage, frames_num - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let mut frames = Frames_hmpsvbfx::init(3);
        frames.track_x(0);
        frames.track_y(1);
        frames.track_z(2);
        println!("{:?}", frames);
        let frames = remove_row(frames, 1);
        println!("{:?}", frames);
    }
}
