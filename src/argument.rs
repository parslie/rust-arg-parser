pub mod option;
pub mod positional;

/// A representation of an argument's data type and plurality
/// (whether it's an array or not).
///
/// ```
/// let array_int32 = DataType::Int32(true);
/// let non_array_int32 = DataType::Int32(false);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Int32(bool),
    Float32(bool),
    String(bool),
    Bool(bool),
    Path(bool),
}

impl DataType {
    /// Checks whether or not the data type is an array.
    pub fn is_array(&self) -> bool {
        match self {
            DataType::Int32(value) => value.to_owned(),
            DataType::Float32(value) => value.to_owned(),
            DataType::String(value) => value.to_owned(),
            DataType::Bool(value) => value.to_owned(),
            DataType::Path(value) => value.to_owned(),
        }
    }
}
