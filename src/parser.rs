use std::{collections::HashMap, path::PathBuf, str::FromStr};

use self::{
    argument::{DataType, Optionality, ParsedArgument, UnparsedArgument},
    validation::{validate_long, validate_short},
};

pub mod argument;
mod validation;

pub struct ParseResult {
    arguments: HashMap<String, ParsedArgument>,
}

impl ParseResult {
    fn get_arg(&self, key: &str) -> &ParsedArgument {
        if let Some(arg) = self.arguments.get(key) {
            arg
        } else {
            panic!("Could not find value for argument '{}'", key)
        }
    }

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
        parser.add_flag("help", None, Some("help"), false);
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
        if let DataType::Bool = data_type {
            panic!(
                "The bool option '{}' should be added with add_flag instead.",
                dest
            );
        }

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
        let mut defaults_usage = String::new();

        let mut option_help = Vec::new();
        let mut positional_help = Vec::new();
        let mut optional_help = Vec::new();
        let mut defaults_help = Vec::new();
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
            } else if let Optionality::Optional = positional.optionality {
                optionals_usage.push_str(format!(" [{}]", positional.dest).as_str());
                optional_help.push((name, desc));
            } else {
                defaults_usage.push_str(format!(" [{}]", positional.dest).as_str());
                defaults_help.push((name, desc));
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
            "Usage: {}{}{}{}{}",
            env!("CARGO_PKG_NAME"),
            options_usage,
            positional_usage,
            optionals_usage,
            defaults_usage
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

            for (name, desc) in &defaults_help {
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
        let mut parsed_args = HashMap::new();
        let mut raw_args = std::env::args().skip(1);
        let mut unparsed_args = self.options.clone();
        unparsed_args.append(&mut self.flags.clone());
        unparsed_args.append(&mut self.positionals.clone());

        while let Some(raw_arg) = raw_args.next() {
            let idx_result = if raw_arg.starts_with("--") {
                let target_long = &raw_arg[2..];
                self.find_arg_with_long(target_long, &unparsed_args)
            } else if raw_arg.starts_with("-") {
                let target_short = &raw_arg[1..];
                self.find_arg_with_short(target_short, &unparsed_args)
            } else {
                self.find_next_positional(&unparsed_args)
            };

            match idx_result {
                Ok(idx) => {
                    let unparsed_arg = unparsed_args.remove(idx);

                    let value_res = if unparsed_arg.is_flag() {
                        match &unparsed_arg.optionality {
                            Optionality::Default(default_val) => match default_val.as_str() {
                                "true" => Ok("false".to_string()),
                                "false" => Ok("true".to_string()),
                                _ => Err(format!("'{}' is not a valid boolean", default_val)),
                            },
                            _ => Err("flags must have default values".to_string()),
                        }
                    } else if unparsed_arg.is_positional() {
                        Ok(raw_arg)
                    } else {
                        match raw_args.next() {
                            Some(raw_arg) => Ok(raw_arg),
                            None => Err(format!(
                                "no value provided for '{}'",
                                unparsed_arg.get_name()
                            )),
                        }
                    };

                    match value_res {
                        Ok(unparsed_val) => {
                            match self.parse_argument(&unparsed_arg, &unparsed_val) {
                                Ok(parsed_arg) => parsed_args.insert(unparsed_arg.dest, parsed_arg),
                                Err(message) => todo!("handle error message"),
                            };
                        }
                        Err(message) => todo!("handle error message"),
                    };
                }
                Err(message) => todo!("handle error message"),
            };
        }

        if let Some(ParsedArgument::Bool(true)) = parsed_args.get("help") {
            self.print_help();
            std::process::exit(0);
        }

        for unparsed_arg in unparsed_args {
            let parse_result = match &unparsed_arg.optionality {
                Optionality::Required => Err(format!("'{}' is required", unparsed_arg.get_name())),
                Optionality::Optional => continue,
                Optionality::Default(value) => self.parse_argument(&unparsed_arg, value),
            };

            match parse_result {
                Ok(parsed_arg) => parsed_args.insert(unparsed_arg.dest, parsed_arg),
                Err(message) => todo!("handle error message"),
            };
        }

        ParseResult {
            arguments: parsed_args,
        }
    }

    fn find_next_positional(&self, unparsed_args: &Vec<UnparsedArgument>) -> Result<usize, String> {
        let mut idx_result = Err("too many positionals provided".to_string());
        let mut is_required = false;
        let mut has_default = false;

        for (idx, unparsed_arg) in unparsed_args.iter().enumerate() {
            if unparsed_arg.is_positional() {
                match &unparsed_arg.optionality {
                    Optionality::Required => {
                        if !is_required {
                            is_required = true;
                            idx_result = Ok(idx);
                        }
                    }
                    Optionality::Optional => {
                        if !is_required || has_default {
                            has_default = false;
                            is_required = false;
                            idx_result = Ok(idx);
                        }
                    }
                    Optionality::Default(_) => {
                        if idx_result.is_err() {
                            has_default = true;
                            is_required = false;
                            idx_result = Ok(idx);
                        }
                    }
                }
            }
        }

        idx_result
    }

    fn find_arg_with_long(
        &self,
        target_long: &str,
        unparsed_args: &Vec<UnparsedArgument>,
    ) -> Result<usize, String> {
        for (idx, unparsed_arg) in unparsed_args.iter().enumerate() {
            if let Some(long) = &unparsed_arg.long {
                if long == target_long {
                    return Ok(idx);
                }
            }
        }
        Err(format!(
            "could not find corresponding option for '--{}'",
            target_long
        ))
    }

    fn find_arg_with_short(
        &self,
        target_short: &str,
        unparsed_args: &Vec<UnparsedArgument>,
    ) -> Result<usize, String> {
        for (idx, unparsed_arg) in unparsed_args.iter().enumerate() {
            if let Some(short) = &unparsed_arg.short {
                if short == target_short {
                    return Ok(idx);
                }
            }
        }
        Err(format!(
            "could not find corresponding option for '-{}'",
            target_short
        ))
    }

    fn parse_argument(
        &self,
        unparsed_arg: &UnparsedArgument,
        value: &str,
    ) -> Result<ParsedArgument, String> {
        match unparsed_arg.data_type {
            DataType::Int32 => match value.parse::<i32>() {
                Ok(res) => Ok(ParsedArgument::Int32(res)),
                Err(_) => Err(format!("'{}' is not a 32-bit int", value)),
            },
            DataType::Float32 => match value.parse::<f32>() {
                Ok(res) => Ok(ParsedArgument::Float32(res)),
                Err(_) => Err(format!("'{}' is not a 32-bit float", value)),
            },
            DataType::String => Ok(ParsedArgument::String(value.to_string())),
            DataType::Bool => match value {
                "true" => Ok(ParsedArgument::Bool(true)),
                "false" => Ok(ParsedArgument::Bool(false)),
                _ => Err(format!("'{}' is not a valid boolean", value)),
            },
            DataType::Path => match PathBuf::from_str(value) {
                Ok(path) => Ok(ParsedArgument::Path(path)),
                Err(_) => Err(format!("'{}' is not a valid path", value)),
            },
        }
    }
}
