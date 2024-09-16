use std::collections::{HashMap, VecDeque};

use argument::{option::OptionArgument, positional::PositionalArgument, DataType};
use result::{ParseResult, ParseValue};

pub mod argument;
pub mod result;

/// A collection of arguments and sub-parsers, configured
/// to be parsed a certain way.
#[derive(Debug)]
pub struct Parser {
    positionals: VecDeque<PositionalArgument>,
    options: Vec<OptionArgument>,
    parent_parser: Option<*const Self>,
    child_parsers: HashMap<String, Self>,
}

impl Parser {
    /// Creates a parser with no arguments or sub-parsers.
    pub fn new() -> Self {
        Self {
            positionals: VecDeque::new(),
            options: Vec::new(),
            parent_parser: None,
            child_parsers: HashMap::new(),
        }
    }

    // TODO: given the newly thought-of structure for parse result
    //       this only needs to check the current parser.
    /// Checks whether a destination is occupied by an argument
    /// that belongs to the parser or any of its parent or child parsers.
    fn is_destination_occupied(&self, destination: &str) -> bool {
        for positional in &self.positionals {
            if positional.destination.as_str() == destination {
                return true;
            }
        }
        for option in &self.options {
            if option.destination.as_str() == destination {
                return true;
            }
        }

        let mut parent_parser_option = self.parent_parser;
        while let Some(parent_parser_ptr) = parent_parser_option {
            let parent_parser = unsafe { parent_parser_ptr.as_ref().expect("should exists") };
            for positional in &parent_parser.positionals {
                if positional.destination.as_str() == destination {
                    return true;
                }
            }
            for option in &parent_parser.options {
                if option.destination.as_str() == destination {
                    return true;
                }
            }
            parent_parser_option = parent_parser.parent_parser;
        }

        let mut child_parsers: Vec<&Self> = self.child_parsers.values().collect();
        while let Some(child_parser) = child_parsers.pop() {
            for positional in &child_parser.positionals {
                if positional.destination.as_str() == destination {
                    return true;
                }
            }
            for option in &child_parser.options {
                if option.destination.as_str() == destination {
                    return true;
                }
            }
            child_parsers.extend(child_parser.child_parsers.values())
        }

        false
    }

    /// Checks whether a short name is occupied by an argument
    /// that belongs to the parser.
    fn is_short_name_occupied(&self, short_name: &str) -> bool {
        for option in &self.options {
            if option.short_name == Some(short_name.to_string()) {
                return true;
            }
        }
        false
    }

    /// Checks whether a long name is occupied by an argument
    /// that belongs to the parser.
    fn is_long_name_occupied(&self, long_name: &str) -> bool {
        for option in &self.options {
            if option.long_name == Some(long_name.to_string()) {
                return true;
            }
        }
        false
    }

    /// Adds a sub-parser that is parsed by inputting its name after
    /// all other arguments.
    ///
    /// # Panics...
    /// - ...if the name is empty or occupied.
    /// - ...if the last positional argument is optional or an array.
    pub fn sub_parser(&mut self, name: &str) -> &mut Self {
        if name.is_empty() {
            panic!("the name of sub-parsers can't be empty");
        } else if self.child_parsers.contains_key(name) {
            panic!("a sub-parser with the name '{}' already exists", name);
        } else if let Some(last_positional) = self.positionals.back() {
            let is_optional = last_positional.is_required == Some(false)
                || last_positional.default_values.is_some();
            let is_array = last_positional.data_type.is_array();

            if is_optional {
                panic!(
                    "sub-parser '{}' couldn't be added, because the last positional is optional",
                    name
                );
            } else if is_array {
                panic!(
                    "sub-parser '{}' couldn't be added, because the last positional is an array",
                    name
                );
            }
        }

        let mut child_parser = Self::new();
        child_parser.parent_parser = Some(self as *const Self);
        self.child_parsers.insert(name.to_string(), child_parser);
        self.child_parsers.get_mut(name).expect("just added it")
    }

    /// Adds a default-configured positional argument.
    ///
    /// # Panics...
    /// - ...if the desination is occupied.
    pub fn positional(
        &mut self,
        destination: &str,
        data_type: DataType,
    ) -> &mut PositionalArgument {
        let positional = PositionalArgument::new(self, destination, data_type);
        self.positionals.push_back(positional);
        self.positionals.back_mut().expect("just added it")
    }

    /// Adds a default-configured option argument.
    ///
    /// # Panics...
    /// - ...if the names aren't of the forms "-s", "--long-name", or "-s, --long-name".
    /// - ...if the desination is occupied.
    /// - ...if either name is occupied.
    pub fn option(
        &mut self,
        names: &str,
        destination: &str,
        data_type: DataType,
    ) -> &mut OptionArgument {
        let option = OptionArgument::new(self, names, destination, data_type);
        self.options.push(option);
        self.options.last_mut().expect("just added it")
    }

    /// Parses strings based on the parser's argument and sub-parsers.
    ///
    /// If None is provided the arguments that this program started with will be used (AKA argv).
    pub fn parse(&self, args: Option<Vec<String>>) -> ParseResult {
        let args = match args {
            Some(value) => value,
            None => std::env::args().skip(1).collect(), // Skip cause first argument is prog name
        };

        let result = self.parse_inner(args);
        if result.has_errors() {
            result.print_errors();
            std::process::exit(1);
        }

        result
    }

    /// Parses strings based on the parser's argument and sub-parsers.
    ///
    /// This function is only to be called by parser instances.
    fn parse_inner(&self, args: Vec<String>) -> ParseResult {
        let mut args = args.into_iter();
        let mut result = ParseResult::new();
        let mut positionals = self.positionals.clone();
        let mut options = self.options.clone();

        while let Some(arg) = args.next() {
            let is_option = arg.starts_with('-');
            let has_positionals = !positionals.is_empty();
            let has_child_parsers = !self.child_parsers.is_empty();

            // TODO: organize this after documentation is done
            if is_option {
                let mut option_idx = None;
                for (idx, option) in options.iter().enumerate() {
                    let trimmed_name = Some(arg.trim_start_matches('-'));
                    if option.short_name.as_deref() == trimmed_name
                        || option.long_name.as_deref() == trimmed_name
                    {
                        option_idx = Some(idx);
                        break;
                    }
                }

                let option = match option_idx {
                    Some(option_idx) => options.remove(option_idx),
                    None => {
                        let error = format!("no matching option found for '{}'", &arg);
                        result.add_error(error);
                        continue; // Continue to find more errors
                    }
                };

                let value = match args.next() {
                    Some(value) => value,
                    None => {
                        let error = format!("no value provided for '{}'", &arg);
                        result.add_error(error);
                        break; // No arguments left, since we got None from args.next()
                    }
                };

                let parse_value = match ParseValue::from_value(&value, option.data_type) {
                    Ok(parse_value) => parse_value,
                    Err(_) => {
                        let error = format!("'{}' is not of the correct data type", &value);
                        result.add_error(error);
                        continue; // Continue to find more errors
                    }
                };

                let add_result = if option.data_type.is_array() {
                    result.add_array_value(&option.destination, parse_value)
                } else {
                    result.add_non_array_value(&option.destination, parse_value)
                };

                if let Err(_) = add_result {
                    let error = format!(
                        "'{}' has already been parsed (this is a bug, report if it happens)",
                        &option.destination
                    );
                    result.add_error(error);
                    continue; // Continue to find more errors
                }

                if option.data_type.is_array() {
                    options.push(option);
                }
            } else if has_positionals {
                let positional = positionals.pop_front().expect("has checked if non-empty");
                let parse_value = match ParseValue::from_value(&arg, positional.data_type) {
                    Ok(parse_value) => parse_value,
                    Err(_) => {
                        let error = format!("'{}' is not of the correct data type", &arg);
                        result.add_error(error);
                        continue; // Continue to find more errors
                    }
                };

                let add_result = if positional.data_type.is_array() {
                    result.add_array_value(&positional.destination, parse_value)
                } else {
                    result.add_non_array_value(&positional.destination, parse_value)
                };

                if let Err(_) = add_result {
                    let error = format!(
                        "'{}' has already been parsed (this is a bug, report if it happens)",
                        &positional.destination
                    );
                    result.add_error(error);
                    continue; // Continue to find more errors
                }

                if positional.data_type.is_array() {
                    positionals.push_back(positional);
                }
            } else if has_child_parsers {
                match self.child_parsers.get(&arg) {
                    Some(child_parser) => {
                        let args = args.collect();
                        if let Err(_) =
                            result.set_child_result(&arg, child_parser.parse_inner(args))
                        {
                            let error = format!("sub-parser '{}' doesn't exist", arg);
                            result.add_error(error);
                        }
                    }
                    None => {
                        let error = format!("sub-parser '{}' doesn't exist", arg);
                        result.add_error(error);
                    }
                }
                break; // No arguments left, they have been propogated
            } else {
                let error = format!("unable to parse '{}', no positionals left", arg);
                result.add_error(error);
                continue; // Continue to find more errors
            }
        }

        for positional in positionals {
            if result.contains_key(&positional.destination) {
                continue; // Already been parsed
            }

            if positional.is_required != Some(false) {
                let error = format!(
                    "no value provided for positional '{}'",
                    &positional.destination
                );
                result.add_error(error);
                continue;
            } else if let Some(default_values) = positional.default_values {
                for default_value in default_values {
                    let parse_value = match ParseValue::from_value(
                        &default_value,
                        positional.data_type,
                    ) {
                        Ok(parse_value) => parse_value,
                        Err(_) => {
                            let error = format!(
                                "'{}' is not of the correct data type (this is a bug, report if it happens)",
                                &default_value
                            );
                            result.add_error(error);
                            continue; // Continue to find more errors
                        }
                    };

                    let add_result = if positional.data_type.is_array() {
                        result.add_array_value(&positional.destination, parse_value)
                    } else {
                        result.add_non_array_value(&positional.destination, parse_value)
                    };

                    if let Err(_) = add_result {
                        let error = format!(
                            "'{}' has already been parsed (this is a bug, report if it happens)",
                            &positional.destination
                        );
                        result.add_error(error);
                        continue; // Continue to find more errors
                    }
                }
            }
        }

        for option in options {
            if result.contains_key(&option.destination) {
                continue; // Already been parsed
            }

            if option.is_required == Some(true) {
                let error = format!("no value provided for option '{}'", &option.destination);
                result.add_error(error);
                continue;
            } else if let Some(default_values) = option.default_values {
                for default_value in default_values {
                    let parse_value = match ParseValue::from_value(&default_value, option.data_type)
                    {
                        Ok(parse_value) => parse_value,
                        Err(_) => {
                            let error = format!(
                                "'{}' is not of the correct data type (this is a bug, report if it happens)",
                                &default_value
                            );
                            result.add_error(error);
                            continue; // Continue to find more errors
                        }
                    };

                    let add_result = if option.data_type.is_array() {
                        result.add_array_value(&option.destination, parse_value)
                    } else {
                        result.add_non_array_value(&option.destination, parse_value)
                    };

                    if let Err(_) = add_result {
                        let error = format!(
                            "'{}' has already been parsed (this is a bug, report if it happens)",
                            &option.destination
                        );
                        result.add_error(error);
                        continue; // Continue to find more errors
                    }
                }
            }
        }

        if !self.child_parsers.is_empty() && result.get_sub_parser_name().is_none() {
            let error = "no sub-parser chosen".to_string();
            result.add_error(error);
        }

        result
    }
}
