use argument::{positional::PositionalArgument, DataType};

pub mod argument;

#[derive(Debug)]
pub struct Parser {
    positionals: Vec<PositionalArgument>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            positionals: Vec::new(),
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
}
