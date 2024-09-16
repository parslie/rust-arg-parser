use regex::Regex;

use crate::Parser;

use super::DataType;

/// Extracts the short and long names of an option argument.
///
/// Returns **(None, None)** if **names** parameter is invalidly formatted.
fn extract_names(names: &str) -> (Option<String>, Option<String>) {
    let both_name_re = Regex::new(r"^-[A-Za-z0-9] *, *--[A-Za-z0-9-]+$").unwrap();
    let short_name_re = Regex::new(r"^-[A-Za-z0-9]$").unwrap();
    let long_name_re = Regex::new(r"^--[A-Za-z0-9-]+$").unwrap();

    if both_name_re.is_match(names) {
        let stripped_names = names.replace(" ", "");
        let (short_name, long_name) = stripped_names.split_once(',').expect("checked with regex");
        let short_name = short_name[1..].to_string();
        let long_name = long_name[2..].to_string();
        (Some(short_name), Some(long_name))
    } else if short_name_re.is_match(names) {
        let short_name = names[1..].to_string();
        (Some(short_name), None)
    } else if long_name_re.is_match(names) {
        let long_name = names[2..].to_string();
        (None, Some(long_name))
    } else {
        (None, None)
    }
}

/// An argument that is parsed by inputting its name and value
/// in successive order.
///
/// ```
/// let parser = Parser::new();
/// parser.option("-n", "number", DataType::Int32(false));
/// // inputting "-n 32" sets the option's value to 32
/// ```
#[derive(Debug, Clone)]
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
