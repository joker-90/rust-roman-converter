use std::{env, process};

use roman_converter::{Action, run};

fn main() {
    let action = Action::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match run(action) {
        Err(e) => {
            eprintln!("Application Error: {}", e);
            process::exit(1);
        }
        Ok(converted) => {
            println!("Result: {}", converted)
        }
    }
}
