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
}
