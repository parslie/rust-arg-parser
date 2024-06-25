use std::{path::PathBuf, str::FromStr};

use crate::unparsed::DataType;

#[derive(Debug)]
pub enum ParsedArgument {
    Int32(i32),
    Float32(f32),
    String(String),
    Bool(bool),
    Path(PathBuf),
}

impl ParsedArgument {
    pub fn from_value(value: &str, data_type: DataType) -> Result<Self, String> {
        match data_type {
            DataType::Int32 => match value.parse::<i32>() {
                Ok(value) => Ok(Self::Int32(value)),
                Err(_) => Err(format!("'{}' is not a 32-bit integer", value)),
            },
            DataType::Float32 => match value.parse::<f32>() {
                Ok(value) => Ok(Self::Float32(value)),
                Err(_) => Err(format!("'{}' is not a 32-bit float", value)),
            },
            DataType::String => Ok(Self::String(value.to_string())),
            DataType::Bool => match value.parse::<bool>() {
                Ok(value) => Ok(Self::Bool(value)),
                Err(_) => Err(format!("'{}' is not a boolean", value)),
            },
            DataType::Path => match PathBuf::from_str(value) {
                // TODO: further validation needed here to make sure it is correct
                Ok(value) => Ok(Self::Path(value)),
                Err(_) => Err(format!("'{}' is not a path", value)),
            },
        }
    }

    pub unsafe fn value<T: Clone>(&self) -> T {
        let generic_ptr = match self {
            ParsedArgument::Int32(value) => value as *const i32 as *const T,
            ParsedArgument::Float32(value) => value as *const f32 as *const T,
            ParsedArgument::String(value) => value as *const String as *const T,
            ParsedArgument::Bool(value) => value as *const bool as *const T,
            ParsedArgument::Path(value) => value as *const PathBuf as *const T,
        };
        unsafe { (*generic_ptr).clone() }
    }
}
