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

macro_rules! serialization_format {
    () => {
        r"    serialization_format (str): The serialization format to use. The supported 
        format are: json and bincode_ (default configurations).
            
.. _bincode:
    https://github.com/bincode-org/bincode
        "
    }
}
pub(crate) use serialization_format;
pub(crate) fn serialization_not_supported<T>(
    serialization_format: &str,
) -> pyo3::PyResult<T> {
    Err(pyo3::exceptions::PyValueError::new_err(format!(
        "serialization format not supported: {}; the supported formats are: json and \
         bincode",
        serialization_format
    )))
}

macro_rules! tracker_impl {
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
pub(crate) use single_pass;
pub(crate) use tracker_impl;

macro_rules! serde {
    ($type:ty) => {
        #[pyo3::pymethods]
        impl $type {
            /// Serialize the internal data structure.
            ///
            /// Args:
            ///     file_path (str): The path to the file to write to.
            #[doc = crate::impl_helper::serialization_format!()]
            #[pyo3(signature = (file_path, serialization_format="json"))]
            fn serialize(
                &self,
                file_path: &str,
                serialization_format: &str,
            ) -> pyo3::PyResult<()> {
                fn failed_to_serialize<T>(e: T) -> pyo3::PyErr
                where
                    T: std::fmt::Debug,
                {
                    pyo3::exceptions::PyValueError::new_err(format!(
                        "failed to serialize: {e:?}"
                    ))
                }
                std::fs::write(
                    file_path,
                    if serialization_format == "json" {
                        serde_json::to_vec(&self.0).map_err(failed_to_serialize)?
                    } else if serialization_format == "bincode" {
                        bincode::serialize(&self.0).map_err(failed_to_serialize)?
                    } else {
                        return crate::impl_helper::serialization_not_supported(
                            serialization_format,
                        );
                    },
                )
                .map_err(|e| {
                    pyo3::exceptions::PyValueError::new_err(format!(
                        "failed to write to file: {e:?}"
                    ))
                })
            }

            /// Deerialize the internal data structure.
            ///
            /// Args:
            ///     file_path (str): The path to the file to read from.
            #[doc = crate::impl_helper::serialization_format!()]
            #[staticmethod]
            #[pyo3(signature = (file_path, serialization_format="json"))]
            fn deserialize(
                file_path: &str,
                serialization_format: &str,
            ) -> pyo3::PyResult<Self> {
                fn failed_to_deserialize<T>(e: T) -> pyo3::PyErr
                where
                    T: std::fmt::Debug,
                {
                    pyo3::exceptions::PyValueError::new_err(format!(
                        "failed to deserialize: {e:?}"
                    ))
                }
                let contents = std::fs::read(file_path).map_err(|e| {
                    pyo3::exceptions::PyValueError::new_err(format!(
                        "failed to read file: {e:?}"
                    ))
                })?;
                Ok(if serialization_format == "json" {
                    Self(
                        serde_json::from_slice(&contents)
                            .map_err(failed_to_deserialize)?,
                    )
                } else if serialization_format == "bincode" {
                    Self(
                        bincode::deserialize(&contents)
                            .map_err(failed_to_deserialize)?,
                    )
                } else {
                    return crate::impl_helper::serialization_not_supported(
                        serialization_format,
                    );
                })
            }
        }
    };
}
pub(crate) use serde;

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
representation."
            }
    }
    pub(crate) use transform;
}
