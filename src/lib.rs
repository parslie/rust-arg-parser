use std::collections::HashMap;

use parsed::ParsedArgument;
use result::ParseResult;
use unparsed::builder::UnparsedArgumentBuilder;
use unparsed::{DataType, Optionality, UnparsedArgument};

mod error;
mod parsed;
pub mod prelude;
pub mod result;
pub mod unparsed;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Parser {
    name: String,
    description: String,
    positionals: Vec<UnparsedArgument>,
    options: Vec<UnparsedArgument>,
    help_arg: Option<UnparsedArgument>,
}

impl Parser {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            positionals: Vec::new(),
            options: Vec::new(),
            help_arg: None,
        }
    }

    pub fn help_flag(&mut self) -> UnparsedArgumentBuilder {
        UnparsedArgumentBuilder::new("help", DataType::Bool, self)
            .is_help_flag(true)
            .default("false")
            .required(false)
    }

    pub fn argument(&mut self, destination: &str, data_type: DataType) -> UnparsedArgumentBuilder {
        UnparsedArgumentBuilder::new(destination, data_type, self)
    }

    pub fn parse_args(self) -> ParseResult {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let result = self.parse_args_inner(args);

        if !result.errors.is_empty() {
            self.print_help_page();
            println!();
            for error in result.errors {
                println!("ERROR: {}", &error);
            }
            std::process::exit(1);
        }

        result
    }

    fn parse_args_inner(&self, args: Vec<String>) -> ParseResult {
        let mut parsed_args = HashMap::new();
        let mut parse_errors = Vec::new();

        let mut positionals = self.positionals.clone();
        let mut options = self.options.clone();

        // Parse user inputted args into parsed arguments
        let mut raw_args = args.into_iter();
        while let Some(raw_arg) = raw_args.next() {
            let is_option = raw_arg.starts_with('-');

            let unparsed_arg = if is_option {
                let option_pred = |o: &UnparsedArgument| {
                    if raw_arg.starts_with("--") {
                        let target_long = (&raw_arg[2..]).to_string();
                        o.long_name == Some(target_long)
                    } else {
                        let target_short = (&raw_arg[1..]).to_string();
                        o.short_name == Some(target_short)
                    }
                };

                if let Some(help_arg) = &self.help_arg {
                    let is_help_flag = option_pred(help_arg);
                    if is_help_flag {
                        self.print_help_page();
                        std::process::exit(0);
                    }
                }

                let option_idx = match options.iter().position(option_pred) {
                    Some(idx) => idx,
                    None => {
                        parse_errors.push(error::unrecognized_option(&raw_arg));
                        continue; // Continues so more errors can be discovered
                    }
                };

                options.remove(option_idx)
            } else {
                if !positionals.is_empty() {
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

            let parsed_arg = match ParsedArgument::from_value(&value, unparsed_arg.data_type) {
                Ok(inner) => inner,
                Err(_) => {
                    parse_errors.push(error::invalid_value(&unparsed_arg, &value));
                    continue; // Continues so more errors can be discovered
                }
            };
            parsed_args.insert(unparsed_arg.destination.clone(), parsed_arg);
        }

        // Remove optional options & set default values
        let option_range = (0..options.len()).rev();
        for option_idx in option_range {
            let option = &options[option_idx];
            if let Optionality::Optional = &option.optionality {
                options.remove(option_idx);
            } else if let Optionality::Default(value) = &option.optionality {
                let parsed_arg = ParsedArgument::from_value(&value, option.data_type)
                    .expect("validation makes sure default is valid");
                parsed_args.insert(option.destination.clone(), parsed_arg);
                options.remove(option_idx);
            }
        }

        // Check whether there are unparsed required arguments
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

    pub fn print_help_page(&self) {
        const TAB: &str = "   ";

        /*
        Usage: name [OPTIONS] [POSITIONALS]

        description

        Positionals:
            [DEST] - [DESCRIPTION]

        Options:
            [NAME] - [DESCRIPTION]
        */

        let mut usage_str = format!("Usage: {}", &self.name);
        if !self.options.is_empty() {
            usage_str.push_str(" [OPTION]...");
        }
        for positional in &self.positionals {
            usage_str.push_str(" [");
            usage_str.push_str(&positional.get_name());
            usage_str.push_str("]");
        }

        println!("{}", usage_str);
        println!("{}", &self.description);

        if !self.positionals.is_empty() {
            let mut name_len = 0usize;
            let mut name_desc_tuples = Vec::new();

            for positional in &self.positionals {
                let name = positional.get_name();
                let desc = "TODO: DESCRIPTIONS".to_string();
                if name.len() > name_len {
                    name_len = name.len();
                }
                name_desc_tuples.push((name, desc));
            }

            println!("\nPositionals:");
            for (name, description) in name_desc_tuples {
                print!("{}{}", TAB, &name);
                for _ in 0..(name_len - name.len()) {
                    print!(" ");
                }
                println!(" - {}", &description);
            }
        }

        if !self.options.is_empty() {
            let mut name_len = 0usize;
            let mut name_desc_tuples = Vec::new();

            let mut options = self.options.clone();
            if let Some(help_arg) = &self.help_arg {
                options.push(help_arg.clone());
            }
            // TODO: sort options

            for option in options {
                let name = option.get_name();
                let desc = "TODO: DESCRIPTIONS".to_string();
                if name.len() > name_len {
                    name_len = name.len();
                }
                name_desc_tuples.push((name, desc));
            }

            println!("\nOptions:");
            for (name, description) in name_desc_tuples {
                print!("{}{}", TAB, &name);
                for _ in 0..(name_len - name.len()) {
                    print!(" ");
                }
                println!(" - {}", &description);
            }
        }
    }
}
