pub mod option;
pub mod positional;

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Int32(bool),
    Float32(bool),
    String(bool),
    Bool(bool),
    Path(bool),
}

impl DataType {
    pub fn is_array(&self) -> bool {
        let result = match self {
            DataType::Int32(is_array) => is_array,
            DataType::Float32(is_array) => is_array,
            DataType::String(is_array) => is_array,
            DataType::Bool(is_array) => is_array,
            DataType::Path(is_array) => is_array,
        };
        result.to_owned()
    }
}
