use std::{
    error,
    fmt,
    fs::File,
    io::{
        self,
        Read,
        Write,
    },
    path::Path,
    str::FromStr,
};

use serde::{
    de::DeserializeOwned,
    Serialize,
};

#[derive(Debug)]
pub enum DeserializeFileError<T> {
    File(io::Error),
    Deserialize(T),
}

impl<T: fmt::Display> fmt::Display for DeserializeFileError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeserializeFileError::File(e) => write!(f, "file error: {}", e),
            DeserializeFileError::Deserialize(e) => {
                write!(f, "deserialization error: {}", e)
            },
        }
    }
}

impl<T: fmt::Display + fmt::Debug + error::Error> error::Error
    for DeserializeFileError<T>
{
}

pub trait Serde {
    type Error;

    fn read<T: DeserializeOwned, R: Read>(&self, r: R) -> Result<T, Self::Error>;

    fn write<T: Serialize, W: Write>(&self, w: W, value: &T)
    -> Result<(), Self::Error>;

    fn read_file<T: DeserializeOwned, P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<T, DeserializeFileError<Self::Error>> {
        let file = File::open(path).map_err(DeserializeFileError::File)?;
        self.read(file).map_err(DeserializeFileError::Deserialize)
    }

    fn write_file<T: Serialize, P: AsRef<Path>>(
        &self,
        path: P,
        value: &T,
    ) -> Result<(), DeserializeFileError<Self::Error>> {
        let file = File::create(path).map_err(DeserializeFileError::File)?;
        self.write(file, value).map_err(DeserializeFileError::Deserialize)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SerdeJson;

impl Serde for SerdeJson {
    type Error = serde_json::Error;

    fn read<T: DeserializeOwned, R: Read>(&self, r: R) -> Result<T, Self::Error> {
        serde_json::from_reader(r)
    }

    fn write<T: Serialize, W: Write>(
        &self,
        w: W,
        value: &T,
    ) -> Result<(), Self::Error> {
        serde_json::to_writer(w, value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bincode;

impl Serde for Bincode {
    type Error = bincode::Error;

    fn read<T: DeserializeOwned, R: Read>(&self, r: R) -> Result<T, Self::Error> {
        bincode::deserialize_from(r)
    }

    fn write<T: Serialize, W: Write>(
        &self,
        w: W,
        value: &T,
    ) -> Result<(), Self::Error> {
        bincode::serialize_into(w, value)
    }
}

macro_rules! dynamic {
    ($(($variant:ident, $format:path, $error:path, $name:literal),)*) => {
        #[derive(Debug, Clone, Copy)]
        pub enum Dynamic {
            $($variant,)*
        }

        #[derive(Debug)]
        pub enum DynamicError {
            $($variant($error),)*
        }

        impl Serde for Dynamic {
            type Error = DynamicError;

            fn read<T: DeserializeOwned, R: Read>(&self, r: R) -> Result<T, Self::Error> {
                match self {
                    $(
                        Self::$variant => {
                            $format.read(r).map_err(DynamicError::$variant)
                        },
                    )*
                }
            }

            fn write<T: Serialize, W: Write>(
                &self,
                w: W,
                value: &T,
            ) -> Result<(), Self::Error> {
                match self {
                    $(
                        Self::$variant => {
                            $format.write(w, value).map_err(DynamicError::$variant)
                        },
                    )*
                }
            }
        }

        impl FromStr for Dynamic {
            type Err = UnknownFormat;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $name => Ok(Self::$variant),
                    )*
                    _ => Err(UnknownFormat(s.to_owned())),
                }
            }
        }

        impl fmt::Display for DynamicError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        Self::$variant(e) => write!(f, "{} error: {}", $name, e),
                    )*
                }
            }
        }

        impl error::Error for DynamicError {}
    };
}
dynamic! {
    (SerdeJson, SerdeJson, serde_json::Error, "serde_json"),
    (Bincode, Bincode, bincode::Error, "bincode"),
}

#[derive(Debug)]
pub struct UnknownFormat(String);

impl fmt::Display for UnknownFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown format: {}", self.0)
    }
}

impl error::Error for UnknownFormat {}

impl Dynamic {
    pub fn read_file<T: DeserializeOwned, P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<T, DeserializeFileError<DynamicError>> {
        <Self as Serde>::read_file(self, path)
    }

    pub fn write_file<T: Serialize, P: AsRef<Path>>(
        &self,
        path: P,
        value: &T,
    ) -> Result<(), DeserializeFileError<DynamicError>> {
        <Self as Serde>::write_file(self, path, value)
    }
}

impl TryFrom<&str> for Dynamic {
    type Error = UnknownFormat;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}
