use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[macro_export]
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

#[macro_export]
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

pub fn serialize_error<T: fmt::Display>(error: T) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(format!("failed to serialize: {error}"))
}

pub fn deserialize_error<T: fmt::Display>(error: T) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(format!("failed to deserialize: {error}"))
}

#[macro_export]
macro_rules! inner_type {
    (newtype, $self:expr) => {
        $self.0
    };
    (plain, $self:expr) => {
        $self
    };
}
#[macro_export]
macro_rules! map_type {
    (newtype, $to_wrap:expr) => {
        Result::map($to_wrap, Self)
    };
    (plain, $to_wrap:expr) => {
        $to_wrap
    };
}

// most of the times, we are dealing with a newtype, but sometimes it is a plain type
// (don't remove this functionality here (if it is not needed here), because
// mbqc_scheduling relies on it (at the moment))

#[macro_export]
macro_rules! serde {
    ($type:ty) => {
        $crate::serde!($type, newtype);
    };
    ($type:ty, $opaque:tt) => {
        #[pyo3::pymethods]
        impl $type {
            /// Serialize the internal data structure into a file.
            ///
            /// Args:
            ///     file_path (str): The path to the file to write to.
            #[doc = $crate::serialization_format!()]
            #[pyo3(signature = (file_path, serialization_format="serde_json"))]
            fn serialize(
                &self,
                file_path: &str,
                serialization_format: &str,
            ) -> pyo3::PyResult<()> {
                $crate::impl_helper::serialization::serialize_to_file(
                    file_path,
                    &$crate::inner_type!($opaque, self),
                    serialization_format,
                )
                .map_err($crate::impl_helper::serialization::serialize_error)
            }

            /// Deserialize the internal data structure from a file.
            ///
            /// Args:
            ///     file_path (str): The path to the file to read from.
            #[doc = $crate::serialization_format!()]
            #[staticmethod]
            #[pyo3(signature = (file_path, serialization_format="serde_json"))]
            fn deserialize(
                file_path: &str,
                serialization_format: &str,
            ) -> pyo3::PyResult<Self> {
                $crate::map_type!(
                    $opaque,
                    $crate::impl_helper::serialization::deserialize_from_file(
                        file_path,
                        serialization_format,
                    )
                )
                .map_err($crate::impl_helper::serialization::deserialize_error)
            }

            /// Serialize the internal data structure into a string.
            ///
            /// Args:
            #[doc = $crate::serialization_format_string_compatible!()]
            #[pyo3(signature = (serialization_format="serde_json"))]
            fn serialize_to_string(
                &self,
                serialization_format: &str,
            ) -> pyo3::PyResult<String> {
                $crate::impl_helper::serialization::serialize_to_string(
                    &$crate::inner_type!($opaque, self),
                    serialization_format,
                )
                .map_err($crate::impl_helper::serialization::serialize_error)
            }

            /// Deserialize the internal data structure from a string.
            ///
            /// Args:
            ///     string (str): The string to read from.
            #[doc = $crate::serialization_format_string_compatible!()]
            #[staticmethod]
            #[pyo3(signature = (string, serialization_format="serde_json"))]
            fn deserialize_from_string(
                string: &str,
                serialization_format: &str,
            ) -> pyo3::PyResult<Self> {
                $crate::map_type!(
                    $opaque,
                    $crate::impl_helper::serialization::deserialize_from_string(
                        string,
                        serialization_format,
                    )
                )
                .map_err($crate::impl_helper::serialization::deserialize_error)
            }
        }
    };
}
use std::{
    error,
    fs::{self, File},
    io::{self},
    path::Path,
};

pub(crate) use serde;

fn open(path: impl AsRef<Path>) -> io::Result<File> {
    File::open(path)
}

fn create(path: impl AsRef<Path>) -> io::Result<File> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(path)
}

#[derive(Debug)]
pub struct UnknownFormat(String);

impl fmt::Display for UnknownFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown format: {}", self.0)
    }
}

impl error::Error for UnknownFormat {}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn serialize_to_file<T: Serialize, P: AsRef<Path>>(
    path: P,
    value: &T,
    format: &str,
) -> Result<()> {
    match format {
        "serde_json" => serde_json::to_writer(create(path)?, value)?,
        "bincode" => bincode::serialize_into(create(path)?, value)?,
        _ => return Err(UnknownFormat(format.to_owned()).into()),
    };
    Ok(())
}

pub fn deserialize_from_file<T: DeserializeOwned, P: AsRef<Path>>(
    path: P,
    format: &str,
) -> Result<T> {
    Ok(match format {
        "serde_json" => serde_json::from_reader(open(path)?)?,
        "bincode" => bincode::deserialize_from(open(path)?)?,
        _ => return Err(UnknownFormat(format.to_owned()).into()),
    })
}

pub fn serialize_to_string<T: Serialize>(value: &T, format: &str) -> Result<String> {
    Ok(match format {
        "serde_json" => serde_json::to_string(value)?,
        _ => return Err(UnknownFormat(format.to_owned()).into()),
    })
}

pub fn deserialize_from_string<'a, T: Deserialize<'a>>(
    s: &'a str,
    format: &str,
) -> Result<T> {
    Ok(match format {
        "serde_json" => serde_json::from_str(s)?,
        _ => return Err(UnknownFormat(format.to_owned()).into()),
    })
}
