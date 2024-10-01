#![allow(non_camel_case_types)]

use pauli_tracker::pauli::{Pauli, PauliStack};

mod enumlike;
mod tuple;

pub use enumlike::PauliEnum;
pub use tuple::PauliTuple;

use crate::boolean_vector::{BitVec, Vec_b};

impl_api::pauli!(PauliEnum, pauli_enum_);
impl_api::pauli!(PauliTuple, pauli_tuple_);

pub type PauliStack_vb = PauliStack<Vec_b>;
pub type PauliStack_bv = PauliStack<BitVec>;

macro_rules! boilerplate_stack {
    ($(($typ:ty, $pre:tt, $stack:ty),)*) => {$(
        impl_api::basic!($typ, $pre);
        impl_api::pauli_stack!($typ, $pre, $stack);
    )*};
}

boilerplate_stack! {
    (PauliStack_vb, pauli_stack_vb_, Vec_b),
    (PauliStack_bv, pauli_stack_bv_, BitVec),
}
