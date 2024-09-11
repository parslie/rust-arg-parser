use crate::Parser;

use super::DataType;

/// An argument that is parsed by inputting its name and value
/// in successive order.
///
/// ```
/// let parser = Parser::new();
/// parser.option("-n", "number", DataType::Int32(false));
/// // inputting "-n 32" sets the option's value to 32
/// ```
pub struct OptionArgument {
    pub(crate) short_name: Option<String>,
    pub(crate) long_name: Option<String>,
    pub(crate) destination: String,
    pub(crate) data_type: DataType,
    pub(crate) is_required: Option<bool>,
    pub(crate) default_values: Option<Vec<String>>,
}

impl OptionArgument {
    /// Creates a default-configured option argument.
    ///
    /// # Panics...
    /// - ...if the desination is occupied.
    /// - ...if either name is occupied.
    pub(crate) fn new(
        parser: &Parser,
        names: &str,
        destination: &str,
        data_type: DataType,
    ) -> Self {
        todo!("create function for checking for occupied destinations");
        todo!("create function for extracting short and long names");
        todo!("create function for checking for occupied names");
        todo!("return new OptionArgument");
    }
}
