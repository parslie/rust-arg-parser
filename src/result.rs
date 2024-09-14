use std::{collections::HashMap, path::PathBuf, rc::Rc, str::FromStr};

use crate::argument::DataType;

/// A value resulting from parsing arguments.
#[derive(Debug)]
pub(crate) enum ParseValue {
    Int32(i32),
    Float32(f32),
    String(String),
    Bool(bool),
    Path(PathBuf),
}

impl ParseValue {
    /// Checks whether two parse values are of the same data type.
    fn is_same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (ParseValue::Int32(_), ParseValue::Int32(_)) => true,
            (ParseValue::Float32(_), ParseValue::Float32(_)) => true,
            (ParseValue::String(_), ParseValue::String(_)) => true,
            (ParseValue::Bool(_), ParseValue::Bool(_)) => true,
            (ParseValue::Path(_), ParseValue::Path(_)) => true,
            _ => false,
        }
    }

    /// Parses a value of a certain data type.
    ///
    /// # Errors
    /// - The value is not of the specified data type.
    pub(crate) fn from_value(value: &str, data_type: DataType) -> Result<Self, ()> {
        match data_type {
            DataType::Int32(_) => match value.parse::<i32>() {
                Ok(value) => Ok(ParseValue::Int32(value)),
                Err(_) => Err(()),
            },
            DataType::Float32(_) => match value.parse::<f32>() {
                Ok(value) => Ok(ParseValue::Float32(value)),
                Err(_) => Err(()),
            },
            DataType::String(_) => Ok(ParseValue::String(value.to_string())),
            DataType::Bool(_) => match value.parse::<bool>() {
                Ok(value) => Ok(ParseValue::Bool(value)),
                Err(_) => Err(()),
            },
            DataType::Path(_) => match PathBuf::from_str(value) {
                Ok(value) => Ok(ParseValue::Path(value)),
                Err(_) => Err(()),
            },
        }
    }
}

/// A collection of results from parsing arguments, which includes
/// errors and values associated with unique names.
#[derive(Debug)]
pub struct ParseResult {
    non_array_values: HashMap<String, ParseValue>,
    array_values: HashMap<String, Vec<ParseValue>>,
    child_result_name: Option<String>,
    child_result: Rc<Option<ParseResult>>,
    errors: Vec<String>,
}

impl ParseResult {
    /// Creates an empty parse result.
    pub(crate) fn new() -> Self {
        Self {
            non_array_values: HashMap::new(),
            array_values: HashMap::new(),
            child_result_name: None,
            child_result: Rc::new(None),
            errors: Vec::new(),
        }
    }

    /// Adds a non-array value to the result.
    ///
    /// # Errors
    /// - The key is already used for a non-array value, array value, or child parser.
    pub(crate) fn add_non_array_value(&mut self, key: &str, value: ParseValue) -> Result<(), ()> {
        let key_is_used =
            self.non_array_values.contains_key(key) || self.array_values.contains_key(key);

        if key_is_used {
            Err(())
        } else {
            self.non_array_values.insert(key.to_string(), value);
            Ok(())
        }
    }

    /// Adds an array value to the result.
    ///
    /// # Errors
    /// - The key is already used for a non-array value or child parser.
    /// - The value's data type isn't the same as the rest in the array.
    pub(crate) fn add_array_value(&mut self, key: &str, value: ParseValue) -> Result<(), ()> {
        let is_non_array_key = self.non_array_values.contains_key(key);

        if is_non_array_key {
            Err(())
        } else if let Some(array) = self.array_values.get_mut(key) {
            if let Some(first_value) = array.first() {
                if !first_value.is_same_type(&value) {
                    return Err(());
                }
            }
            array.push(value);
            Ok(())
        } else {
            let array = vec![value];
            self.array_values.insert(key.to_string(), array);
            Ok(())
        }
    }

    /// Sets the child result of the result.
    ///
    /// # Errors
    /// - A child result has already been set.
    pub(crate) fn set_child_result(&mut self, key: &str, result: ParseResult) -> Result<(), ()> {
        if self.child_result_name.is_some() {
            Err(())
        } else {
            self.child_result = Rc::new(Some(result));
            self.child_result_name = Some(key.to_string());
            Ok(())
        }
    }

    /// Adds an error to the result.
    pub(crate) fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
}
