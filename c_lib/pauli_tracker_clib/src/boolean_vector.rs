#![allow(non_camel_case_types)]

use bitvec::vec;
use pauli_tracker::boolean_vector::BooleanVector;

use crate::RawVec;

pub type Vec_b = Vec<bool>;
// specifying the default generics of BitVec and including bitvec in the list of crates
// parsed by cbindgen, doesn't really work (we get incomplete structs ...) -> instead of
// modifiying the output of cbindgen, we do some hacky cfg thing (we enable the cbindgen
// cfg in generate_bindings)
#[cfg(cbindgen)]
pub struct BitVec;
#[cfg(not(cbindgen))]
pub type BitVec = vec::BitVec<u64, bitvec::order::Lsb0>;

macro_rules! boilerplate {
    ($(($typ:ty, $pre:tt),)*) => {$(
        impl_api::basic!($typ, $pre);
        impl_api::boolean_vector!($typ, $pre);
    )*};
}

boilerplate!((Vec_b, vec_b_), (BitVec, bitvec_),);

pub type RawVec_b = RawVec<bool>;
pub type RawVec_u64 = RawVec<u64>;

impl_api::raw_vec!(Vec_b, vec_b_, RawVec_b);

/// Note that the `len`gth is not the number of bits, but the number of storage chunks.
#[no_mangle]
pub extern "C" fn bitvec_get_raw(x: &mut BitVec) -> RawVec_u64 {
    RawVec_u64 {
        data: x.as_mut_bitptr().pointer(),
        len: x.len(),
    }
}
