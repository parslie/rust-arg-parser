use regex::Regex;

use crate::Parser;

use super::DataType;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct OptionArgument {
    pub(crate) short_name: Option<String>,
    pub(crate) long_name: Option<String>,
    pub(crate) destination: String,
    pub(crate) data_type: DataType,
    pub(crate) is_required: Option<bool>,
    pub(crate) defaults: Option<Vec<String>>,
}

impl OptionArgument {
    pub(crate) fn new(
        parser: &Parser,
        names: &str,
        destination: &str,
        data_type: DataType,
    ) -> Self {
        for positional in &parser.positionals {
            if destination == positional.destination.as_str() {
                panic!("destination '{}' is occupied by a positional", destination);
            }
        }
        for option in &parser.options {
            if destination == option.destination.as_str() {
                panic!(
                    "destination '{}' is occupied by another option",
                    destination
                );
            }
        }

        let both_name_re = Regex::new(r"^-[A-Za-z0-9] *,--[A-Za-z0-9-]+$").unwrap();
        let short_name_re = Regex::new(r"^-[A-Za-z0-9]$").unwrap();
        let long_name_re = Regex::new(r"^--[A-Za-z0-9-]+$").unwrap();

        let (short_name, long_name) = if both_name_re.is_match(names) {
            let (short_name, long_name) = names.split_once(',').expect("checked with regex");
            let short_name = short_name[1..].to_string();
            let long_name = long_name[2..].to_string();
            (Some(short_name), Some(long_name))
        } else if short_name_re.is_match(names) {
            let short_name = names[1..].to_string();
            (Some(short_name), None)
        } else if long_name_re.is_match(names) {
            let long_name = names[2..].to_string();
            (None, Some(long_name))
        } else {
            panic!("names of option '{}' need to be formatted like '-v', '--verbose', or '-v,--verbose'", destination);
        };

        for option in &parser.options {
            if short_name.is_some() && option.short_name == short_name {
                panic!(
                    "short name '-{}' is occupied by another option",
                    short_name.unwrap().as_str()
                );
            }
            if long_name.is_some() && option.long_name == long_name {
                panic!(
                    "long name '--{}' is occupied by another option",
                    long_name.unwrap().as_str()
                );
            }
        }

        Self {
            short_name,
            long_name,
            destination: destination.to_string(),
            data_type,
            is_required: None,
            defaults: None,
        }
    }

    pub fn is_required(&mut self, is_required: bool) -> &mut Self {
        if is_required && self.defaults.is_some() {
            panic!(
                "option '{}' cannot be required and have a default value simultaneously",
                &self.destination
            );
        }
        self.is_required = Some(is_required);
        self
    }

    pub fn defaults(&mut self, defaults: &[&str]) -> &mut Self {
        if !self.data_type.is_array() && defaults.len() > 1 {
            panic!(
                "option '{}' is not an array and can only have one default value",
                &self.destination
            );
        }
        if self.is_required == Some(true) {
            panic!(
                "option '{}' cannot be required and have a default value simultaneously",
                &self.destination
            );
        }
        // TODO: try parse default value when parsing is implemented
        self.defaults = Some(defaults.iter().map(|default| default.to_string()).collect());
        self
    }
}
