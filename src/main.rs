use argument_parser::{argument::DataType, Parser};

fn main() {
    let mut parser = Parser::new();

    parser.positional("files", DataType::Path(true));

    parser
        .option("-E, --show-ends", "show_ends", DataType::Bool(false))
        .defaults(&["false"]);
    parser
        .option("-n, --number", "number", DataType::Bool(false))
        .defaults(&["false"]);
    parser
        .option("-T, --show-tabs", "show_tabs", DataType::Bool(false))
        .defaults(&["false"]);

    println!("{:?}", parser);

    let parse_result = parser.parse_args();
    println!("{:?}", parse_result);
}
