use anyhow::Result;

use crate::lexer::{Token, TokenQueue};
use crate::parser::ast::Stmt;

pub use utilities::Displayable;

mod ast;
mod binding_power;
mod expr;
mod lookups;
mod stmt;
mod error_handling;
mod utilities;

pub struct Parser {
    tokens: TokenQueue,
    current: Option<Token>,
}

impl Parser {
    pub fn new(mut tokens: TokenQueue) -> Self {
        let current = tokens.pop_front();
        Self { tokens, current }
    }

    pub fn parse(&mut self) -> Result<Stmt> {
        let mut stmts = vec![];

        while self.has_token() {
            stmts.push(self.parse_statement()?)
        }

        Ok(Stmt::Block(stmts))
    }
}

impl Parser {
    fn advance(&mut self) -> Option<Token> {
        let ret = self.current.clone();
        self.current = self.tokens.pop_front();
        ret
    }

    fn has_token(&self) -> bool {
        if let Some(token) = &self.current {
            !token.is_eof()
        } else {
            false
        }
    }
}
