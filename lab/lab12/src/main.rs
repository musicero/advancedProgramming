use std::error::Error;

use std::io::{self, Write};

mod interpreter;
mod tests;
mod tokenizer;

use interpreter::*;
use tokenizer::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let _ = args.next();
    if let Some(arg) = args.next() {
        from_file(arg)?;
    } else {
        interactive()?;
    };
    Ok(())
}

fn from_file(filename: String) -> Result<(), Box<dyn Error>> {
    if !filename.ends_with(".mco") {
        // automaticaly boxing the error by using ?.
        Err("File must end with .mco")?;
    }
    let input = std::fs::read_to_string(filename)?;
    let program = match tokenize(input) {
        Ok(program) => program,
        Err(err) => {
            // the interpreter should not panic because the user gave a bad program.
            println!("Error: {:?}", err);
            return Ok(());
        }
    };
    let mut runtime = Runtime::new();
    for statement in program {
        if let Err(err) = interpret(&mut runtime, statement) {
            println!("Error: {:?}", err);
            runtime.print_stack();
            return Ok(());
        }
    }
    match runtime.stack_len() {
        0 => println!("Program returned with no value"),
        1 => println!("{}", runtime.pop_check().unwrap()),
        // print all values remaining
        _ => {
            println!("Return error: Multiple values remaining: ");
            runtime.print_stack();
        }
    };
    return Ok(());
}

fn interactive() -> Result<(), Box<dyn Error>> {
    // If we want the runtime to persist between inputs
    // let mut runtime = Runtime::new();
    loop {
        print!("macho>> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // print error if there is one
        let program = match tokenize(input) {
            Ok(program) => program,
            Err(err) => {
                println!("Error: {:?}", err);
                continue;
            }
        };

        let mut runtime = Runtime::new();
        for statement in program {
            if let Err(err) = interpret(&mut runtime, statement) {
                println!("Error: {:?}", err);
                runtime.print_stack();
                continue;
            }
        }
        match runtime.stack_len() {
            0 => println!("Return error: No values remaining"),
            1 => println!("{}", runtime.pop_check().unwrap()),
            // print all values remaining
            _ => {
                println!("Return error: Multiple values remaining: ");
                runtime.print_stack();
            }
        };
    }
}
