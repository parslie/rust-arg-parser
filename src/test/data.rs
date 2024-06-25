use crate::unparsed::DataType;
use crate::Parser;

pub const PARSER_NAME: &str = "test_prog";
pub const PARSER_DESC: &str = "A test program.";

pub const VALID_INT32: &str = "69";
pub const VALID_FLOAT32: &str = "42.0";
pub const VALID_STRING: &str = "invalid string";
pub const VALID_BOOL: &str = "true";
pub const VALID_PATH: &str = "bee_movie_script.txt";

pub const VALID_POSITIONALS: [(&str, DataType, &str); 5] = [
    ("pos_1", DataType::Int32, "an integer positional"),
    ("pos_2", DataType::Float32, "a float positional"),
    ("pos_3", DataType::String, "a string positional"),
    ("pos_4", DataType::Bool, "a bool positional"),
    ("pos_5", DataType::Path, "a path positional"),
];

pub const VALID_REQUIRED_OPTIONS: [(&str, DataType, &str, &str); 4] = [
    ("req_1", DataType::Int32, "req-1", "an integer optional"),
    ("req_2", DataType::Float32, "req-2", "a float optional"),
    ("req_3", DataType::String, "req-3", "a string optional"),
    ("req_4", DataType::Path, "req-4", "a path optional"),
];

pub const VALID_OPTIONAL_OPTIONS: [(&str, DataType, &str, &str); 4] = [
    ("opt_1", DataType::Int32, "opt-1", "an integer optional"),
    ("opt_2", DataType::Float32, "opt-2", "a float optional"),
    ("opt_3", DataType::String, "opt-3", "a string optional"),
    ("opt_4", DataType::Path, "opt-4", "a path optional"),
];

pub const VALID_DEFAULT_OPTIONS: [(&str, DataType, &str, &str, &str); 5] = [
    (
        "def_1",
        DataType::Int32,
        "def-1",
        VALID_INT32,
        "an integer optional",
    ),
    (
        "def_2",
        DataType::Float32,
        "def-2",
        VALID_FLOAT32,
        "a float optional",
    ),
    (
        "def_3",
        DataType::String,
        "def-3",
        VALID_STRING,
        "a string optional",
    ),
    (
        "def_4",
        DataType::Bool,
        "def-4",
        VALID_BOOL,
        "a bool optional",
    ),
    (
        "def_5",
        DataType::Path,
        "def-5",
        VALID_PATH,
        "a path optional",
    ),
];

pub fn create_valid_parser() -> Parser {
    let mut parser = Parser::new(PARSER_NAME, PARSER_DESC);

    for (destination, data_type, _) in VALID_POSITIONALS {
        parser
            .argument(destination, data_type)
            //.description(description)
            .build();
    }

    for (destination, data_type, long_name, _) in VALID_REQUIRED_OPTIONS {
        parser
            .argument(destination, data_type)
            //.description(description)
            .long_name(long_name)
            .required(true)
            .build();
    }

    for (destination, data_type, long_name, _) in VALID_OPTIONAL_OPTIONS {
        parser
            .argument(destination, data_type)
            //.description(description)
            .long_name(long_name)
            .required(false)
            .build();
    }

    for (destination, data_type, long_name, default_value, _) in VALID_DEFAULT_OPTIONS {
        parser
            .argument(destination, data_type)
            //.description(description)
            .long_name(long_name)
            .default(default_value)
            .build();
    }

    parser
}

pub fn create_valid_args() -> Vec<String> {
    let mut args = Vec::new();

    for (_, data_type, long_name, _) in &VALID_REQUIRED_OPTIONS {
        args.push(format!("--{}", &long_name));
        let value = match data_type {
            DataType::Int32 => VALID_INT32,
            DataType::Float32 => VALID_FLOAT32,
            DataType::String => VALID_STRING,
            DataType::Bool => continue,
            DataType::Path => VALID_PATH,
        };
        args.push(value.to_string());
    }

    for (_, data_type, _) in &VALID_POSITIONALS {
        let value = match data_type {
            DataType::Int32 => VALID_INT32,
            DataType::Float32 => VALID_FLOAT32,
            DataType::String => VALID_STRING,
            DataType::Bool => VALID_BOOL,
            DataType::Path => VALID_PATH,
        };
        args.push(value.to_string());
    }

    args
}
