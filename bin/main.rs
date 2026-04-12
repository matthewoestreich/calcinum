#[macro_use]
mod macros;
mod context;

use context::Context;
use rustyline::DefaultEditor;
use std::{
    env,
    io::{Write, stdout},
    process,
};

const EXPECTED_ARGS_LEN: usize = 1;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        repl_mode();
        process::exit(0);
    } else if args.len() != EXPECTED_ARGS_LEN {
        eprintln!(
            "ERROR expected {EXPECTED_ARGS_LEN} argument(s), got {}",
            args.len()
        );
        process::exit(1);
    } else {
        match args.first().expect("verified argc > 0").as_str() {
            "--version" | "-v" => {
                println!("{}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            }
            s => match calcinum::eval(s) {
                Ok(r) => {
                    println!("{r}");
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("ERROR parsing expression\n\n{s}\n\n{e}");
                    process::exit(1);
                }
            },
        }
    }
}

fn repl_mode() {
    let mut rl = DefaultEditor::new().expect("rustyline");
    let mut ctx = Context::new();

    // ADD YOUR COMMAND AND DESCRIPTION HERE SO IT
    // IS PRINTED TO CONSOLE SO USERS ARE AWARE OF IT!
    let commands = vec![
        ("clear", "   clears the screen"),
        ("reset", "   resets history"),
        ("exit", "    exits the repl"),
        ("history", " prints available history"),
        ("commands", "prints this message"),
    ];

    print_commands(&commands);

    loop {
        let line_num = format_cyan!("@{}", ctx.size() + 1);
        let prompt = format!("[{line_num}]> ");
        let line = rl.readline(&prompt);

        match line {
            Ok(input) => {
                //
                // ADD YOUR COMMAND'S "HANDLER" HERE!
                //
                let input = input.as_str();
                match input {
                    "clear" => clear_screen(),
                    "reset" => {
                        ctx.reset();
                        _ = rl.clear_history();
                    }
                    "history" => ctx.print_history(),
                    "exit" => break,
                    "commands" => print_commands(&commands),
                    s => ctx.parse_and_eval(s),
                };
                rl.add_history_entry(input).expect("input added to history");
            }
            Err(_) => break,
        }
    }
}

/// Each tuple is : ("command_name", "command_description")
fn print_commands(commands: &Vec<(&str, &str)>) {
    println!("\nCommands:\n");
    for cmd in commands {
        print_magenta!("{}", cmd.0);
        println!("      {}", cmd.1);
    }
    println!();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().expect("stdout flush");
}
