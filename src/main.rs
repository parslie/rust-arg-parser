use parser::{DataType, Optionality, Parser};

mod parser;

fn main() {
    let mut parser = Parser::new();

    parser.add_positional("req_pos", DataType::String, Optionality::Required);
    parser.add_positional("opt_pos", DataType::Int32, Optionality::Optional);
    parser.add_positional(
        "def_pos",
        DataType::Float32,
        Optionality::Default("3.1415".to_string()),
    );

    parser.add_option(
        "req_opt",
        DataType::Float32,
        Some("a"),
        None,
        Optionality::Required,
    );
    parser.add_option(
        "opt_opt",
        DataType::Int32,
        Some("s"),
        Some("sad"),
        Optionality::Optional,
    );
    parser.add_option(
        "def_opt",
        DataType::String,
        None,
        Some("das"),
        Optionality::Default("default_opt".to_string()),
    );

    parser.add_flag("set_false_flag", Some("q"), None, true);
    parser.add_flag("set_true_flag", None, Some("qwerty"), false);

    let parse_result = parser.parse_arguments();
    println!("{:?}", parse_result);
}
