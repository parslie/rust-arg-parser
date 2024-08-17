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
    use super::*;
}

mod invalid {
    use super::*;

    #[test]
    fn duplicate_destination() {
        todo!();
    }

    #[test]
    fn duplicate_short_name() {
        todo!();
    }

    #[test]
    fn duplicate_long_name() {
        todo!();
    }

    #[test]
    fn duplicate_both_names() {
        todo!();
    }

    #[test]
    fn required_and_default() {
        todo!();
    }

    #[test]
    fn too_many_defaults() {
        todo!();
    }

    #[ignore = "parsing has not been implemented yet, so can't be tested"]
    #[test]
    fn wrong_default_types() {
        todo!();
    }
}
