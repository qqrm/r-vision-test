use serde::{Deserialize, Serialize};
use std::ffi::OsStr;

/// Represents the type of data contained in a file.
/// Includes various data formats like Text, PNG, JSON, and YAML.
#[derive(Serialize, Deserialize, Debug)]
pub enum DataType {
    Text,
    Png,
    Json,
    Yaml,
    Unknown,
    NoExtension,
}

/// Represents a chunk of data from a file.
/// Includes metadata like the file name and data type.
#[derive(Serialize, Deserialize, Debug)]
pub struct DataChunk {
    pub file_name: String,
    pub data_type: DataType,
    pub data: Vec<u8>,
    pub last_chunk: bool,
}

/// Parses the file extension and returns the corresponding `DataType`.
///
/// # Arguments
///
/// * `ext` - An `Option<&OsStr>` representing the file extension.
pub fn parse_extension(ext: Option<&OsStr>) -> DataType {
    match ext.and_then(OsStr::to_str) {
        Some("txt") => DataType::Text,
        Some("png") => DataType::Png,
        Some("json") => DataType::Json,
        Some("yaml") => DataType::Yaml,
        _ => ext.map_or(DataType::NoExtension, |_| DataType::Unknown),
    }
}

/// Returns the file extension as a string based on the given `DataType`.
///
/// # Arguments
///
/// * `data_type` - A reference to a `DataType` enum.
pub fn get_extension(data_type: &DataType) -> String {
    match data_type {
        DataType::Text => "txt",
        DataType::Png => "png",
        DataType::Json => "json",
        DataType::Yaml => "yaml",
        DataType::Unknown => "hz",
        DataType::NoExtension => "",
    }
    .to_owned()
}
