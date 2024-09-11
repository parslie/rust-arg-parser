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
    destination: String,
    data_type: DataType,
    is_required: Option<bool>,
    default_values: Option<Vec<String>>,
}
