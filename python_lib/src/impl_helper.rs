// Tracker must be in scope for the macro to work.

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
        crate::impl_helper::double_pass_named_bits!
            ($type, $(($name, bit_a, bit_b),)*);
    };
}

macro_rules! impl_passes {
    ($type:ty) => {
        crate::impl_helper::single_pass!(
            $type, track_x, track_y, track_z, id, x, y, z, s, sdg, sz, szdg, hxy, h,
            sh, hs, shs, sx, sxdg, hyz,
        );
        crate::impl_helper::double_pass!($type, cz, swap, iswap, iswapdg,);
        crate::impl_helper::double_pass_named_bits!(
            $type,
            (cx, control, target),
            (cy, control, target),
            (move_z_to_z, source, destination),
            (move_z_to_x, source, destination),
            (move_x_to_z, source, destination),
            (move_x_to_x, source, destination),
        );
    };
}

pub(crate) use double_pass;
pub(crate) use double_pass_named_bits;
pub(crate) use impl_passes;
pub(crate) use single_pass;

pub mod links {
    macro_rules! live {
        () =>
            {
"https://docs.rs/pauli_tracker/latest/pauli_tracker/tracker/live/struct.Live.html"
            }
    }
    pub(crate) use live;

    macro_rules! frames {
        () =>
            {
"https://docs.rs/pauli_tracker/latest/pauli_tracker/tracker/frames/struct.Frames.html"
            }
    }
    pub(crate) use frames;

    macro_rules! map {
        () =>
            {
"https://docs.rs/pauli_tracker/latest/pauli_tracker/collection/type.Map.html"
            }
    }
    pub(crate) use map;

    macro_rules! naive_vector {
        () =>
            {
"https://docs.rs/pauli_tracker/latest/pauli_tracker/collection/struct.NaiveVector.html"
            }
    }
    pub(crate) use naive_vector;

    macro_rules! pauli_dense {
        () =>
            {
"https://docs.rs/pauli_tracker/latest/pauli_tracker/pauli/struct.PauliDense.html"
            }
    }
    pub(crate) use pauli_dense;

    macro_rules! pauli_stack {
        () =>
            {
"https://docs.rs/pauli_tracker/latest/pauli_tracker/pauli/struct.PauliStack.html"
            }
    }
    pub(crate) use pauli_stack;

    macro_rules! bit_vec {
        () => {
            "https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html"
        };
    }
    pub(crate) use bit_vec;
}

pub mod doc {
    macro_rules! transform {
        () =>
            {
r"Return and transform the internal Rust data representation into the according Python
representation.

This may be costly, since it copies the data."
            }
    }
    pub(crate) use transform;
}
