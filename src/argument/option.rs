use crate::Parser;

use super::DataType;

fn extract_names(names: &str) -> (Option<String>, Option<String>) {
    todo!("create function for extracting short and long names");
}

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
    /// - ...if the names aren't of the forms "-s", "--long-name", or "-s, --long-name".
    /// - ...if the desination is occupied.
    /// - ...if either name is occupied.
    pub(crate) fn new(
        parser: &Parser,
        names: &str,
        destination: &str,
        data_type: DataType,
    ) -> Self {
        if parser.is_destination_occupied(destination) {
            panic!("the destination '{}' is already occupied", destination);
        }

        let (short_name, long_name) = match extract_names(names) {
            (None, None) => panic!(
                "names '{}' for argument '{}' were invalidly formatted",
                names, destination
            ),
            value => value,
        };

        if let Some(short_name) = &short_name {
            if parser.is_short_name_occupied(short_name) {
                panic!("the short name '{}' is already occupied", short_name);
            }
        }
        if let Some(long_name) = &long_name {
            if parser.is_long_name_occupied(long_name) {
                panic!("the long name '{}' is already occupied", long_name);
            }
        }

        OptionArgument {
            short_name,
            long_name,
            destination: destination.to_string(),
            data_type,
            is_required: None,
            default_values: None,
        }
    }
}
