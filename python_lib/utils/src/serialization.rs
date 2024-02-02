use std::{
    error, fmt,
    fs::{self, File},
    io::{self},
    path::Path,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
