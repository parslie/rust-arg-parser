use std::panic::catch_unwind;

use crate::{argument::DataType, Parser};

struct Input<'a> {
    destination: &'a str,
    data_type: DataType,
    is_required: Option<bool>,
    defaults: Option<&'a [&'a str]>,
}

fn create_parser_with_inputs(inputs: &[Input]) -> Parser {
    let mut parser = Parser::new();
    for input in inputs {
        let positional = parser.positional(input.destination, input.data_type);
        if let Some(is_required) = input.is_required {
            positional.is_required(is_required);
        }
        if let Some(defaults) = &input.defaults {
            positional.defaults(defaults);
        }
    }
    parser
}

mod valid {
    use super::*;

    #[test]
    fn unspecified_requirement() {
        let inputs: [Input; 5] = [
            Input {
                destination: "pos_1",
                data_type: DataType::Int32(false),
                is_required: None,
                defaults: None,
            },
            Input {
                destination: "pos_2",
                data_type: DataType::Float32(false),
                is_required: None,
                defaults: None,
            },
            Input {
                destination: "pos_3",
                data_type: DataType::String(false),
                is_required: None,
                defaults: None,
            },
            Input {
                destination: "pos_4",
                data_type: DataType::Bool(false),
                is_required: None,
                defaults: None,
            },
            Input {
                destination: "pos_5",
                data_type: DataType::Path(false),
                is_required: None,
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_ok());
        let parser = result.expect("just checked that it's Ok(_)");
        assert_eq!(parser.positionals.len(), inputs.len());
    }

    #[test]
    fn specified_requirement() {
        let inputs: [Input; 5] = [
            Input {
                destination: "pos_1",
                data_type: DataType::Int32(false),
                is_required: Some(true),
                defaults: None,
            },
            Input {
                destination: "pos_2",
                data_type: DataType::Float32(false),
                is_required: Some(true),
                defaults: None,
            },
            Input {
                destination: "pos_3",
                data_type: DataType::String(false),
                is_required: Some(true),
                defaults: None,
            },
            Input {
                destination: "pos_4",
                data_type: DataType::Bool(false),
                is_required: Some(true),
                defaults: None,
            },
            Input {
                destination: "pos_5",
                data_type: DataType::Path(false),
                is_required: Some(true),
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_ok());
        let parser = result.expect("just checked that it's Ok(_)");
        assert_eq!(parser.positionals.len(), inputs.len());
    }

    #[test]
    fn last_optional() {
        let inputs: [Input; 2] = [
            Input {
                destination: "pos_1",
                data_type: DataType::Path(false),
                is_required: None,
                defaults: None,
            },
            Input {
                destination: "pos_2",
                data_type: DataType::Path(false),
                is_required: Some(false),
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_ok());
        let parser = result.expect("just checked that it's Ok(_)");
        assert_eq!(parser.positionals.len(), inputs.len());
    }

    #[test]
    fn last_default() {
        let inputs: [Input; 2] = [
            Input {
                destination: "pos_1",
                data_type: DataType::Path(false),
                is_required: None,
                defaults: None,
            },
            Input {
                destination: "pos_2",
                data_type: DataType::Path(false),
                is_required: None,
                defaults: Some(&["./README.md"]),
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_ok());
        let parser = result.expect("just checked that it's Ok(_)");
        assert_eq!(parser.positionals.len(), inputs.len());
    }

    #[test]
    fn last_array() {
        let inputs: [Input; 2] = [
            Input {
                destination: "pos_1",
                data_type: DataType::Path(false),
                is_required: None,
                defaults: None,
            },
            Input {
                destination: "pos_2",
                data_type: DataType::Path(true),
                is_required: None,
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_ok());
        let parser = result.expect("just checked that it's Ok(_)");
        assert_eq!(parser.positionals.len(), inputs.len());
    }

    #[ignore = "parsing has not been implemented yet, so can't be tested"]
    #[test]
    fn correct_default_types() {
        todo!();
    }
}

mod invalid {
    use super::*;

    #[test]
    fn duplicate_destination() {
        let inputs: [Input; 2] = [
            Input {
                destination: "same_destination",
                data_type: DataType::Path(false),
                is_required: None,
                defaults: None,
            },
            Input {
                destination: "same_destination",
                data_type: DataType::Path(false),
                is_required: None,
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn non_last_optional() {
        let inputs: [Input; 2] = [
            Input {
                destination: "badly_placed_optional",
                data_type: DataType::String(false),
                is_required: Some(false),
                defaults: None,
            },
            Input {
                destination: "other_positional",
                data_type: DataType::String(false),
                is_required: None,
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn non_last_default() {
        let inputs: [Input; 2] = [
            Input {
                destination: "badly_placed_optional",
                data_type: DataType::Bool(false),
                is_required: None,
                defaults: Some(&["true"]),
            },
            Input {
                destination: "other_positional",
                data_type: DataType::Bool(false),
                is_required: None,
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn non_last_array() {
        let inputs: [Input; 2] = [
            Input {
                destination: "badly_placed_array",
                data_type: DataType::String(true),
                is_required: None,
                defaults: None,
            },
            Input {
                destination: "other_positional",
                data_type: DataType::String(false),
                is_required: None,
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn required_and_default() {
        let inputs: [Input; 1] = [Input {
            destination: "conflicting_requirement_and_default",
            data_type: DataType::Float32(false),
            is_required: Some(true),
            defaults: Some(&["420.69"]),
        }];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn too_many_defaults() {
        let inputs: [Input; 1] = [Input {
            destination: "non_array_with_many_defaults",
            data_type: DataType::Int32(false),
            is_required: None,
            defaults: Some(&["42", "64"]),
        }];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[ignore = "parsing has not been implemented yet, so can't be tested"]
    #[test]
    fn wrong_default_types() {
        todo!();
    }
}
