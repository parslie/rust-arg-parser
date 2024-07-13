pub mod builder;

#[derive(Debug, Clone)]
pub enum Optionality {
    Required,
    Optional,
    Default(String),
}

impl Optionality {
    pub fn is_default(&self) -> bool {
        if let Self::Default(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Int32,
    Float32,
    String,
    Bool,
    Path,
}

#[derive(Debug, Clone)]
pub struct UnparsedArgument {
    pub description: String,
    pub destination: String,
    pub data_type: DataType,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub optionality: Optionality,
}

impl UnparsedArgument {
    pub fn get_name(&self) -> String {
        if self.short_name.is_none() && self.long_name.is_none() {
            self.destination.to_uppercase()
        } else if self.short_name.is_some() && self.long_name.is_some() {
            let short = self.short_name.as_ref().unwrap();
            let long = self.long_name.as_ref().unwrap();
            format!("-{}, --{}", short, long)
        } else if self.short_name.is_some() {
            let short = self.short_name.as_ref().unwrap();
            format!("-{}", short)
        } else {
            let long = self.long_name.as_ref().unwrap();
            format!("--{}", long)
        }
    }

    pub fn is_option(&self) -> bool {
        self.short_name.is_some() || self.long_name.is_some()
    }

    pub unsafe fn get_default(&self) -> String {
        if let Optionality::Default(value) = &self.optionality {
            value.clone()
        } else {
            panic!("No default value for argument '{}'", self.get_name());
        }
    }
}
