use std::{path::PathBuf, str::FromStr};

use crate::parsed::ParsedArgument;

#[derive(Debug, Clone)]
pub enum Optionality {
    Required,
    Optional,
    Default(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int32,
    Float32,
    String,
    Bool,
    Path,
}

#[derive(Debug, Clone)]
pub struct UnparsedArgument {
    pub dest: String,
    pub data_type: DataType,
    pub short: Option<String>,
    pub long: Option<String>,
    pub optionality: Optionality,
}

impl UnparsedArgument {
    pub fn get_name(&self) -> String {
        if self.short.is_none() && self.long.is_none() {
            self.dest.clone()
        } else if self.short.is_some() && self.long.is_some() {
            let short = self.short.as_ref().unwrap();
            let long = self.long.as_ref().unwrap();
            format!("-{}, --{}", short, long)
        } else if self.short.is_some() {
            let short = self.short.as_ref().unwrap();
            format!("-{}", short)
        } else {
            let long = self.long.as_ref().unwrap();
            format!("--{}", long)
        }
    }

    pub fn parse_value(&self, value: &str) -> Result<ParsedArgument, String> {
        match self.data_type {
            DataType::Int32 => match value.parse::<i32>() {
                Ok(value) => Ok(ParsedArgument::Int32(value)),
                Err(_) => Err(format!("'{}' is not a 32-bit integer", value)),
            },
            DataType::Float32 => match value.parse::<f32>() {
                Ok(value) => Ok(ParsedArgument::Float32(value)),
                Err(_) => Err(format!("'{}' is not a 32-bit float", value)),
            },
            DataType::String => Ok(ParsedArgument::String(value.to_string())),
            DataType::Bool => match value.parse::<bool>() {
                Ok(value) => Ok(ParsedArgument::Bool(value)),
                Err(_) => Err(format!("'{}' is not a boolean", value)),
            },
            DataType::Path => match PathBuf::from_str(value) {
                Ok(value) => Ok(ParsedArgument::Path(value)),
                Err(_) => Err(format!("'{}' is not a path", value)),
            },
        }
    }
}
