use std::path::PathBuf;

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

    let file_paths = unsafe { parse_result.get_array_unchecked::<PathBuf>("files") };
    println!("file_paths = {:?}", file_paths);
    let show_ends = unsafe { parse_result.get_single_unchecked::<bool>("show_ends") };
    println!("show_ends = {:?}", show_ends);
    let number = unsafe { parse_result.get_single_unchecked::<bool>("number") };
    println!("number = {:?}", number);
    let show_tabs = unsafe { parse_result.get_single_unchecked::<bool>("show_tabs") };
    println!("show_tabs = {:?}", show_tabs);
}
