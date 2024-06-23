use anyhow::{anyhow, Result};
use crate::lexer::Token;
use crate::parser::ast::Expr;
use crate::parser::binding_power::BindingPower;
use crate::parser::Parser;

impl Parser {
    pub(super) fn parse_expression(&mut self, bp: BindingPower) -> Result<Expr> {
        let mut left = if let Some(current) = &self.current {
            if let Some(handler) = Self::nud(current) {
                handler(self)
            } else {
                Err(anyhow!(
                    "[PARSER] NUD handler expected for token {current:?}"
                ))
            }
        } else {
            Err(anyhow!(
                "[PARSER] Trying to parse an Expression starting at the end"
            ))
        }?;

        while bp < self.current.clone().into() {
            if let Some(current) = &self.current {
                if let Some(handler) = Self::led(current) {
                    left = handler(self, left, current.into())?;
                } else {
                    return Err(anyhow!(
                        "[PARSER] LED handler expected for token {current:?}"
                    ));
                }
            } else {
                return Err(anyhow!(
                    "[PARSER] End of tokens reached before the end of the expression"
                ));
            }
        }

        Ok(left)
    }

    pub(super) fn parse_binary_expression(&mut self, left: Expr, bp: BindingPower) -> Result<Expr> {
        let operator = self
            .advance()
            .ok_or(anyhow!("[PARSER] Missing operator token"))?;
        let right = self.parse_expression(bp)?;

        Ok(Expr::binary(left, operator, right))
    }

    pub(super) fn parse_assignment_expression(&mut self, left: Expr, bp: BindingPower) -> Result<Expr> {
        let operator = self.advance().ok_or(anyhow!("[PARSER] Should be unreachable"))?;
        let right = self.parse_expression(bp)?;

        Ok(Expr::assignment(left, operator, right))
    }

    pub(super) fn parse_primary_expr(&mut self) -> Result<Expr> {
        if let Some(token) = self.current.clone() {
            match self.advance() {
                Some(Token::NumericLiteral(value)) => Ok(Expr::Number(value)),
                Some(Token::StringLiteral(value)) => Ok(Expr::String(value.clone())),
                Some(Token::Symbol(name)) => Ok(Expr::Symbol(name.clone())),
                _ => Err(anyhow!(
                    "[PARSER] Cannot create Primary expression from {token:?}"
                )),
            }
        } else {
            Err(anyhow!(
                "[PARSER] Cannot create Primary expression from void token"
            ))
        }
    }

    pub(super) fn parse_prefix_expression(&mut self) -> Result<Expr> {
        let operator = self.advance().ok_or(anyhow!("[PARSER] Should be unreachable"))?;
        let right = self.parse_expression(BindingPower::Default)?;

        Ok(Expr::prefix(operator, right))
    }

    pub(super) fn parse_grouping_expression(&mut self) -> Result<Expr> {
        self.advance();
        let expr = self.parse_expression(BindingPower::Default);
        self.expect(&Token::ClosingParenthesis)?;
        expr
    }
}