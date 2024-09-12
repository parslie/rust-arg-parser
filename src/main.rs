use argument_parser::{argument::DataType, Parser};

/*
Should try
- Add positional
- Add sub-parser
- Change positional to be optional
*/

fn main() {
    let mut parser = Parser::new();
    parser.positional("output_file", DataType::Path(false));

    // Echo sub parser
    let echo_parser = parser.sub_parser("echo");
    echo_parser
        .positional("inputs", DataType::String(true))
        .is_required(false);
    println!("{:?}", echo_parser);
    println!("Echo parser:\n{:?}", echo_parser);

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
    println!("Cat parser:\n{:?}", cat_parser);

    // Print parse result
    println!("Parser:\n{:?}", parser);
    //let parse_result = parser.parse(None);
    //println!("Parse result:\n{:?}", parse_result);
}
