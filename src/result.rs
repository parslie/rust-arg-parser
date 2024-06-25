use std::collections::HashMap;

use crate::parsed::ParsedArgument;

#[derive(Debug)]
pub struct ParseResult {
    pub arguments: HashMap<String, ParsedArgument>,
    pub errors: Vec<String>,
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
