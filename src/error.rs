use crate::unparsed::UnparsedArgument;

pub const REQUIRED_POSITIONALS: &str = "Missing required positionals.";
pub const REQUIRED_OPTIONS: &str = "Missing required options.";

pub fn unrecognized_positional(raw_arg: &str) -> String {
    format!(
        "Unrecognized positional '{}'. You have already given all positionals a value.",
        raw_arg
    )
}

pub fn unrecognized_option(raw_arg: &str) -> String {
    format!(
        "Unrecognized option '{}'. (have you already used it?)",
        raw_arg
    )
}

pub fn no_value_provided(unparsed_arg: &UnparsedArgument) -> String {
    format!(
        "No value provided for option '{}'.",
        unparsed_arg.get_name()
    )
}

pub fn invalid_value(unparsed_arg: &UnparsedArgument, value: &str) -> String {
    match unparsed_arg.is_option() {
        false => format!(
            "Invalid value '{}' for positional '{}'.",
            value,
            unparsed_arg.get_name()
        ),
        true => format!(
            "Invalid value '{}' for option '{}'.",
            value,
            unparsed_arg.get_name()
        ),
    }
}
