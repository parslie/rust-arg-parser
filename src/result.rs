use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub(crate) enum ParseValue {
    Int32(i32),
    Float32(f32),
    String(String),
    Bool(bool),
    Path(PathBuf),
}

#[derive(Debug)]
pub struct ParseResult {
    single_values: HashMap<String, ParseValue>,
    array_values: HashMap<String, Vec<ParseValue>>,
}

impl ParseResult {
    pub(crate) fn new() -> Self {
        Self {
            single_values: HashMap::new(),
            array_values: HashMap::new(),
        }
    }

    pub(crate) fn add_single_value(
        &mut self,
        key: &str,
        parse_value: ParseValue,
    ) -> Result<(), String> {
        if self.single_values.contains_key(key) {
            Err(format!("a value for '{}' has already been parsed", key))
        } else {
            self.single_values.insert(key.to_string(), parse_value);
            Ok(())
        }
    }

    pub(crate) fn add_array_value(
        &mut self,
        key: &str,
        parse_value: ParseValue,
    ) -> Result<(), String> {
        if let Some(array) = self.array_values.get_mut(key) {
            // TODO: if array doesn't contain same type, return Err
            array.push(parse_value);
            Ok(())
        } else {
            // TODO: create new array
            let array = vec![parse_value];
            self.array_values.insert(key.to_string(), array);
            Ok(())
        }
    }

    pub fn get_single<T: Clone>(&self, key: &str) -> Option<T> {
        if !self.single_values.contains_key(key) {
            None
        } else {
            unsafe { Some(self.get_single_unchecked(key)) }
        }
    }

    pub fn get_array<T: Clone>(&self, key: &str) -> Option<Vec<T>> {
        if !self.array_values.contains_key(key) {
            None
        } else {
            unsafe { Some(self.get_array_unchecked(key)) }
        }
    }

    pub unsafe fn get_single_unchecked<T: Clone>(&self, key: &str) -> T {
        let generic_ptr = match &self.single_values[key] {
            ParseValue::Int32(value) => value as *const i32 as *const T,
            ParseValue::Float32(value) => value as *const f32 as *const T,
            ParseValue::String(value) => value as *const String as *const T,
            ParseValue::Bool(value) => value as *const bool as *const T,
            ParseValue::Path(value) => value as *const PathBuf as *const T,
        };
        (*generic_ptr).clone()
    }

    pub unsafe fn get_array_unchecked<T: Clone>(&self, key: &str) -> Vec<T> {
        let mut array = Vec::new();
        for parse_value in &self.array_values[key] {
            let generic_ptr = match parse_value {
                ParseValue::Int32(value) => value as *const i32 as *const T,
                ParseValue::Float32(value) => value as *const f32 as *const T,
                ParseValue::String(value) => value as *const String as *const T,
                ParseValue::Bool(value) => value as *const bool as *const T,
                ParseValue::Path(value) => value as *const PathBuf as *const T,
            };
            array.push((*generic_ptr).clone());
        }
        array
    }
}
