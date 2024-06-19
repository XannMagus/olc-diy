use std::collections::VecDeque;

use anyhow::Result;

pub use shared_types::{Token, TokenKind};
use shared_types::states::{StartState, State, TemporaryData};

mod shared_types;

pub struct Lexer {
    input: String,
}

pub type TokenQueue = VecDeque<Token>;

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
        }
    }

    pub fn parse(self) -> Result<TokenQueue> {
        let cloned = self.input.clone();
        let iter = cloned.chars().peekable();
        let mut tmp = TemporaryData::new(self.input.clone(), iter);
        let mut state: Box<dyn State> = Box::new(StartState);

        loop {
            (state, tmp) = state.handle(tmp)?;
            if state.is_final() {
                (_, tmp) = state.handle(tmp)?;
                break;
            }
        }

        Ok(tmp.output())
    }
}


// Utilities

pub fn display_queue(queue: &TokenQueue) -> String {
    queue.iter().fold(
        String::new(),
        |mut acc, x| {
            if !acc.is_empty() {
                acc.push('\n');
            }
            acc.push_str(&x.to_string());
            acc
        },
    )
}