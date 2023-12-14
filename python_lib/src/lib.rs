use std::hash::BuildHasherDefault;

use lib::collection;
use pyo3::{
    types::PyModule,
    PyResult,
    Python,
};
use rustc_hash::FxHasher;

type Map<T> = collection::Map<T, BuildHasherDefault<FxHasher>>;

// need to put the whole impl block into the macro because macros are expanded from
// outside to inside (we need the functions to be expanded before the pymethods macro
// kicks in)
macro_rules! single_pass {
    ($type:ty, $($name:ident,)*) => {
        #[pyo3::pymethods]
        impl $type {
            $(
                fn $name(&mut self, bit: usize) {
                    self.0.$name(bit);
                }
            )*
        }
    };
}
macro_rules! double_pass_named_bits {
    ($type:ty, $(($name:ident, $bit_a:ident, $bit_b:ident),)*) => {
        #[pyo3::pymethods]
        impl $type {
            $(
                fn $name(&mut self, $bit_a: usize, $bit_b: usize) {
                    self.0.$name($bit_a, $bit_b);
                }
            )*
        }
    };
}
macro_rules! double_pass {
    ($type:ty, $($name:ident,)*) => {
        double_pass_named_bits! ($type, $(($name, bit_a, bit_b),)*);
    };
}

mod frames;
use frames::Frames;
mod live;
use live::Live;

/// The Pauli Tracker
#[pyo3::pymodule]
fn pauli_tracker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Live>()?;
    m.add_class::<Frames>()?;
    Ok(())
}
