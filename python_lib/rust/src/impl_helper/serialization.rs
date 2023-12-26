macro_rules! serialization_format {
    () => {
        r"    serialization_format (str): The serialization format to use. The supported 
        format are: serde_json_ and bincode_ (default configurations).
            
.. _bincode:
    https://github.com/bincode-org/bincode
.. _serde_json:
    https://github.com/serde-rs/json
        "
    }
}
use core::fmt;

pub(crate) use serialization_format;

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
            /// Serialize the internal data structure.
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
                let format =
                    utils::serialization::Dynamic::try_from(serialization_format)
                        .map_err(crate::impl_helper::serialization::serialize_error)?;
                format
                    .write_file(file_path, &self.0)
                    .map_err(crate::impl_helper::serialization::serialize_error)
            }

            /// Deserialize the internal data structure.
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
                let format =
                    utils::serialization::Dynamic::try_from(serialization_format)
                        .map_err(
                            crate::impl_helper::serialization::deserialize_error,
                        )?;
                format
                    .read_file(file_path)
                    .map(Self)
                    .map_err(crate::impl_helper::serialization::deserialize_error)
            }
        }
    };
}
pub(crate) use serde;
