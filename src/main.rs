use std::{fs::OpenOptions, io::Read, process::exit};

use parser::{
    argument::{DataType, Optionality},
    Parser,
};

mod parser;

fn create_parser() -> Parser {
    let mut parser = Parser::new();
    parser.add_positional("path", DataType::Path, Optionality::Required);
    parser
}

fn main() {
    let parser = create_parser();
    let args = parser.parse_arguments();

    let path = args.get_path("path");

    match OpenOptions::new().read(true).open(path) {
        Ok(mut file) => {
            let mut file_contents = String::new();
            match file.read_to_string(&mut file_contents) {
                Ok(_) => {
                    println!("{}", file_contents);
                    exit(0);
                }
                Err(err) => {
                    println!("{}", err.to_string());
                    exit(1);
                }
            }
        }
        Err(err) => {
            println!("{}", err.to_string());
            exit(1);
        }
    }
}
