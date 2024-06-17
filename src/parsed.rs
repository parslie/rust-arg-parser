use std::path::PathBuf;

#[derive(Debug)]
pub enum ParsedArgument {
    Int32(i32),
    Float32(f32),
    String(String),
    Bool(bool),
    Path(PathBuf),
}

impl ParsedArgument {
    pub unsafe fn value<T: Clone>(&self) -> T {
        let generic_ptr = match self {
            ParsedArgument::Int32(value) => value as *const i32 as *const T,
            ParsedArgument::Float32(value) => value as *const f32 as *const T,
            ParsedArgument::String(value) => value as *const String as *const T,
            ParsedArgument::Bool(value) => value as *const bool as *const T,
            ParsedArgument::Path(value) => value as *const PathBuf as *const T,
        };
        unsafe { (*generic_ptr).clone() }
    }
}
