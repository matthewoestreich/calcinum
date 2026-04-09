use calcinum::parse_expression;
use std::env;

fn main() {
    let mut args = env::args().skip(1);

    match args.next() {
        None => {
            eprintln!("Missing argument! Please provide an expression as a string, e.g., \"2 + 2\"")
        }
        Some(ref expression) => match parse_expression(expression) {
            Ok(r) => println!("{r}"),
            Err(e) => {
                eprintln!("ERROR parsing expression\n\n{expression}\n\n{e}");
            }
        },
    }
}
