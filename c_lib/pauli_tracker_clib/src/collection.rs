#![allow(non_camel_case_types)]

use std::hash::BuildHasherDefault;

use pauli_tracker::collection::{Base, BufferedVector, Init, Map, MappedVector};
use rustc_hash::FxHasher;

use crate::{
    RawVec,
    pauli::{PauliEnum, PauliStack_bv, PauliStack_vb, PauliTuple},
};

pub type Map_psvbfx = Map<PauliStack_vb, BuildHasherDefault<FxHasher>>;
pub type Map_psbvfx = Map<PauliStack_bv, BuildHasherDefault<FxHasher>>;
pub type Map_pefx = Map<PauliEnum, BuildHasherDefault<FxHasher>>;
pub type Map_ptfx = Map<PauliTuple, BuildHasherDefault<FxHasher>>;

pub type MappedVector_psvbfx = MappedVector<PauliStack_vb, BuildHasherDefault<FxHasher>>;
pub type MappedVector_psbvfx = MappedVector<PauliStack_bv, BuildHasherDefault<FxHasher>>;
pub type MappedVector_pefx = MappedVector<PauliEnum, BuildHasherDefault<FxHasher>>;
pub type MappedVector_ptfx = MappedVector<PauliTuple, BuildHasherDefault<FxHasher>>;

pub type BufferedVector_psvb = BufferedVector<PauliStack_vb>;
pub type BufferedVector_psbv = BufferedVector<PauliStack_bv>;
pub type BufferedVector_pe = BufferedVector<PauliEnum>;
pub type BufferedVector_pt = BufferedVector<PauliTuple>;

macro_rules! boilerplate {
    ($(($typ:ty, $pre:tt, $etyp:ty),)*) => {$(
        impl_api::basic!($typ, $pre);
        impl_api::init!($typ, $pre);
        impl_api::base!($typ, $pre, $etyp);
    )*};
}

boilerplate!(
    (Map_psvbfx, map_psvbfx_, PauliStack_vb),
    (Map_psbvfx, map_psbvfx_, PauliStack_bv),
    (Map_pefx, map_pefx_, PauliEnum),
    (Map_ptfx, map_ptfx_, PauliTuple),
    (MappedVector_psvbfx, mapped_vector_psvbfx_, PauliStack_vb),
    (MappedVector_psbvfx, mapped_vector_psbvfx_, PauliStack_bv),
    (MappedVector_pefx, mapped_vector_pefx_, PauliEnum),
    (MappedVector_ptfx, mapped_vector_ptfx_, PauliTuple),
    (BufferedVector_psvb, buffered_vector_psvb_, PauliStack_vb),
    (BufferedVector_psbv, buffered_vector_psbv_, PauliStack_bv),
    (BufferedVector_pe, buffered_vector_pe_, PauliEnum),
    (BufferedVector_pt, buffered_vector_pt_, PauliTuple),
);

pub type RawVec_psvb = RawVec<PauliStack_vb>;
pub type RawVec_psbv = RawVec<PauliStack_bv>;
pub type RawVec_pe = RawVec<PauliEnum>;
pub type RawVec_pt = RawVec<PauliTuple>;

impl_api::raw_vec_newtyped!(BufferedVector_psvb, buffered_vector_psvb_, RawVec_psvb);
impl_api::raw_vec_newtyped!(BufferedVector_psbv, buffered_vector_psbv_, RawVec_psbv);
impl_api::raw_vec_newtyped!(BufferedVector_pe, buffered_vector_pe_, RawVec_pe);
impl_api::raw_vec_newtyped!(BufferedVector_pt, buffered_vector_pt_, RawVec_pt);
