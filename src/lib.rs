use std::collections::{HashMap, VecDeque};

use argument::{option::OptionArgument, positional::PositionalArgument};

pub mod argument;

/// A collection of arguments and sub-parsers, configured
/// to be parsed a certain way.
pub struct Parser {
    positionals: VecDeque<PositionalArgument>,
    options: Vec<OptionArgument>,
    parent_parser: Option<*const Self>,
    child_parsers: HashMap<String, Self>,
}
