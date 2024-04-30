use parser::{DataType, Optionality, Parser};

mod parser;

fn main() {
    let mut parser = Parser::new();
    parser.add_positional("a_positional", DataType::Bool, Optionality::Required);
    println!("{:?}", parser);

    //let parse_result = parser.parse_arguments();
    //println!("{:?}", parse_result);

    parser.print_help();
}
