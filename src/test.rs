use std::panic;

use data::*;

mod data;

#[test]
fn valid_parser() {
    let parser = panic::catch_unwind(|| data::create_valid_parser());
    assert!(parser.is_ok());
    let parser = parser.unwrap();

    assert_eq!(parser.name.as_str(), PARSER_NAME);
    assert_eq!(parser.description.as_str(), PARSER_DESC);

    assert_eq!(parser.positionals.len(), VALID_POSITIONALS.len());
    assert_eq!(
        parser.options.len(),
        VALID_REQUIRED_OPTIONS.len() + VALID_OPTIONAL_OPTIONS.len() + VALID_DEFAULT_OPTIONS.len()
    );
}

#[test]
fn valid_arguments() {
    let parser = panic::catch_unwind(|| create_valid_parser());
    assert!(parser.is_ok());
    let parser = parser.unwrap();

    let args = create_valid_args();
    let parse_result = parser.parse_args_inner(args);
    assert!(parse_result.errors.is_empty());

    for (destination, _, _) in VALID_POSITIONALS {
        assert!(parse_result.has(destination));
    }

    for (destination, _, _, _) in VALID_REQUIRED_OPTIONS {
        assert!(parse_result.has(destination));
    }

    for (destination, _, _, _) in VALID_OPTIONAL_OPTIONS {
        assert!(!parse_result.has(destination));
    }

    for (destination, _, _, _, _) in VALID_DEFAULT_OPTIONS {
        assert!(parse_result.has(destination));
    }
}
