use crate::parsed::ParsedArgument;
use std::collections::HashMap;

mod parsed;
mod unparsed;

pub struct ParseResult {
    arguments: HashMap<String, ParsedArgument>,
}

impl ParseResult {
    pub unsafe fn get<T: Clone>(&self, key: &str) -> T {
        match self.arguments.get(key) {
            Some(arg) => unsafe { arg.value::<T>() },
            None => panic!("Could not find value for argument '{}'", key),
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.arguments.contains_key(key)
    }
}

pub struct Parser {}
