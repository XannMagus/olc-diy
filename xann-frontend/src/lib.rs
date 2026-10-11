//! Lexing and parsing for xann.
//!
//! Parsing currently produces a raw AST, without semantic guarantees.
//! Future semantic analysis will produce the shared checked representation
//! owned by `xann-core` for both execution backends.

mod formatting;

pub mod ast;
pub mod lexer;
pub mod parser;

pub use formatting::{Displayable, ToDoc};

/// Tokenize and parse source into a raw program AST.
pub fn parse(source: &str) -> anyhow::Result<ast::Stmt> {
    parser::Parser::new(lexer::Lexer::new(source.to_owned()).tokenize()?).parse()
}
