use std::{collections::VecDeque, env::Args, iter::Skip};

use argument::{option::OptionArgument, positional::PositionalArgument, DataType};
use result::{ParseResult, ParseValue};

pub mod argument;
pub mod result;

#[derive(Debug)]
pub struct Parser {
    positionals: VecDeque<PositionalArgument>,
    options: Vec<OptionArgument>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            positionals: VecDeque::new(),
            options: Vec::new(),
        }
    }

    pub fn positional(
        &mut self,
        destination: &str,
        data_type: DataType,
    ) -> &mut PositionalArgument {
        let positional = PositionalArgument::new(self, destination, data_type);
        self.positionals.push_back(positional);
        self.positionals.back_mut().expect("was just added")
    }

    pub fn option(
        &mut self,
        names: &str,
        destination: &str,
        data_type: DataType,
    ) -> &mut OptionArgument {
        let option = OptionArgument::new(self, names, destination, data_type);
        self.options.push(option);
        self.options.last_mut().expect("was just added")
    }

    fn parse_positional(&mut self, result: &mut ParseResult, value: &str) {
        let positional = match self.positionals.pop_front() {
            Some(positional) => positional,
            None => todo!("error for exhausted positionals"),
        };

        let parse_value = match ParseValue::from_value(positional.data_type, &value) {
            Ok(parse_value) => parse_value,
            Err(_err) => todo!("error for invalid value for positional"),
        };

        let add_result = if positional.data_type.is_array() {
            self.positionals.push_front(positional.clone());
            result.add_array_value(&positional.destination, parse_value)
        } else {
            result.add_single_value(&positional.destination, parse_value)
        };

        if let Err(_err) = add_result {
            todo!("error for unable to add parse value")
        }
    }

    fn parse_option(&mut self, result: &mut ParseResult, name: &str, value: &str) {
        let mut option_idx = None;
        for (idx, option) in self.options.iter().enumerate() {
            if option.has_name(name) {
                option_idx = Some(idx);
                break;
            }
        }

        let option = if let Some(option_idx) = option_idx {
            self.options.remove(option_idx)
        } else {
            todo!("error for no matching option")
        };

        let parse_value = match ParseValue::from_value(option.data_type, &value) {
            Ok(parse_value) => parse_value,
            Err(_err) => todo!("error for invalid value for option"),
        };

        let add_result = if option.data_type.is_array() {
            self.options.push(option.clone());
            result.add_array_value(&option.destination, parse_value)
        } else {
            result.add_single_value(&option.destination, parse_value)
        };

        if let Err(_err) = add_result {
            todo!("error for unable to add parse value")
        }
    }

    pub fn parse_args(mut self) -> ParseResult {
        let mut raw_args = std::env::args().skip(1); // First arg is always prog name
        let mut result = ParseResult::new();

        while let Some(raw_arg) = raw_args.next() {
            let is_option = raw_arg.starts_with('-');
            if is_option {
                let value = match raw_args.next() {
                    Some(value) => value,
                    None => todo!("error for no option value provided"),
                };
                self.parse_option(&mut result, &raw_arg, &value);
            } else {
                self.parse_positional(&mut result, &raw_arg);
            }
        }

        // TODO: process unparsed arguments

        result
    }
}
