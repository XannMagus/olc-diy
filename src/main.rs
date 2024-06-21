use std::fs;
use std::io::{BufRead, stdin};
use std::path::PathBuf;

use anyhow::Result;

use crate::lexer::{display_queue, Lexer};

mod lexer;

fn main() {
    let mut handle = stdin().lock();
    let mut input = String::new();
    let path = {
        let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_buf.push("examples/00.lang");
        path_buf
    };

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

        if input.eq_ignore_ascii_case("file") {
            input = fs::read_to_string(&path).unwrap_or("".to_string());
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
    let token_queue = Lexer::new(expr.to_string()).tokenize()?;
    println!("{}", display_queue(&token_queue));

    Ok(())
}
