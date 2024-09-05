use std::{
    collections::{HashMap, VecDeque},
    env::Args,
    iter::Skip,
};

use argument::{option::OptionArgument, positional::PositionalArgument, DataType};
use result::{ParseResult, ParseValue};

pub mod argument;
pub mod result;

#[derive(Debug)]
pub struct Parser {
    // Argument variables
    positionals: VecDeque<PositionalArgument>,
    options: Vec<OptionArgument>,
    // Sub parser variables
    parent_parser: Option<*const Parser>,
    child_parsers: HashMap<String, Parser>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            positionals: VecDeque::new(),
            options: Vec::new(),
            parent_parser: None,
            child_parsers: HashMap::new(),
        }
    }

    // TODO: add functions to add child parser
    // TODO: if positionals are exhausted, choose child parser
    // TODO: validate that the the last positional is not optional or an array when adding a child parser
    // TODO: validate that there are no child parsers when adding an array or optional positional

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

    fn parse_option(&mut self, result: &mut ParseResult, name: &str, raw_args: &mut Skip<Args>) {
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

        // NOTE TO SELF: Boolean arrays need to have specified values
        // since an incremental argument type is to be
        // introduced in the future, making it so allowing them to be
        // unspecified is practically useless.
        let value = match option.data_type {
            DataType::Bool(false) => match &option.defaults {
                Some(defaults) => {
                    // Validation in OptionArgument ensures there is one default value
                    match unsafe { defaults.get_unchecked(0).as_str() } {
                        "true" => "false".to_string(),
                        "false" => "true".to_string(),
                        _ => todo!("panic, default is invalid, this is a bug with the validation"),
                    }
                }
                None => todo!("panic, theres no defaults, this is a bug with the validation"),
            },
            _ => match raw_args.next() {
                Some(value) => value,
                None => todo!("error for no value provided"),
            },
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
                self.parse_option(&mut result, &raw_arg, &mut raw_args);
            } else {
                self.parse_positional(&mut result, &raw_arg);
            }
        }

        for positional in self.positionals {
            if positional.is_required == Some(true) {
                todo!("error for missing required positional")
            } else if let Some(defaults) = positional.defaults {
                if result.has_array(&positional.destination) {
                    // Array arguments still exist in the vectors,
                    // so they need to be skipped if they've already
                    // been parsed.
                    continue;
                }
                for default in defaults {
                    let parse_value = match ParseValue::from_value(positional.data_type, &default) {
                        Ok(parse_value) => parse_value,
                        Err(_) => {
                            todo!("panic, default is invalid, this is a bug with the validation")
                        }
                    };
                    let add_result = if positional.data_type.is_array() {
                        result.add_array_value(&positional.destination, parse_value)
                    } else {
                        result.add_single_value(&positional.destination, parse_value)
                    };
                    if let Err(_err) = add_result {
                        todo!("error for unable to add parse value")
                    }
                }
            }
        }

        for option in self.options {
            if option.is_required == Some(true) {
                todo!("error for missing required option")
            } else if let Some(defaults) = option.defaults {
                if result.has_array(&option.destination) {
                    // Array arguments still exist in the vectors,
                    // so they need to be skipped if they've already
                    // been parsed.
                    continue;
                }
                for default in defaults {
                    let parse_value = match ParseValue::from_value(option.data_type, &default) {
                        Ok(parse_value) => parse_value,
                        Err(_) => {
                            todo!("panic, default is invalid, this is a bug with the validation")
                        }
                    };
                    let add_result = if option.data_type.is_array() {
                        result.add_array_value(&option.destination, parse_value)
                    } else {
                        result.add_single_value(&option.destination, parse_value)
                    };
                    if let Err(_err) = add_result {
                        todo!("error for unable to add parse value")
                    }
                }
            }
        }

        result
    }
}
