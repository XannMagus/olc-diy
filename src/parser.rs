use anyhow::{anyhow, bail, Error as aError, Result};

use crate::lexer::{Keyword, OperatorKind, Token, TokenQueue};
use crate::parser::ast::{Expr, Stmt};
use crate::parser::binding_power::BindingPower;

mod ast;
mod binding_power;

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

    fn expect_error_with_fn<F>(&mut self, check: F, err: Option<aError>) -> Result<Option<Token>>
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

    fn expect_error(&mut self, token_kind: &Token, err: Option<aError>) -> Result<Option<Token>> {
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

    fn expect(&mut self, token_kind: &Token) -> Result<Option<Token>> {
        self.expect_error(token_kind, None)
    }

    fn nud(token: &Token) -> Option<fn(&mut Self) -> Result<Expr>> {
        match token {
            Token::NumericLiteral(_)
            | Token::StringLiteral(_)
            | Token::Symbol(_)
            | Token::Keyword(_) => Some(Self::parse_primary_expr),
            _ => None,
        }
    }

    fn led(token: &Token) -> Option<fn(&mut Self, Expr, BindingPower) -> Result<Expr>> {
        match token {
            Token::Operator(operator) => match operator {
                OperatorKind::LogicalOr
                | OperatorKind::LogicalAnd
                | OperatorKind::Range
                | OperatorKind::Exp
                | OperatorKind::Product
                | OperatorKind::Quotient
                | OperatorKind::Difference
                | OperatorKind::Sum
                | OperatorKind::Equals
                | OperatorKind::Different
                | OperatorKind::GreaterThan
                | OperatorKind::GreaterThanEqual
                | OperatorKind::LessThan
                | OperatorKind::LessThanEqual
                | OperatorKind::Assignment
                | OperatorKind::PlusEqual
                | OperatorKind::MinusEqual
                | OperatorKind::TimesEqual
                | OperatorKind::DivideEqual => Some(Self::parse_binary_expression),
                _ => None,
            },
            _ => None,
        }
    }

    fn stmt(token: &Token) -> Option<fn(&mut Self) -> Result<Stmt>> {
        match token {
            Token::Keyword(keyword) => match keyword {
                Keyword::Const | Keyword::Let => Some(Self::parse_variable_declaration_stmt),
                _ => None,
            },
            _ => None,
        }
    }

    fn parse_primary_expr(&mut self) -> Result<Expr> {
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

    fn parse_binary_expression(&mut self, left: Expr, bp: BindingPower) -> Result<Expr> {
        let operator = self
            .advance()
            .ok_or(anyhow!("[PARSER] Missing operator token"))?;
        let right = self.parse_expression(bp)?;

        Ok(Expr::binary(left, operator, right))
    }

    fn parse_expression(&mut self, bp: BindingPower) -> Result<Expr> {
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
                    left = handler(self, left, bp)?;
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

    fn parse_statement(&mut self) -> Result<Stmt> {
        if let Some(token) = &self.current {
            if let Some(handler) = Self::stmt(token) {
                handler(self)
            } else {
                let expression = self.parse_expression(BindingPower::Default)?;
                self.expect(&Token::EndOfStatement)?;

                Ok(Stmt::Expression(expression))
            }
        } else {
            Err(anyhow!(
                "[PARSER] Trying to parse an empty token as statement"
            ))
        }
    }

    fn parse_variable_declaration_stmt(&mut self) -> Result<Stmt> {
        if let Some(token) = &self.advance() {
            let is_constant = match token {
                Token::Keyword(keyword) => match keyword {
                    Keyword::Let => false,
                    Keyword::Const => true,
                    _ => bail!("[PARSER] Can't be reached"),
                },
                _ => bail!("[PARSER] Can't be reached"),
            };
            let var_name = if let Some(token) = self.expect_error(
                &Token::Symbol("whatevs".to_string()),
                Some(anyhow!("[PARSER] Expected to get the variable name")),
            )? {
                if let Token::Symbol(name) = token {
                    name
                } else {
                    bail!("[PARSER] Reached an unreachable state")
                }
            } else {
                bail!("[PARSER] Expected to get the variable name")
            };
            self.expect(&Token::Operator(OperatorKind::Assignment))?;
            let assigned_expr = self.parse_expression(BindingPower::Assignment)?;
            self.expect(&Token::EndOfStatement)?;

            Ok(Stmt::variable_declaration(
                var_name,
                is_constant,
                assigned_expr,
            ))
        } else {
            Err(anyhow!("[PARSER] Can't be reached"))
        }
    }
}
