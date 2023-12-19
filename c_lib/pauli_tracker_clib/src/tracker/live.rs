#![allow(non_camel_case_types)]

use pauli_tracker::{
    collection::Init,
    tracker::{
        live,
        Tracker,
    },
};

use crate::{
    collection::{
        BufferedVector_pe,
        BufferedVector_pt,
        Map_pefx,
    },
    pauli::{
        PauliEnum,
        PauliTuple,
    },
};

pub type Live_hmpefx = live::Live<Map_pefx>;
pub type Live_bvpe = live::Live<BufferedVector_pe>;
pub type Live_bvpt = live::Live<BufferedVector_pt>;

macro_rules! boilerplate {
    ($(($typ:ty, $pre:tt, $stack:ty),)*) => {$(
        impl_api::basic!($typ, $pre);
        impl_api::init!($typ, $pre);
        impl_api::tracker!($typ, $pre, $stack, foo);
    )*};
}

boilerplate!(
    (Live_hmpefx, live_hmpefx_, PauliEnum),
    (Live_bvpe, live_bvpe_, PauliEnum),
    (Live_bvpt, live_bvpt_, PauliTuple),
);
