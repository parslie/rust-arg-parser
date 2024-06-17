use parsed::ParsedArgument;
use regex::Regex;
use std::collections::HashMap;
use unparsed::{DataType, Optionality, UnparsedArgument};

mod parsed;
mod unparsed;

#[cfg(test)]
mod test;

pub struct ParseResult {
    arguments: HashMap<String, ParsedArgument>,
}

impl ParseResult {
    pub unsafe fn get<T: Clone>(&self, key: &str) -> T {
        match self.arguments.get(key) {
            Some(arg) => unsafe { arg.value::<T>() },
            None => panic!("Could not find value for argument '{}'", key),
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.arguments.contains_key(key)
    }
}

#[derive(Debug)]
pub struct Parser {
    name: String,
    description: String,
    positionals: Vec<UnparsedArgument>,
    options: Vec<UnparsedArgument>,
}

impl Parser {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            positionals: Vec::new(),
            options: Vec::new(),
        }
    }

    fn validate_new_arg(&self, arg: &UnparsedArgument) {
        // Short and long name validation doesn't need to be done here
        // since it's done before creating the arg

        if let Optionality::Default(value) = &arg.optionality {
            // TODO: validate default values
        }

        for existing_args in [&self.positionals, &self.options] {
            for existing_arg in existing_args {
                if arg.dest == existing_arg.dest {
                    panic!("Two arguments with dest '{}' exist!", &arg.dest);
                } else if arg.short.is_some() && arg.short == existing_arg.short {
                    let short = arg.short.as_ref().unwrap();
                    panic!("Two arguments with short '-{}' exist!", short);
                } else if arg.long.is_some() && arg.long == existing_arg.long {
                    let long = arg.long.as_ref().unwrap();
                    panic!("Two arguments with long '--{}' exist!", long);
                }
            }
        }
    }

    pub fn add_positional(&mut self, dest: &str, data_type: DataType, optionality: Optionality) {
        let arg = UnparsedArgument {
            dest: dest.to_string(),
            data_type: data_type,
            short: None,
            long: None,
            optionality: optionality,
        };
        self.validate_new_arg(&arg);
        self.positionals.push(arg);
    }

    pub fn add_option(
        &mut self,
        dest: &str,
        name: &str,
        data_type: DataType,
        optionality: Optionality,
    ) {
        if name.is_empty() {
            panic!("Empty name provided for option with dest '{}'!", dest);
        }

        let only_short_re = Regex::new(r"^-[A-Za-z0-9]$").unwrap();
        let only_long_re = Regex::new(r"^--[A-Za-z0-9]+(-[A-Za-z0-9]+)*$").unwrap();
        let both_re = Regex::new(r"^-[A-Za-z0-9],--[A-Za-z0-9]+(-[A-Za-z0-9]+)*$").unwrap();

        let (short, long) = if only_short_re.is_match(name) {
            let short = name[1..].to_string();
            (Some(short), None)
        } else if only_long_re.is_match(name) {
            let long = name[2..].to_string();
            (None, Some(long))
        } else if both_re.is_match(name) {
            let (short, long) = name.split_once(',').unwrap();
            let short = short[1..].to_string();
            let long = long[2..].to_string();
            (Some(short), Some(long))
        } else {
            panic!("Invalid name format for argument with dest '{}'!", dest);
        };

        let arg = UnparsedArgument {
            dest: dest.to_string(),
            data_type: data_type,
            short: short,
            long: long,
            optionality: optionality,
        };
        self.validate_new_arg(&arg);
        self.options.push(arg);
    }
}
