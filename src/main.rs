use std::io::{BufRead, stdin};

use anyhow::Result;

use crate::lexer::{display_queue, Lexer};

mod lexer;
mod solver;
mod compiler;

fn main() {
    let mut handle = stdin().lock();
    let mut input = String::new();

    loop {
        println!("Solve ('quit' or 'exit' to exit):");
        handle.read_line(&mut input).expect("Failed to read line");

        while let Some('\n') | Some('\r') | Some(' ') = input.chars().next_back() {
            input.pop();
        }

        input = input.replace("\\n", "\n");

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        let result = process(&input);
        match result {
            Ok(_) => {}
            Err(err) => {
                println!("{err}");
            }
        }

        input.clear();
    }
}

fn process(expr: &String) -> Result<()> {
    let token_queue = Lexer::new(expr.to_string()).parse()?;
    println!("{}", display_queue(&token_queue));

    Ok(())
}
