use argument_parser::{argument::DataType, Parser};

fn main() {
    let mut parser = Parser::new();
    parser.positional("a_float", DataType::Float32(false));
    parser
        .positional("a_string_array", DataType::String(true))
        .defaults(&["string_1", "string_2"]);
    println!("{:?}", parser);
}
