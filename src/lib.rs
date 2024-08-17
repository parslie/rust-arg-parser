use argument::{option::OptionArgument, positional::PositionalArgument, DataType};

pub mod argument;

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
}
