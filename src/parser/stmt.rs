use anyhow::{anyhow, bail};
use crate::lexer::{Keyword, OperatorKind, Token};
use crate::parser::ast::Stmt;
use crate::parser::binding_power::BindingPower;
use crate::parser::Parser;

impl Parser {

    pub(super) fn parse_statement(&mut self) -> anyhow::Result<Stmt> {
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

    pub(super) fn parse_variable_declaration_stmt(&mut self) -> anyhow::Result<Stmt> {
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