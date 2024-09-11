use std::collections::{HashMap, VecDeque};

use argument::{option::OptionArgument, positional::PositionalArgument, DataType};

pub mod argument;

/// A collection of arguments and sub-parsers, configured
/// to be parsed a certain way.
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
                || !last_positional.default_values.is_some();
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
}
