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