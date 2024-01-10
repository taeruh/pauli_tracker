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
        crate::impl_helper::tracker::double_pass_named_bits!
            ($type, $(($name, bit_a, bit_b),)*);
    };
}

macro_rules! tracker_impl {
    ($type:ty) => {
        crate::impl_helper::tracker::single_pass!(
            $type, track_x, track_y, track_z, id, x, y, z, s, sdg, sz, szdg, hxy, h, sh,
            hs, shs, sx, sxdg, hyz,
        );
        crate::impl_helper::tracker::double_pass!($type, cz, swap, iswap, iswapdg,);
        crate::impl_helper::tracker::double_pass_named_bits!(
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
pub(crate) use single_pass;
pub(crate) use tracker_impl;
