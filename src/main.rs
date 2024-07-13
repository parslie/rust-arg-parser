use argument_parser::{unparsed::DataType, Parser};

fn create_parser() -> Parser {
    let mut parser = Parser::new("prog_name", "this is the prog description");

    // Help flag
    parser
        .help_flag()
        .short_name("h")
        .long_name("help")
        .description("Print this help page.")
        .build();

    // Positionals
    parser
        .argument("pos_float", DataType::Float32)
        .description("A positional float.")
        .build();
    parser
        .argument("pos_int", DataType::Int32)
        .description("A positional integer.")
        .build();

    // Options
    parser
        .argument("opt_string", DataType::String)
        .long_name("string-opt")
        .description("An optional string.")
        .build();
    parser
        .argument("opt_bool", DataType::Bool)
        .short_name("b")
        .default("false")
        .description("A false-by-default boolean.")
        .build();

    parser
}

fn main() {
    let parser = create_parser();
    let parse_result = parser.parse_args();
    println!("Parse results:\n{:?}", parse_result);
}
