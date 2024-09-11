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
    short_name: Option<String>,
    long_name: Option<String>,
    destination: String,
    data_type: DataType,
    is_required: Option<bool>,
    default_values: Option<Vec<String>>,
}
