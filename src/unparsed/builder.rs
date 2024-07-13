use regex::bytes::Regex;

use crate::{parsed::ParsedArgument, Parser};

use super::{DataType, Optionality, UnparsedArgument};

pub struct UnparsedArgumentBuilder<'a> {
    parser: &'a mut Parser,
    destination: String,
    data_type: DataType,
    short_name: Option<String>,
    long_name: Option<String>,
    required: Option<bool>,
    default: Option<String>,
    is_help_flag: bool,
}

impl<'a> UnparsedArgumentBuilder<'a> {
    pub fn new(destination: &str, data_type: DataType, parser: &'a mut Parser) -> Self {
        Self {
            parser,
            destination: destination.to_string(),
            data_type,
            short_name: None,
            long_name: None,
            required: None,
            default: None,
            is_help_flag: false,
        }
    }

    pub(crate) fn is_help_flag(mut self, value: bool) -> Self {
        self.is_help_flag = value;
        self
    }

    pub fn short_name(mut self, short_name: &str) -> Self {
        self.short_name = Some(short_name.to_string());
        self
    }

    pub fn long_name(mut self, long_name: &str) -> Self {
        self.long_name = Some(long_name.to_string());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }

    pub fn default(mut self, default: &str) -> Self {
        self.default = Some(default.to_string());
        self
    }

    fn build_positional(self) {
        if let Some(false) = self.required {
            panic!("Positionals cannot be optional.")
        }

        if let Some(_) = self.default {
            panic!("Positionals cannot have default values.")
        }

        for positional in &self.parser.positionals {
            if positional.destination == self.destination {
                panic!(
                    "Argument with destination '{}' already exists.",
                    &self.destination
                );
            }
        }

        for option in &self.parser.options {
            if option.destination == self.destination {
                panic!(
                    "Argument with destination '{}' already exists.",
                    &self.destination
                );
            }
        }

        let argument = UnparsedArgument {
            destination: self.destination,
            data_type: self.data_type,
            short_name: None,
            long_name: None,
            optionality: Optionality::Required,
        };
        self.parser.positionals.push(argument);
    }

    fn build_option(self) {
        let required = match self.required {
            Some(inner) => inner,
            None => false,
        };

        let optionality = match self.default {
            Some(default) => match required {
                true => panic!("Options cannot be required and have a default value."),
                false => match ParsedArgument::from_value(&default, self.data_type) {
                    Ok(_) => Optionality::Default(default.to_string()),
                    Err(error) => panic!("Invalid default '{}': {}.", &default, &error),
                },
            },
            None => match required {
                true => Optionality::Required,
                false => Optionality::Optional,
            },
        };

        if let Some(short_name) = &self.short_name {
            let regex = Regex::new(r"^[A-Za-z0-9]$").unwrap();
            if !regex.is_match(short_name.as_bytes()) {
                panic!("Option short names must consist of a single alphanumerical character.");
            }
        }

        if let Some(long_name) = &self.long_name {
            let regex = Regex::new(r"^[A-Za-z0-9][A-Za-z0-9\-]*$").unwrap();
            if !regex.is_match(long_name.as_bytes()) {
                panic!(
                    "Option long names must consist of only alphanumerical characters and hyphens (and start with an alphanumerical)."
                );
            }
        }

        for positional in &self.parser.positionals {
            if positional.destination == self.destination {
                panic!(
                    "Argument with destination '{}' already exists.",
                    &self.destination
                );
            }
        }

        for option in &self.parser.options {
            if option.destination == self.destination {
                panic!(
                    "Argument with destination '{}' already exists.",
                    &self.destination
                );
            } else if self.short_name.is_some() && option.short_name == self.short_name {
                let short_name = self.short_name.unwrap();
                panic!("Option with short name '-{}' already exists.", &short_name);
            } else if self.long_name.is_some() && option.long_name == self.long_name {
                let long_name = self.long_name.unwrap();
                panic!("Option with long name '--{}' already exists.", &long_name);
            }
        }

        let argument = UnparsedArgument {
            destination: self.destination,
            data_type: self.data_type,
            short_name: self.short_name,
            long_name: self.long_name,
            optionality,
        };
        self.parser.options.push(argument);
    }

    fn build_help_flag(self) {
        if let Some(true) = self.required {
            panic!("Help flag must not be required.");
        }

        let optionality = match self.default.as_deref() {
            Some("false") => Optionality::Default("false".to_string()),
            _ => panic!("Help flag must be false by default"),
        };

        if let Some(short_name) = &self.short_name {
            let regex = Regex::new(r"^[A-Za-z0-9]$").unwrap();
            if !regex.is_match(short_name.as_bytes()) {
                panic!("Help flag short name must consist of a single alphanumerical character.");
            }
        }

        if let Some(long_name) = &self.long_name {
            let regex = Regex::new(r"^[A-Za-z0-9][A-Za-z0-9\-]*$").unwrap();
            if !regex.is_match(long_name.as_bytes()) {
                panic!(
                    "Help flag long name must consist of only alphanumerical characters and hyphens (and start with an alphanumerical)."
                );
            }
        }

        for positional in &self.parser.positionals {
            if positional.destination == self.destination {
                panic!(
                    "Argument with destination '{}' already exists.",
                    &self.destination
                );
            }
        }

        for option in &self.parser.options {
            if option.destination == self.destination {
                panic!(
                    "Argument with destination '{}' already exists.",
                    &self.destination
                );
            } else if self.short_name.is_some() && option.short_name == self.short_name {
                let short_name = self.short_name.unwrap();
                panic!("Option with short name '-{}' already exists.", &short_name);
            } else if self.long_name.is_some() && option.long_name == self.long_name {
                let long_name = self.long_name.unwrap();
                panic!("Option with long name '--{}' already exists.", &long_name);
            }
        }

        let argument = UnparsedArgument {
            destination: self.destination,
            data_type: self.data_type,
            short_name: self.short_name,
            long_name: self.long_name,
            optionality,
        };
        self.parser.help_arg = Some(argument);
    }

    pub fn build(self) {
        let is_option = self.short_name.is_some() || self.long_name.is_some() || self.is_help_flag;
        if self.is_help_flag {
            self.build_help_flag();
        } else if is_option {
            self.build_option();
        } else {
            self.build_positional();
        }
    }
}
