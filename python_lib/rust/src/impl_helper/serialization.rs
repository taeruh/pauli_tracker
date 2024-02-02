macro_rules! serialization_format {
    () => {
        r"    serialization_format (str): The serialization format to use. The supported 
        formats are: serde_json_ and bincode_.
            
.. _bincode:
    https://github.com/bincode-org/bincode
.. _serde_json:
    https://github.com/serde-rs/json
        "
    }
}
macro_rules! serialization_format_string_compatible {
    () => {
        r"    serialization_format (str): The serialization format to use. The supported 
        formats are: serde_json_.

.. _serde_json:
    https://github.com/serde-rs/json
        "
    }
}

use core::fmt;

pub(crate) use serialization_format;
pub(crate) use serialization_format_string_compatible;

pub fn serialize_error<T: fmt::Display>(error: T) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(format!("failed to serialize: {error}"))
}

pub fn deserialize_error<T: fmt::Display>(error: T) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(format!("failed to deserialize: {error}"))
}

macro_rules! serde {
    ($type:ty) => {
        #[pyo3::pymethods]
        impl $type {
            /// Serialize the internal data structure into a file.
            ///
            /// Args:
            ///     file_path (str): The path to the file to write to.
            #[doc = crate::impl_helper::serialization::serialization_format!()]
            #[pyo3(signature = (file_path, serialization_format="serde_json"))]
            fn serialize(
                &self,
                file_path: &str,
                serialization_format: &str,
            ) -> pyo3::PyResult<()> {
                utils::serialization::serialize_to_file(
                    file_path,
                    &self.0,
                    serialization_format,
                )
                .map_err(crate::impl_helper::serialization::serialize_error)
            }

            /// Deserialize the internal data structure from a file.
            ///
            /// Args:
            ///     file_path (str): The path to the file to read from.
            #[doc = crate::impl_helper::serialization::serialization_format!()]
            #[staticmethod]
            #[pyo3(signature = (file_path, serialization_format="serde_json"))]
            fn deserialize(
                file_path: &str,
                serialization_format: &str,
            ) -> pyo3::PyResult<Self> {
                utils::serialization::deserialize_from_file(
                    file_path,
                    serialization_format,
                )
                .map(Self)
                .map_err(crate::impl_helper::serialization::deserialize_error)
            }

            /// Serialize the internal data structure into a string.
            ///
            /// Args:
            #[doc = crate::impl_helper::serialization::serialization_format_string_compatible!()]
            #[pyo3(signature = (serialization_format="serde_json"))]
            fn serialize_to_string(
                &self,
                serialization_format: &str,
            ) -> pyo3::PyResult<String> {
                utils::serialization::serialize_to_string(&self.0, serialization_format)
                    .map_err(crate::impl_helper::serialization::serialize_error)
            }

            /// Deserialize the internal data structure from a string.
            ///
            /// Args:
            ///     string (str): The string to read from.
            #[doc = crate::impl_helper::serialization::serialization_format_string_compatible!()]
            #[staticmethod]
            #[pyo3(signature = (string, serialization_format="serde_json"))]
            fn deserialize_from_string(
                string: &str,
                serialization_format: &str,
            ) -> pyo3::PyResult<Self> {
                utils::serialization::deserialize_from_string(
                    string,
                    serialization_format,
                )
                .map(Self)
                .map_err(crate::impl_helper::serialization::deserialize_error)
            }
        }
    };
}
pub(crate) use serde;
