pub mod option;
pub mod positional;

/// A representation of an argument's data type and plurality
/// (whether it's an array or not).
///
/// ```
/// let array_int32 = DataType::Int32(true);
/// let non_array_int32 = DataType::Int32(false);
/// ```
#[derive(Clone, Copy)]
pub enum DataType {
    Int32(bool),
    Float32(bool),
    String(bool),
    Bool(bool),
    Path(bool),
}
