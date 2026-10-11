use std::collections::VecDeque;

use anyhow::Result;

use shared_types::states::{StartState, State, TemporaryData};
pub use shared_types::Keyword;
pub use shared_types::OperatorKind;
pub use shared_types::Token;

mod shared_types;

pub struct Lexer {
    input: String,
}

pub type TokenQueue = VecDeque<Token>;

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn tokenize(self) -> Result<TokenQueue> {
        let mut tmp = TemporaryData::new(self.input.chars().peekable());
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
