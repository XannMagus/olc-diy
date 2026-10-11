use anyhow::{anyhow, bail, Result};

use crate::ast::{Stmt, StructFieldInfo};
use crate::lexer::{Keyword, OperatorKind, Token};
use crate::parser::binding_power::BindingPower;
use crate::parser::utilities::UniqueKeyHashMap;
use crate::parser::Parser;

impl Parser {
    pub(super) fn parse_statement(&mut self) -> Result<Stmt> {
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

    pub(super) fn parse_variable_declaration_stmt(&mut self) -> Result<Stmt> {
        if let Some(token) = &self.advance() {
            let is_constant = match token {
                Token::Keyword(keyword) => match keyword {
                    Keyword::Let => false,
                    Keyword::Const => true,
                    _ => bail!("[PARSER] Can't be reached"),
                },
                _ => bail!("[PARSER] Can't be reached"),
            };
            let var_name = if let Some(Token::Symbol(name)) = self.expect_error(
                &Token::Symbol("whatevs".to_string()),
                Some(anyhow!("[PARSER] Expected to get the variable name")),
            )? {
                name
            } else {
                bail!("[PARSER] Expected to get the variable name")
            };
            let explicit_type = if let Some(Token::Colon) = &self.current {
                self.advance();
                Some(self.parse_type(BindingPower::Default.to_type())?)
            } else {
                None
            };

            let assigned_expr = if let Some(Token::EndOfStatement) = self.current {
                if explicit_type.is_none() {
                    bail!("[PARSER] Missing right-hand side in variable declaration or explicit type.");
                }
                None
            } else {
                self.expect(&Token::Operator(OperatorKind::Assignment))?;
                Some(self.parse_expression(BindingPower::Assignment)?)
            };

            self.expect(&Token::EndOfStatement)?;

            if is_constant && assigned_expr.is_none() {
                bail!("[PARSER] Cannot declare constant {var_name} without initial value.");
            }

            Ok(Stmt::variable_declaration(
                var_name,
                is_constant,
                assigned_expr,
                explicit_type,
            ))
        } else {
            Err(anyhow!("[PARSER] Can't be reached"))
        }
    }

    pub(super) fn parse_struct_definition_stmt(&mut self) -> Result<Stmt> {
        self.expect(&Token::Keyword(Keyword::Struct))?
            .ok_or(anyhow!("[PARSER] Can't be reached (EOF)"))?;
        if let Token::Symbol(name) = self
            .expect(&Token::Symbol("whatever".to_string()))?
            .ok_or(anyhow!("[PARSER] Reached EOF in struct definition."))?
        {
            self.expect(&Token::OpeningScope)?
                .ok_or(anyhow!("[PARSER] Reached EOF in struct definition"))?;
            let mut map: UniqueKeyHashMap<String, StructFieldInfo> = UniqueKeyHashMap::new();
            while self.has_token() && !matches!(self.current, Some(Token::ClosingScope)) {
                let is_static = if let Some(Token::Keyword(Keyword::Static)) = self.current {
                    self.expect(&Token::Keyword(Keyword::Static))?;
                    true
                } else {
                    false
                };

                let token = self.advance().expect(
                    "[PANIC] Unreachable, has_token() returned true but advance() returned None",
                );
                match token {
                    Token::Symbol(name) => {
                        self.expect_error(&Token::Colon, Some(anyhow!("[PARSER] Expected type declaration after name of field in struct definition")))?;
                        let field_type = self.parse_type(BindingPower::Default.to_type())?;
                        self.expect(&Token::Separator)?;

                        map.insert(name.clone(), StructFieldInfo::new(name.clone(), field_type, is_static)).map_err(|e| {anyhow!("[PARSER] Property {name} has already been declared in struct definition")})?;
                    }
                    Token::EndOfFile | Token::Unknown => {
                        unreachable!()
                    }
                    _ => {
                        bail!("[PARSER] Unexpected token {token:?} in struct definition")
                    }
                }
            }

            self.expect(&Token::ClosingScope)?
                .ok_or(anyhow!("[PARSER] Reached EOF in struct definition"))?;
            Ok(Stmt::StructDeclaration {
                identifier: name,
                fields: map.to_vec(),
            })
        } else {
            bail!("[PARSER] Expected identifier ")
        }
    }
}
