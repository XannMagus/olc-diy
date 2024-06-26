use std::fs;
use std::io::{stdin, BufRead};
use std::path::PathBuf;

use anyhow::Result;
use debug_print::debug_println;

use crate::lexer::Lexer;
use crate::parser::{Displayable, Parser};

mod lexer;
mod parser;

fn main() {
    let mut handle = stdin().lock();
    let mut input = String::new();
    let path_1 = {
        let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_buf.push("examples/00.lang");
        path_buf
    };
    let path_2 = {
        let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_buf.push("examples/01.lang");
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
            input = fs::read_to_string(&path_1).unwrap_or("".to_string());
        }

        if input.eq_ignore_ascii_case("file1") {
            input = fs::read_to_string(&path_2).unwrap_or("".to_string());
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
    let mut parser = Parser::new(Lexer::new(expr.to_string()).tokenize()?);
    let ast = parser.parse()?;
    debug_println!("{}", Displayable(ast));

    Ok(())
}
