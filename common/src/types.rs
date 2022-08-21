use serde::{Deserialize, Serialize};
use std::ffi::OsStr;

#[derive(Serialize, Deserialize, Debug)]
pub enum DataType {
    Text,
    Png,
    Json,
    Yaml,
    Unknown,
    NoExtension,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataChunk {
    pub file_name: String,
    pub data_type: DataType,
    pub data: Vec<u8>,
    pub last_chunk: bool,
}

pub fn parse_extention(ext: Option<&OsStr>) -> DataType {
    match ext {
        Some(ext) => match ext.to_str() {
            Some("txt") => DataType::Text,
            Some("png") => DataType::Png,
            Some("json") => DataType::Json,
            Some("yaml") => DataType::Yaml,
            _ => DataType::Unknown,
        },
        None => DataType::NoExtension,
    }
}

pub fn get_extention(data_type: &DataType) -> String {
    match data_type {
        DataType::Text => "txt".to_owned(),
        DataType::Png => "png".to_owned(),
        DataType::Json => "json".to_owned(),
        DataType::Yaml => "yaml".to_owned(),
        DataType::Unknown => "hz".to_owned(),
        DataType::NoExtension => "".to_owned(),
    }
}
