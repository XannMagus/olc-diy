use crate::lexer::Token;
use crate::parser::ast::Type;
use crate::parser::binding_power::{BindingPower, TypeBindingPower};
use crate::parser::Parser;
use anyhow::{anyhow, Result};

impl Parser {
    pub(super) fn parse_symbol_type(&mut self) -> Result<Type> {
        let type_name = if let Some(token) = self.expect(&Token::Symbol(" whatevs".to_string()))? {
            match token {
                Token::Symbol(name) => name,
                _ => return Err(anyhow!("[PARSER] Unreachable state")),
            }
        } else {
            return Err(anyhow!("[PARSER] Unreachable state"));
        };
        Ok(Type::Symbol(type_name))
    }

    pub(super) fn parse_array_type(&mut self) -> Result<Type> {
        self.expect(&Token::OpeningBracket)?;
        self.expect(&Token::ClosingBracket)?;

        let underlying = self.parse_type(BindingPower::Default.to_type())?;

        Ok(Type::Array(Box::new(underlying)))
    }

    pub(super) fn parse_type(&mut self, bp: TypeBindingPower) -> Result<Type> {
        let mut left = if let Some(current) = &self.current {
            if let Some(handler) = Self::type_nud(current) {
                handler(self)
            } else {
                Err(anyhow!("[PARSER] TYPE_NUD expected for token {current:?}"))
            }
        } else {
            Err(anyhow!("[PARSER] Trying to parse a type starting at EOF"))
        }?;

        while bp < self.current.clone().into() {
            if let Some(current) = &self.current {
                if let Some(handler) = Self::type_led(current) {
                    left = handler(self, left, current.into())?;
                } else {
                    return Err(anyhow!(
                        "[PARSER] TYPE_LED handler expected for {current:?}"
                    ));
                }
            } else {
                return Err(anyhow!(
                    "[PARSER] End of tokens reached before the end of type expression"
                ));
            }
        }

        Ok(left)
    }
}
