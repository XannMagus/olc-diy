use crate::lexer::Token;
use crate::parser::Parser;
use anyhow::{anyhow, Error};

impl Parser {
    pub(super) fn expect_error_with_fn<F>(
        &mut self,
        check: F,
        err: Option<Error>,
    ) -> anyhow::Result<Option<Token>>
    where
        F: Fn(&Token) -> bool,
    {
        let token = &self
            .current
            .clone()
            .ok_or(anyhow!("[PARSER] expected a token, got VOID instead"))?;

        if !check(&token) {
            if let Some(err) = err {
                Err(err)
            } else {
                Err(anyhow!("[PARSER] failed check for {token:?}"))
            }
        } else {
            Ok(self.advance())
        }
    }

    pub(super) fn expect_error(
        &mut self,
        token_kind: &Token,
        err: Option<Error>,
    ) -> anyhow::Result<Option<Token>> {
        let err = if let Some(err) = err {
            Some(err)
        } else {
            Some(anyhow!(
                "[PARSER] expected {token_kind:?}, got {:?} instead",
                self.current.clone().unwrap()
            ))
        };
        self.expect_error_with_fn(|token| Token::match_variant(&token, token_kind), err)
    }

    pub(super) fn expect(&mut self, token_kind: &Token) -> anyhow::Result<Option<Token>> {
        self.expect_error(token_kind, None)
    }
}
