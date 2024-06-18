use std::panic;

use crate::{
    unparsed::{DataType, Optionality},
    Parser,
};

const PARSER_NAME: &str = "test_prog";
const PARSER_DESC: &str = "A test program.";

const REQ_POSITIONALS: [(&str, DataType); 2] = [
    ("req_pos_1", DataType::Int32),
    ("req_pos_2", DataType::Path),
];

const OPT_POSITIONALS: [(&str, DataType); 2] = [
    ("opt_pos_1", DataType::Int32),
    ("opt_pos_2", DataType::Path),
];

const DEF_POSITIONALS: [(&str, DataType, &str); 2] = [
    ("def_pos_1", DataType::Bool, "true"),
    ("def_pos_2", DataType::Float32, "2.0"),
];

const REQ_OPTIONS: [(&str, &str, DataType); 2] = [
    ("req_opt_1", "-r,--reqopt1", DataType::String),
    ("req_opt_2", "--reqopt2", DataType::Float32),
];

const OPT_OPTIONS: [(&str, &str, DataType); 2] = [
    ("opt_opt_1", "-o", DataType::String),
    ("opt_opt_2", "--optopt2", DataType::Float32),
];

const DEF_OPTIONS: [(&str, &str, DataType, &str); 2] = [
    ("def_opt_1", "-d,--defopt1", DataType::Bool, "true"),
    ("def_opt_2", "--defopt2", DataType::Int32, "-4"),
];

const VALID_STRING: &str = "valid string";
const VALID_INT32: &str = "69";
const VALID_FLOAT32: &str = "42.0";
const VALID_BOOL: &str = "true";
const VALID_PATH: &str = "textfile.txt";

fn create_parser() -> Parser {
    let mut parser = Parser::new(PARSER_NAME, PARSER_DESC);

    for (dest, data_type) in REQ_POSITIONALS {
        parser.add_positional(dest, data_type, Optionality::Required);
    }

    for (dest, data_type) in OPT_POSITIONALS {
        parser.add_positional(dest, data_type, Optionality::Optional);
    }

    for (dest, data_type, default_value) in DEF_POSITIONALS {
        parser.add_positional(
            dest,
            data_type,
            Optionality::Default(default_value.to_string()),
        );
    }

    for (dest, name, data_type) in REQ_OPTIONS {
        parser.add_option(dest, name, data_type, Optionality::Required);
    }

    for (dest, name, data_type) in OPT_OPTIONS {
        parser.add_option(dest, name, data_type, Optionality::Optional);
    }

    for (dest, name, data_type, default_value) in DEF_OPTIONS {
        parser.add_option(
            dest,
            name,
            data_type,
            Optionality::Default(default_value.to_string()),
        );
    }

    parser
}

#[test]
fn create_valid_parser() {
    let parser = panic::catch_unwind(|| create_parser());
    assert!(parser.is_ok());
    let parser = parser.unwrap();

    assert_eq!(parser.name.as_str(), PARSER_NAME);
    assert_eq!(parser.description.as_str(), PARSER_DESC);

    assert_eq!(
        parser.positionals.len(),
        REQ_POSITIONALS.len() + OPT_POSITIONALS.len() + DEF_POSITIONALS.len()
    );

    assert_eq!(
        parser.options.len(),
        REQ_OPTIONS.len() + OPT_OPTIONS.len() + DEF_OPTIONS.len()
    );
}

#[test]
fn parse_valid_args() {
    let parser = panic::catch_unwind(|| create_parser());
    assert!(parser.is_ok());
    let parser = parser.unwrap();

    let mut args = Vec::new();
    for (_dest, data_type) in REQ_POSITIONALS {
        let arg = match data_type {
            DataType::Int32 => VALID_INT32,
            DataType::Float32 => VALID_FLOAT32,
            DataType::String => VALID_STRING,
            DataType::Bool => VALID_BOOL,
            DataType::Path => VALID_PATH,
        };
        args.push(arg.to_string());
    }
    for (_dest, name, data_type) in REQ_OPTIONS {
        let arg = match data_type {
            DataType::Int32 => VALID_INT32,
            DataType::Float32 => VALID_FLOAT32,
            DataType::String => VALID_STRING,
            DataType::Bool => VALID_BOOL,
            DataType::Path => VALID_PATH,
        };
        let name = name.split(',').next().unwrap();
        args.push(name.to_string());
        if data_type != DataType::Bool {
            args.push(arg.to_string());
        }
    }

    let parse_result = parser.parse_args_inner(args);
    assert!(parse_result.errors.is_empty());

    for (dest, _data_type) in REQ_POSITIONALS {
        // TODO: could test values are as they should be
        assert!(parse_result.has(dest));
    }
    for (dest, _data_type) in OPT_POSITIONALS {
        assert!(!parse_result.has(dest));
    }
    for (dest, _data_type, _value) in DEF_POSITIONALS {
        // TODO: could test values are as they should be
        assert!(parse_result.has(dest));
    }

    for (dest, _name, _data_type) in REQ_OPTIONS {
        // TODO: could test values are as they should be
        assert!(parse_result.has(dest));
    }
    for (dest, _name, _data_type) in OPT_OPTIONS {
        assert!(!parse_result.has(dest));
    }
    for (dest, _name, _data_type, _value) in DEF_OPTIONS {
        // TODO: could test values are as they should be
        assert!(parse_result.has(dest));
    }
}
