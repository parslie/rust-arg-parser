use self::validation::{validate_long, validate_short};

mod validation;

#[derive(Debug)]
pub enum Optionality {
    Required,
    Optional,
    Default(String),
}

#[derive(Debug)]
pub enum DataType {
    Int32,
    Float32,
    String,
    Bool,
}

#[derive(Debug)]
struct UnparsedArgument {
    dest: String,
    data_type: DataType,
    short: Option<String>,
    long: Option<String>,
    optionality: Optionality,
}

#[derive(Debug)]
pub struct Parser {
    positionals: Vec<UnparsedArgument>,
    options: Vec<UnparsedArgument>,
    flags: Vec<UnparsedArgument>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            positionals: Vec::new(),
            options: Vec::new(),
            flags: Vec::new(),
        }
    }

    pub fn add_positional(&mut self, dest: &str, data_type: DataType, optionality: Optionality) {
        let argument = UnparsedArgument {
            dest: dest.to_string(),
            data_type: data_type,
            short: None,
            long: None,
            optionality: optionality,
        };
        self.positionals.push(argument);
    }

    pub fn add_option(
        &mut self,
        dest: &str,
        data_type: DataType,
        short: Option<&str>,
        long: Option<&str>,
        optionality: Optionality,
    ) {
        if short.is_none() && long.is_none() {
            panic!(
                "The short and long of option '{}' can't both be empty.",
                dest
            );
        } else if !validate_short(short) {
            panic!("The short of option '{}' is invalid.", dest);
        } else if !validate_long(long) {
            panic!("The long of option '{}' is invalid.", dest);
        }

        let argument = UnparsedArgument {
            dest: dest.to_string(),
            data_type: data_type,
            short: match short {
                Some(val) => Some(val.to_string()),
                None => None,
            },
            long: match long {
                Some(val) => Some(val.to_string()),
                None => None,
            },
            optionality: optionality,
        };
        self.options.push(argument);
    }

    pub fn add_flag(&mut self, dest: &str, short: Option<&str>, long: Option<&str>, default: bool) {
        if short.is_none() && long.is_none() {
            panic!("The short and long of flag '{}' can't both be empty.", dest);
        } else if !validate_short(short) {
            panic!("The short of flag '{}' is invalid.", dest);
        } else if !validate_long(long) {
            panic!("The long of flag '{}' is invalid.", dest);
        }

        let argument = UnparsedArgument {
            dest: dest.to_string(),
            data_type: DataType::Bool,
            short: match short {
                Some(val) => Some(val.to_string()),
                None => None,
            },
            long: match long {
                Some(val) => Some(val.to_string()),
                None => None,
            },
            optionality: match default {
                true => Optionality::Default("true".to_string()),
                false => Optionality::Default("false".to_string()),
            },
        };
        self.flags.push(argument);
    }
}
