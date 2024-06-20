use anyhow::{anyhow, Result};

use crate::solver::Expression;
use crate::lexer::{Token, TokenKind, TokenQueue};

pub struct Compiler {
    operator_stack: Vec<Token>,
    previous_token: Option<Token>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            operator_stack: vec![],
            previous_token: None
        }
    }

    pub fn to_expression(mut self, input: &TokenQueue) -> Result<Expression> {
        let mut rpn: TokenQueue = TokenQueue::new();

        for token in input.iter() {
            match token.kind() {
                TokenKind::NumericLiteral => {
                    self.previous_token = Some(token.clone());
                    rpn.push_back(token.clone());
                }
                TokenKind::Operator(mut o1) => {
                    o1 = o1.correct_arity(&self.previous_token)?;

                    while let Some(o2) = self.operator_stack.last() {
                        match o2.kind() {
                            TokenKind::Operator(o2) => {
                                if o2.precedence() >= o1.precedence() {
                                    rpn.push_back(self.operator_stack.pop().unwrap());
                                } else {
                                    break;
                                }
                            }
                            TokenKind::OpeningParenthesis => {
                                break;
                            }
                            _ => {}
                        }
                    }
                    let mut updated_token = token.clone();
                    updated_token.update_kind(TokenKind::Operator(o1));

                    self.previous_token = Some(updated_token.clone());
                    self.operator_stack.push(updated_token);
                }
                TokenKind::OpeningParenthesis => {
                    self.operator_stack.push(token.clone());
                    self.previous_token = Some(token.clone());
                }
                TokenKind::ClosingParenthesis => {
                    while let Some(last) = self.operator_stack.last() {
                        match last.kind() {
                            TokenKind::Operator(_) => {
                                rpn.push_back(self.operator_stack.pop().unwrap());
                            }
                            TokenKind::OpeningParenthesis => {
                                self.operator_stack.pop();
                                break;
                            }
                            TokenKind::Unknown => {
                                return Err(anyhow!("Somehow we missed a parsing error here"));
                            }
                            _ => {}
                        }
                    }
                    self.previous_token = Some(token.clone());
                }
                TokenKind::OpeningScope |
                TokenKind::ClosingScope |
                TokenKind::Symbol |
                TokenKind::Separator |
                TokenKind::StringLiteral |
                TokenKind::EndOfStatement |
                TokenKind::Keyword(_) => {
                    return Err(anyhow!("This is not handled yet!"));
                }
                TokenKind::Unknown => {
                    return Err(anyhow!("Somehow we missed a parsing error here"));
                }
            }
        }

        while let Some(op) = self.operator_stack.pop() {
            rpn.push_back(op);
        }

        Ok(Expression::new(rpn))
    }
}