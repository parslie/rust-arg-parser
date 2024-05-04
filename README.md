# Currently Unnamed Argument Parser
I've used a few argument parsing libraries for my projects, such as argparse and click for Python, clap for Rust, and CLI11 for C++.
Some have been easy to use and some have been hard to use.
So I wanted to take my own stab at it to see if I can make one that is at least moderately easy to use.

## Example Program
```rust
fn create_parser() -> Parser {
  let parser = Parser::new();
  parser.add_positional("path", DataType::String, Optionality::Required);
  parser.add_flag("verbose", Some("v"), Some("verbose"), false)
  parser
}

fn process_args(args: HashMap<String, ParsedArgument>) -> (String, bool) {
  let mut path = String::new();
  let mut verbose = false;

  if let ParsedArgument::String(value) = &args["path"] {
    path = value.to_owned();
  }
  if let ParsedArgument::Bool(value) = &args["verbose"] {
    verbose = value.to_owned();
  }

  (path, verbose)
}

fn main() {
  let parser = create_parser();
  let unprocessed_args = parser.parse_arguments();
  let (path, verbose) = process_args(args);
  // TODO: do stuff here...
}
```

## Current Features
These features may be removed or changed if I find a better way to implement them,
or if I find that they prevent development of more important features.

### Adding Arguments
You can currently add positionals and options to your parser via the following functions.
- ```Parser::add_positional(...)``` adds a positional.
- ```Parser::add_option(...)``` adds a non-boolean option.
- ```Parser::add_flag(...)``` adds a boolean option.

### Argument Optionality
You can specify whether an argument is required or optional via the Optionality enum.
- ```Optionality::Required``` specifies that the argument is required.
- ```Optionality::Optional``` specifies that the argument is optional.
- ```Optionality::Default(String)``` specifies that the argument is optional and has a default value.

### Argument Types
When adding arguments to your parser, you specify their type via the DataType enum.
- ```DataType::String``` specifies that the argument is a **string of characters**.
- ```DataType::Bool``` specifies that the argument is a **true or false boolean**.
- ```DataType::Int32``` specifies that the argument is a **signed, 32-bit integer**.
- ```DataType::Float32``` specifies that the argument is a **32-bit float**.

## Planned Features
- Sub-parsers will be added, so that CLI tools like git can be created.
- A better way of getting the value of parsed arguments.
