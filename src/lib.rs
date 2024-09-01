use argument::{option::OptionArgument, positional::PositionalArgument, DataType};
use result::{ParseResult, ParseValue};

pub mod argument;
pub mod result;

#[derive(Debug)]
pub struct Parser {
    positionals: Vec<PositionalArgument>,
    options: Vec<OptionArgument>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            positionals: Vec::new(),
            options: Vec::new(),
        }
    }

    pub fn positional(
        &mut self,
        destination: &str,
        data_type: DataType,
    ) -> &mut PositionalArgument {
        let positional = PositionalArgument::new(self, destination, data_type);
        self.positionals.push(positional);
        self.positionals.last_mut().expect("was just added")
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

    pub fn parse_args(&self) -> ParseResult {
        let args = std::env::args().skip(1); // First arg is always prog name

        let mut result = ParseResult::new();
        result.add_single_value("test_single", ParseValue::Int32(2));
        result.add_array_value("test_array", ParseValue::Float32(2.0));
        result.add_array_value("test_array", ParseValue::Float32(6.9));

        result
    }
}
