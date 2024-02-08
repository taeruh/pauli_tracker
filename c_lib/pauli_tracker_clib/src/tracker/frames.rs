#![allow(non_camel_case_types)]

use std::collections::HashMap;

use pauli_tracker::{
    collection::Init,
    pauli::PauliStack,
    tracker::{frames::Frames, Tracker},
};

use crate::{
    collection::{
        BufferedVector_psbv, BufferedVector_psvb, Map_psbvfx, Map_psvbfx,
        MappedVector_psbvfx, MappedVector_psvbfx,
    },
    pauli::{PauliStack_bv, PauliStack_vb},
};

pub type Frames_hmpsvbfx = Frames<Map_psvbfx>;
pub type Frames_hmpsbvfx = Frames<Map_psbvfx>;
pub type Frames_bvpsvb = Frames<BufferedVector_psvb>;
pub type Frames_bvpsbv = Frames<BufferedVector_psbv>;
pub type Frames_mvpsvbfx = Frames<MappedVector_psvbfx>;
pub type Frames_mvpsbvfx = Frames<MappedVector_psbvfx>;

#[no_mangle]
/// don't use this
pub extern "C" fn show_frames(frames: &Frames_hmpsbvfx) {
    println!(
        "{:?}",
        frames
            .as_storage()
            .into_iter()
            .map(|(k, v)| (
                k,
                PauliStack {
                    z: v.z.as_raw_slice(),
                    x: v.x.as_raw_slice(),
                }
            ))
            .collect::<HashMap<_, _>>()
    );
}

macro_rules! boilerplate {
    ($(($typ:ty, $pre:tt, $stack:ty, $storage:ty, $stack_transposed:ty),)*)
    => {$(
        impl_api::basic!($typ, $pre);
        impl_api::init!($typ, $pre);
        impl_api::tracker!($typ, $pre, $stack, is_frames);
        impl_api::frames!($typ, $pre, $storage, $stack_transposed);
        impl_api::storage_wrapper!($typ, $pre, $storage);
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

// actually, one should also include the storage abbreviation in the name, but since we
// always use Map_* as storage, I'm omitting it here (for now)
boilerplate!(
    (
        Frames_hmpsvbfx,
        frames_hmpsvbfx_,
        PauliStack_vb,
        Map_psvbfx,
        BufferedVector_psvb
    ),
    (
        Frames_hmpsbvfx,
        frames_hmpsbvfx_,
        PauliStack_bv,
        Map_psbvfx,
        BufferedVector_psbv
    ),
    (
        Frames_bvpsvb,
        frames_bvpsvb_,
        PauliStack_vb,
        BufferedVector_psvb,
        BufferedVector_psvb
    ),
    (
        Frames_bvpsbv,
        frames_bvpsbv_,
        PauliStack_bv,
        BufferedVector_psbv,
        BufferedVector_psbv
    ),
    (
        Frames_mvpsvbfx,
        frames_mvpsvb_,
        PauliStack_vb,
        MappedVector_psvbfx,
        BufferedVector_psvb
    ),
    (
        Frames_mvpsbvfx,
        frames_mvpsbv_,
        PauliStack_bv,
        MappedVector_psbvfx,
        BufferedVector_psbv
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
