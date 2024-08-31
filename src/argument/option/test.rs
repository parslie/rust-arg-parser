use crate::{argument::DataType, Parser};

struct Input<'a> {
    names: &'a str,
    destination: &'a str,
    data_type: DataType,
    is_required: Option<bool>,
    defaults: Option<&'a [&'a str]>,
}

fn create_parser_with_inputs(inputs: &[Input]) -> Parser {
    let mut parser = Parser::new();
    for input in inputs {
        let option = parser.option(input.names, input.destination, input.data_type);
        if let Some(is_required) = input.is_required {
            option.is_required(is_required);
        }
        if let Some(defaults) = &input.defaults {
            option.defaults(defaults);
        }
    }
    parser
}

mod valid {
    use std::panic::catch_unwind;

    use super::*;

    #[test]
    fn unspecified_requirement() {
        let inputs = [
            Input {
                names: "--int",
                destination: "a",
                data_type: DataType::Int32(false),
                is_required: None,
                defaults: None,
            },
            Input {
                names: "--float",
                destination: "b",
                data_type: DataType::Float32(false),
                is_required: None,
                defaults: None,
            },
            Input {
                names: "--str",
                destination: "c",
                data_type: DataType::String(false),
                is_required: None,
                defaults: None,
            },
            Input {
                names: "--bool",
                destination: "d",
                data_type: DataType::Bool(false),
                is_required: None,
                defaults: None,
            },
            Input {
                names: "--path",
                destination: "e",
                data_type: DataType::Path(false),
                is_required: None,
                defaults: None,
            },
            Input {
                names: "--array",
                destination: "f",
                data_type: DataType::Path(true),
                is_required: None,
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_ok());
        let parser = result.expect("just checked that it's Ok(_)");
        assert_eq!(parser.options.len(), inputs.len());
    }

    #[test]
    fn specified_required() {
        let inputs = [
            Input {
                names: "--req-int",
                destination: "a",
                data_type: DataType::Int32(false),
                is_required: Some(true),
                defaults: None,
            },
            Input {
                names: "--req-float",
                destination: "b",
                data_type: DataType::Float32(false),
                is_required: Some(true),
                defaults: None,
            },
            Input {
                names: "--req-str",
                destination: "c",
                data_type: DataType::String(false),
                is_required: Some(true),
                defaults: None,
            },
            Input {
                names: "--req-bool",
                destination: "d",
                data_type: DataType::Bool(false),
                is_required: Some(true),
                defaults: None,
            },
            Input {
                names: "--req-path",
                destination: "e",
                data_type: DataType::Path(false),
                is_required: Some(true),
                defaults: None,
            },
            Input {
                names: "--req-array",
                destination: "f",
                data_type: DataType::Path(true),
                is_required: Some(true),
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_ok());
        let parser = result.expect("just checked that it's Ok(_)");
        assert_eq!(parser.options.len(), inputs.len());
    }

    #[test]
    fn specified_optional() {
        let inputs = [
            Input {
                names: "--opt-int",
                destination: "a",
                data_type: DataType::Int32(false),
                is_required: Some(false),
                defaults: None,
            },
            Input {
                names: "--opt-float",
                destination: "b",
                data_type: DataType::Float32(false),
                is_required: Some(false),
                defaults: None,
            },
            Input {
                names: "--opt-str",
                destination: "c",
                data_type: DataType::String(false),
                is_required: Some(false),
                defaults: None,
            },
            Input {
                names: "--opt-bool",
                destination: "d",
                data_type: DataType::Bool(false),
                is_required: Some(false),
                defaults: None,
            },
            Input {
                names: "--opt-path",
                destination: "e",
                data_type: DataType::Path(false),
                is_required: Some(false),
                defaults: None,
            },
            Input {
                names: "--opt-array",
                destination: "f",
                data_type: DataType::Path(true),
                is_required: Some(false),
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_ok());
        let parser = result.expect("just checked that it's Ok(_)");
        assert_eq!(parser.options.len(), inputs.len());
    }

    #[ignore = "parsing has not been implemented yet, so can't be tested"]
    #[test]
    fn correct_default_types() {
        todo!();
    }
}

mod invalid {
    use std::panic::catch_unwind;

    use super::*;

    #[test]
    fn invalid_short_name() {
        let inputs = [Input {
            names: "-long-name",
            destination: "invalid_opt",
            data_type: DataType::Int32(false),
            is_required: None,
            defaults: None,
        }];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn invalid_long_name() {
        let inputs = [Input {
            names: "--invalid-symbols.#!",
            destination: "invalid_opt",
            data_type: DataType::Int32(false),
            is_required: None,
            defaults: None,
        }];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn invalid_both_names() {
        let inputs = [Input {
            names: "-n --not-separated-by-comma",
            destination: "invalid_opt",
            data_type: DataType::Int32(false),
            is_required: None,
            defaults: None,
        }];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn duplicate_destination() {
        let inputs = [
            Input {
                names: "-a",
                destination: "same_destination",
                data_type: DataType::Int32(false),
                is_required: None,
                defaults: None,
            },
            Input {
                names: "-b",
                destination: "same_destination",
                data_type: DataType::Int32(false),
                is_required: None,
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn duplicate_short_name() {
        let inputs = [
            Input {
                names: "-a",
                destination: "a_destination",
                data_type: DataType::Int32(false),
                is_required: None,
                defaults: None,
            },
            Input {
                names: "-a",
                destination: "another_destination",
                data_type: DataType::Int32(false),
                is_required: None,
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn duplicate_long_name() {
        let inputs = [
            Input {
                names: "--a-name",
                destination: "a_destination",
                data_type: DataType::Int32(false),
                is_required: None,
                defaults: None,
            },
            Input {
                names: "--a-name",
                destination: "another_destination",
                data_type: DataType::Int32(false),
                is_required: None,
                defaults: None,
            },
        ];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn required_and_default() {
        let inputs = [Input {
            names: "--req-and-def",
            destination: "a_destination",
            data_type: DataType::Int32(false),
            is_required: Some(true),
            defaults: Some(&["3"]),
        }];

        let result = catch_unwind(|| create_parser_with_inputs(&inputs));
        assert!(result.is_err());
    }

    #[test]
    fn too_many_defaults() {
        let inputs = [Input {
            names: "--too-many",
            destination: "a_destination",
            data_type: DataType::Int32(false),
            is_required: Some(true),
            defaults: Some(&["3", "4"]),
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
