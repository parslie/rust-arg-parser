use std::path::PathBuf;

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

    pub fn is_positional(&self) -> bool {
        self.short.is_none() && self.long.is_none()
    }

    pub fn is_flag(&self) -> bool {
        !self.is_positional() && self.data_type == DataType::Bool
    }
}

#[derive(Debug)]
pub enum ParsedArgument {
    Int32(i32),
    Float32(f32),
    String(String),
    Bool(bool),
    Path(PathBuf),
    None,
}
