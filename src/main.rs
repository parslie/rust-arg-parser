use argument_parser::{argument::DataType, Parser};

fn main() {
    let mut parser = Parser::new();
    parser.positional("output_file", DataType::Path(false));

    // Echo sub parser
    let echo_parser = parser.sub_parser("echo");
    echo_parser
        .positional("inputs", DataType::String(true))
        .is_required(false);

    // Cat sub parser
    let cat_parser = parser.sub_parser("cat");
    cat_parser.positional("input_files", DataType::Path(true));
    cat_parser
        .option("-E, --show-ends", "show_ends", DataType::Bool(false))
        .default_value("false"); // TODO: flag instead of option
    cat_parser
        .option("-n, --number", "number", DataType::Bool(false))
        .default_value("false"); // TODO: flag instead of option
    cat_parser
        .option("-T, --show-tabs", "show_tabs", DataType::Bool(false))
        .default_value("false"); // TODO: flag instead of option

    // Print parse result
    println!("{:?}", parser);
    let parse_result = parser.parse(None);
    println!("{:?}", parse_result);
}
