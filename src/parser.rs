use std::collections::HashMap;

use self::validation::{validate_long, validate_short};

mod validation;

#[derive(Debug)]
pub enum ParsedArgument {
    Int32(i32),
    Float32(f32),
    String(String),
    Bool(bool),
    None,
}

#[derive(Debug)]
pub struct ParseResult {
    arguments: HashMap<String, ParsedArgument>,
}

#[derive(Debug, Clone)]
pub enum Optionality {
    Required,
    Optional,
    Default(String),
}

#[derive(Debug, Clone)]
pub enum DataType {
    Int32,
    Float32,
    String,
    Bool,
}

#[derive(Debug, Clone)]
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
        let mut parser = Self {
            positionals: Vec::new(),
            options: Vec::new(),
            flags: Vec::new(),
        };
        parser.add_flag("help", Some("h"), Some("help"), false);
        parser
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

    pub fn print_help(&self) {
        let spacing = 2;

        let positionals = self.positionals.clone();
        let mut options = self.options.clone();
        options.append(self.flags.clone().as_mut());

        let options_usage = if !options.is_empty() {
            " [options...]"
        } else {
            ""
        };
        let mut positional_usage = String::new();
        let mut optionals_usage = String::new();

        let mut option_help = Vec::new();
        let mut positional_help = Vec::new();
        let mut optional_help = Vec::new();
        let mut name_len = 0usize;

        for positional in &positionals {
            let desc = "TODO: DESC".to_string();
            let name = positional.dest.clone();

            if name.len() > name_len {
                name_len = name.len();
            }

            if let Optionality::Required = positional.optionality {
                positional_usage.push_str(format!(" {}", positional.dest).as_str());
                positional_help.push((name, desc));
            } else {
                optionals_usage.push_str(format!(" [{}]", positional.dest).as_str());
                optional_help.push((name, desc));
            }
        }

        for option in &options {
            let desc = "TODO: DESC".to_string();
            let mut name = String::new();
            if let Some(short) = &option.short {
                name.push_str(format!("-{}", short).as_str());
            }
            if let Some(long) = &option.long {
                if name.is_empty() {
                    name.push_str(format!("    --{}", long).as_str());
                } else {
                    name.push_str(format!(", --{}", long).as_str());
                }
            }

            if name.len() > name_len {
                name_len = name.len();
            }

            option_help.push((name, desc));
        }

        println!(
            "Usage: {}{}{}{}",
            env!("CARGO_PKG_NAME"),
            options_usage,
            positional_usage,
            optionals_usage
        );
        println!();

        if !positional_help.is_empty() || optional_help.is_empty() {
            println!("Positionals: ");

            for (name, desc) in &positional_help {
                for _ in 0..spacing {
                    print!(" ");
                }
                print!("{}", name);
                for _ in 0..(name_len + spacing - name.len()) {
                    print!(" ");
                }
                println!("{}", desc);
            }

            for (name, desc) in &optional_help {
                for _ in 0..spacing {
                    print!(" ");
                }
                print!("{}", name);
                for _ in 0..(name_len + spacing - name.len()) {
                    print!(" ");
                }
                println!("{}", desc);
            }

            println!();
        }

        if !option_help.is_empty() {
            println!("Options: ");

            for (name, desc) in &option_help {
                for _ in 0..spacing {
                    print!(" ");
                }
                print!("{}", name);
                for _ in 0..(name_len + spacing - name.len()) {
                    print!(" ");
                }
                println!("{}", desc);
            }
        }
    }

    pub fn parse_arguments(&self) -> ParseResult {
        todo!()
    }
}
