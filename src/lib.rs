use parsed::ParsedArgument;
use regex::Regex;
use std::collections::HashMap;
use unparsed::{DataType, Optionality, UnparsedArgument};

mod error;
mod parsed;
mod unparsed;

#[cfg(test)]
mod test;

pub struct ParseResult {
    arguments: HashMap<String, ParsedArgument>,
    errors: Vec<String>,
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

        // TODO: should positionals be able to be optional or default?
        //       if not, remember to remove from tests

        // TODO: boolean options must have a default

        if let Optionality::Default(value) = &arg.optionality {
            if let Err(_) = arg.parse_value(value.as_str()) {
                panic!(
                    "Default value of argument with dest '{}' is invalid!",
                    &arg.dest
                );
            }
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

    pub fn parse_args(self) -> ParseResult {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let parse_result = self.parse_args_inner(args);

        // TODO: if help option has been provided print help screen then exit

        if !parse_result.errors.is_empty() {
            // TODO: print help page
            std::process::exit(1);
        }

        parse_result
    }

    fn parse_args_inner(self, args: Vec<String>) -> ParseResult {
        let mut parsed_args = HashMap::new();
        let mut parse_errors = Vec::new();

        let mut positionals = self.positionals.clone();
        let mut options = self.options.clone();

        let mut raw_args = args.into_iter();
        while let Some(raw_arg) = raw_args.next() {
            let is_option = raw_arg.starts_with('-');

            let unparsed_arg = if is_option {
                let option_pred = |o: &UnparsedArgument| {
                    if raw_arg.starts_with("--") {
                        let target_long = (&raw_arg[2..]).to_string();
                        o.long == Some(target_long)
                    } else {
                        let target_short = (&raw_arg[1..]).to_string();
                        o.short == Some(target_short)
                    }
                };

                let option_idx = match options.iter().position(option_pred) {
                    Some(idx) => idx,
                    None => {
                        parse_errors.push(error::unrecognized_option(&raw_arg));
                        continue; // Continues so more errors can be discovered
                    }
                };

                options.remove(option_idx)
            } else {
                if positionals.len() > 0 {
                    positionals.remove(0)
                } else {
                    parse_errors.push(error::unrecognized_positional(&raw_arg));
                    continue; // Continues so more errors can be discovered
                }
            };

            let value = if is_option {
                let is_flag = unparsed_arg.data_type == DataType::Bool;

                if is_flag {
                    // Validation makes sure that boolean options must have a default
                    let default_value = unsafe { unparsed_arg.get_default() };
                    match default_value.as_str() {
                        "true" => "false".to_string(),
                        _ => "true".to_string(),
                    }
                } else if let Some(next_raw_arg) = raw_args.next() {
                    next_raw_arg
                } else {
                    parse_errors.push(error::no_value_provided(&unparsed_arg));
                    break; // Breaks because there are no arguments left
                }
            } else {
                raw_arg
            };

            let parsed_arg = match unparsed_arg.parse_value(&value) {
                Ok(value) => value,
                Err(_) => {
                    parse_errors.push(error::invalid_value(&unparsed_arg, &value));
                    continue; // Continues so more errors can be discovered
                }
            };
            parsed_args.insert(unparsed_arg.dest.clone(), parsed_arg);
        }

        // TODO: remove if default/optional positionals should be disallowed
        // Remove optional positionals & set default values
        let positional_range = (0..positionals.len()).rev();
        for positional_idx in positional_range {
            let positional = &positionals[positional_idx];
            if let Optionality::Optional = &positional.optionality {
                positionals.remove(positional_idx);
            } else if let Optionality::Default(value) = &positional.optionality {
                let parsed_arg = positional
                    .parse_value(value.as_str())
                    .expect("validation makes sure default is valid");
                parsed_args.insert(positional.dest.clone(), parsed_arg);
                positionals.remove(positional_idx);
            }
        }

        // Remove optional options & set default values
        let option_range = (0..options.len()).rev();
        for option_idx in option_range {
            let option = &options[option_idx];
            if let Optionality::Optional = &option.optionality {
                options.remove(option_idx);
            } else if let Optionality::Default(value) = &option.optionality {
                let parsed_arg = option
                    .parse_value(value.as_str())
                    .expect("validation makes sure default is valid");
                parsed_args.insert(option.dest.clone(), parsed_arg);
                options.remove(option_idx);
            }
        }

        if !positionals.is_empty() {
            parse_errors.push(error::REQUIRED_POSITIONALS.to_string());
        }
        if !options.is_empty() {
            parse_errors.push(error::REQUIRED_OPTIONS.to_string());
        }

        ParseResult {
            arguments: parsed_args,
            errors: parse_errors,
        }
    }
}
