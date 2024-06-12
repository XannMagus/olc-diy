use std::io::{BufRead, stdin};
use anyhow::Result;

use crate::compiler::Compiler;
use crate::tokenizer::{display_queue, Tokenizer};

mod tokenizer;
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
    let token_queue = Tokenizer::new(expr.to_string()).parse()?;
    println!("{}", display_queue(&token_queue));


    let expr = Compiler::new().to_expression(&token_queue)?;
    println!("{}", expr.solve()?);

    Ok(())
}
