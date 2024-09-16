use crate::Parser;

use super::DataType;

/// An argument that is parsed by inputting its value in the order
/// it was added relative to all other positional arguments.
///
/// ```
/// let parser = Parser::new();
/// parser.positional("number_1", DataType::Int32(false));
/// parser.positional("number_2", DataType::Int32(false));
/// // inputting "420 69" sets the value of number_1 to 420 and
/// // the value of number_2 to 69
/// ```
#[derive(Debug, Clone)]
pub struct PositionalArgument {
    pub(crate) destination: String,
    pub(crate) data_type: DataType,
    pub(crate) is_required: Option<bool>,
    pub(crate) default_values: Option<Vec<String>>,
}

impl PositionalArgument {
    /// Creates a default-configured positional argument.
    ///
    /// # Panics...
    /// - ...if the desination is occupied.
    pub(crate) fn new(parser: &Parser, destination: &str, data_type: DataType) -> Self {
        if parser.is_destination_occupied(destination) {
            panic!("the destination '{}' is already occupied", destination);
        }
        PositionalArgument {
            destination: destination.to_string(),
            data_type,
            is_required: None,
            default_values: None,
        }
    }

    /// Sets whether the argument is required or optional.
    ///
    /// Panics...
    /// - ...if the argument is set to required despite having a default value.
    pub fn is_required(&mut self, is_required: bool) -> &mut Self {
        if self.default_values.is_some() && is_required {
            panic!(
                "'{}' can't be required and have a default value",
                &self.destination
            );
        }

        self.is_required = Some(is_required);
        self
    }

    /// Sets the default value of the argument.
    ///
    /// Panics...
    /// - ...if the argument is required.
    pub fn default_value(&mut self, default_value: &str) -> &mut Self {
        if self.is_required == Some(true) {
            panic!(
                "'{}' can't have a default value and be required",
                &self.destination
            );
        }

        self.default_values = Some(vec![default_value.to_string()]);
        self
    }

    /// Sets the default values of the argument.
    ///
    /// Panics...
    /// - ...if the argument is required.
    /// - ...if the argument is not an array.
    pub fn default_values(&mut self, default_values: &[&str]) -> &mut Self {
        if self.is_required == Some(true) {
            panic!(
                "'{}' can't have a default value and be required",
                &self.destination
            );
        } else if !self.data_type.is_array() {
            panic!(
                "'{}' is not an array and can't have multiple default values",
                &self.destination
            );
        }

        let default_values = default_values
            .iter()
            .map(|value| value.to_string())
            .collect();
        self.default_values = Some(default_values);
        self
    }
}
