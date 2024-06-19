use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Result};
use crate::lexer::{TokenKind, TokenQueue};


#[derive(Debug)]
pub struct Expression {
    rpn: TokenQueue,
}

impl Expression {
    pub fn new(rpn: TokenQueue) -> Self {
        Self { rpn }
    }

    pub fn display_infix(&self) -> Result<String> {
        let mut work_stack: Vec<String> = Vec::new();
        let mut iter = self.rpn.iter().peekable();

        while let Some(token) = iter.next() {
            match token.kind() {
                TokenKind::NumericLiteral => {
                    work_stack.push(token.as_string());
                }
                TokenKind::Operator(operator) => {
                    if operator.arity() == 2 {
                        let Some(right) = work_stack.pop() else { return Err(anyhow!("Malformed Expression")); };
                        let Some(left) = work_stack.pop() else { return Err(anyhow!("Malformed Expression")); };

                        if iter.peek().is_none() {
                            work_stack.push(format!("{left} {operator} {right}"));
                        } else {
                            work_stack.push(format!("({left} {operator} {right})"));
                        }
                    } else if operator.arity() == 1 {
                        let Some(operand) = work_stack.pop() else { return Err(anyhow!("Malformed Expression")); };

                        work_stack.push(format!("{operator}{operand}"));
                    }
                }
                _ => {}
            }
        }

        Ok(work_stack.pop().unwrap())
    }

    pub fn display_postfix(&self) -> Result<String> {
        Ok(
            self.rpn.iter().fold(String::new(), |mut acc, x| {
                if !acc.is_empty() {
                    acc.push(' ');
                }
                acc.push_str(&x.as_string());
                acc
            })
        )
    }

    pub fn debug_display(&self) -> Result<String> {
        Ok(
            self.rpn.iter().fold(String::new(), |mut acc, x| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&x.to_string());
                acc
            })
        )
    }

    pub fn solve(&self) -> Result<f64> {
        let mut solve_stack = Vec::new();

        for token in self.rpn.iter() {
            match token.kind() {
                TokenKind::NumericLiteral => {
                    solve_stack.push(token.value().unwrap());
                }
                TokenKind::Operator(operator) => {
                    if operator.arity() == 2 {
                        let Some(right) = solve_stack.pop() else { return Err(anyhow!("Malformed Expression")); };
                        let Some(left) = solve_stack.pop() else { return Err(anyhow!("Malformed Expression")); };

                        solve_stack.push(operator.compute_2(left, right));
                    } else if operator.arity() == 1 {
                        let Some(operand) = solve_stack.pop() else { return Err(anyhow!("Malformed Expression")); };

                        solve_stack.push(operator.compute_1(operand));
                    }
                }
                _ => {}
            }
        }

        Ok(solve_stack.pop().unwrap())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str_representation = String::new();
        for token in &self.rpn {
            str_representation.push_str(format!("{token}\n").as_str());
        }
        write!(f, "{str_representation}")
    }
}