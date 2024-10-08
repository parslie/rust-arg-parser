use crate::{result::ParseValue, Parser};

use super::DataType;

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct PositionalArgument {
    pub(crate) parser: *const Parser,
    pub(crate) destination: String,
    pub(crate) data_type: DataType,
    pub(crate) is_required: Option<bool>,
    pub(crate) defaults: Option<Vec<String>>,
}

impl PositionalArgument {
    pub(crate) fn new(parser: &Parser, destination: &str, data_type: DataType) -> Self {
        for positional in &parser.positionals {
            if destination == positional.destination.as_str() {
                panic!(
                    "destination '{}' is occupied by another positional",
                    destination
                );
            }
        }
        for option in &parser.options {
            if destination == option.destination.as_str() {
                panic!("destination '{}' is occupied by an option", destination);
            }
        }

        if data_type.is_array() && !parser.child_parsers.is_empty() {
            panic!(
                "positional array '{}' cannot be added, since a sub-parser has been added",
                &destination
            );
        }

        if let Some(prev_positional) = parser.positionals.back() {
            let prev_is_array = prev_positional.data_type.is_array();
            let prev_is_optional = prev_positional.is_required == Some(false)
                || (prev_positional.is_required == None && prev_positional.defaults.is_some());

            if prev_is_optional {
                panic!("only the last positional can be optional");
            } else if prev_is_array {
                panic!("only the last positional can be an array");
            }
        }

        Self {
            parser: parser as *const Parser,
            destination: destination.to_string(),
            data_type,
            is_required: None,
            defaults: None,
        }
    }

    pub fn is_required(&mut self, is_required: bool) -> &mut Self {
        let parser = unsafe { self.parser.as_ref().expect("should not be null") };
        if !is_required && !parser.child_parsers.is_empty() {
            panic!(
                "optional positional '{}' cannot be added, since a sub-parser has been added",
                &self.destination
            );
        }
        if is_required && self.defaults.is_some() {
            panic!(
                "positional '{}' cannot be required and have a default value simultaneously",
                &self.destination
            );
        }
        self.is_required = Some(is_required);
        self
    }

    pub fn defaults(&mut self, defaults: &[&str]) -> &mut Self {
        if !self.data_type.is_array() && defaults.len() > 1 {
            panic!(
                "positional '{}' is not an array and can only have one default value",
                &self.destination
            );
        }
        if self.is_required == Some(true) {
            panic!(
                "positional '{}' cannot be required and have a default value simultaneously",
                &self.destination
            );
        }
        for default_value in defaults {
            if let Err(err) = ParseValue::from_value(self.data_type, default_value) {
                panic!(
                    "positional '{}' default value is invalid: {}",
                    &self.destination, &err
                );
            }
        }
        self.defaults = Some(defaults.iter().map(|default| default.to_string()).collect());
        self
    }
}
