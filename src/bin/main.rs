use calcinum::parse_expression;
use std::{env, process};

fn main() {
    let args = env::args().skip(1);
    let mut args = args.peekable();

    if let Some(v) = args.peek()
        && (v == "--version" || v == "-v")
    {
        let version = env!("CARGO_PKG_VERSION");
        println!("{version}");
        process::exit(0);
    }

    match args.next() {
        None => {
            eprintln!("Missing argument! Please provide an expression as a string, e.g., \"2 + 2\"")
        }
        Some(ref expression) => match parse_expression(expression) {
            Ok(r) => println!("{r}"),
            Err(e) => {
                eprintln!("ERROR parsing expression\n\n{expression}\n\n{e}");
                process::exit(1);
            }
        },
    }
}
