use std::{fs::OpenOptions, io::Read, path::PathBuf, process::exit};

use parser::{DataType, Optionality, ParsedArgument, Parser};

mod parser;

fn create_parser() -> Parser {
    let mut parser = Parser::new();
    parser.add_positional("path", DataType::String, Optionality::Required);
    parser
}

fn main() {
    let parser = create_parser();
    let args = parser.parse_arguments();

    let path = if let ParsedArgument::String(path_str) = &args["path"] {
        PathBuf::from(path_str)
    } else {
        panic!("should be unreachable")
    };

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
