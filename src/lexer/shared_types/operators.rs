use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Result};

use crate::lexer::shared_types::token::Token;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Operator {
    kind: OperatorKind,
    precedence: u8,
    arity: u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum OperatorKind {
    // Arithmetic
    Exp,
    Product,
    Quotient,
    Difference,
    Sum,
    Negate,
    Positive,
    Increment,
    Decrement,
    // Logical
    LogicalOr,
    LogicalAnd,
    LogicalNot,
    Equals,
    Different,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    //Assignment
    Assignment,
    PlusEqual,
    MinusEqual,
    TimesEqual,
    DivideEqual,
    //Range
    Range,
}

impl Operator {
    pub fn arity(&self) -> u8 {
        self.arity
    }

    pub fn precedence(&self) -> u8 {
        self.precedence
    }

    pub fn compute_2(&self, left: f64, right: f64) -> f64 {
        match self.kind {
            OperatorKind::Exp => left.powf(right),
            OperatorKind::Product => left * right,
            OperatorKind::Quotient => left / right,
            OperatorKind::Difference => left - right,
            OperatorKind::Sum => left + right,
            _ => 0.0
        }
    }

    pub fn logical_compute_2(&self, left: bool, right: bool) -> bool {
        match self.kind {
            OperatorKind::LogicalOr => left || right,
            OperatorKind::LogicalAnd => left && right,
            OperatorKind::Equals => left == right,
            OperatorKind::Different => left != right,
            OperatorKind::GreaterThan => left > right,
            OperatorKind::GreaterThanEqual => left >= right,
            OperatorKind::LessThan => left < right,
            OperatorKind::LessThanEqual => left <= right,
            _ => true,
        }
    }

    pub fn compute_1(&self, operand: f64) -> f64 {
        match self.kind {
            OperatorKind::Negate => -operand,
            OperatorKind::Positive => operand,
            OperatorKind::Increment => operand+1.0,
            OperatorKind::Decrement => operand-1.0,
            _ => 0.0,
        }
    }

    pub fn logical_compute_1(&self, operand: bool) -> bool {
        match self.kind {
            OperatorKind::LogicalNot => !operand,
            _ => true,
        }
    }

    pub fn correct_arity(self, previous: &Option<Token>) -> Result<Self> {
        let unary = if let Some(previous) = previous {
            match previous {
                Token::NumericLiteral(_) |
                Token::ClosingParenthesis => false,
                _ => true
            }
        } else {
            true
        };

        let out = match self.kind {
            OperatorKind::Difference =>
                if unary {
                    Self::unary(OperatorKind::Negate, 5)
                } else {
                    self
                }
            OperatorKind::Sum =>
                if unary {
                    Self::unary(OperatorKind::Positive, 5)
                } else {
                    self
                },
            OperatorKind::Negate =>
                if unary {
                    self
                } else {
                    Self::binary(OperatorKind::Difference, 2)
                }
            OperatorKind::Positive =>
                if unary {
                    self
                } else {
                    Self::binary(OperatorKind::Sum, 2)
                }
            _ => self,
        };

        Ok(out)
    }

    pub fn is_logical(&self) -> bool {
        match self.kind {
            OperatorKind::LogicalOr |
            OperatorKind::LogicalAnd |
            OperatorKind::LogicalNot |
            OperatorKind::Equals |
            OperatorKind::Different |
            OperatorKind::GreaterThan |
            OperatorKind::GreaterThanEqual |
            OperatorKind::LessThan |
            OperatorKind::LessThanEqual => true,
            _ => false
        }
    }
}

impl Operator {
    pub fn from(str: &String) -> Result<Self> {
        match str.as_str() {
            "+" => Ok(Self::binary(OperatorKind::Sum, 2)),
            "-" => Ok(Self::binary(OperatorKind::Difference, 2)),
            "*" => Ok(Self::binary(OperatorKind::Product, 3)),
            "/" => Ok(Self::binary(OperatorKind::Quotient, 3)),
            "^" | "**" => Ok(Self::binary(OperatorKind::Exp, 4)),
            "!" => Ok(Self::unary(OperatorKind::LogicalNot, 4)),
            "&&" => Ok(Self::binary(OperatorKind::LogicalAnd, 3)),
            "||" => Ok(Self::binary(OperatorKind::LogicalOr, 3)),
            "==" => Ok(Self::binary(OperatorKind::Equals, 1)),
            "!=" => Ok(Self::binary(OperatorKind::Different, 1)),
            ">" => Ok(Self::binary(OperatorKind::GreaterThan, 1)),
            ">=" => Ok(Self::binary(OperatorKind::GreaterThanEqual, 1)),
            "<" => Ok(Self::binary(OperatorKind::LessThan, 1)),
            "<=" => Ok(Self::binary(OperatorKind::LessThanEqual, 1)),
            "=" => Ok(Self::binary(OperatorKind::Assignment, 0)),
            "+=" => Ok(Self::binary(OperatorKind::PlusEqual, 0)),
            "-=" => Ok(Self::binary(OperatorKind::MinusEqual, 0)),
            "*=" => Ok(Self::binary(OperatorKind::TimesEqual, 0)),
            "/=" => Ok(Self::binary(OperatorKind::DivideEqual, 0)),
            "++" => Ok(Self::unary(OperatorKind::Increment, 6)),
            "--" => Ok(Self::unary(OperatorKind::Decrement, 6)),
            ".." => Ok(Self::binary(OperatorKind::Range,5)),
            str => Err(anyhow!("Unknown Operator {str}"))
        }
    }

    fn unary(kind: OperatorKind, precedence: u8) -> Self {
        Self {
            kind,
            precedence,
            arity: 1,
        }
    }

    fn binary(kind: OperatorKind, precedence: u8) -> Self {
        Self {
            kind,
            precedence,
            arity: 2,
        }
    }
}

impl Display for OperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let representation = match self {
            OperatorKind::Exp => "^",
            OperatorKind::Product => "*",
            OperatorKind::Quotient => "/",
            OperatorKind::Difference | OperatorKind::Negate => "-",
            OperatorKind::Sum | OperatorKind::Positive => "+",
            OperatorKind::LogicalOr => "||",
            OperatorKind::LogicalAnd => "&&",
            OperatorKind::LogicalNot => "!",
            OperatorKind::Equals => "==",
            OperatorKind::Different => "!=",
            OperatorKind::GreaterThan => ">",
            OperatorKind::GreaterThanEqual => ">=",
            OperatorKind::LessThan => "<",
            OperatorKind::LessThanEqual => "<=",
            OperatorKind::Increment => "++",
            OperatorKind::Decrement => "--",
            OperatorKind::Assignment => "=",
            OperatorKind::PlusEqual => "+=",
            OperatorKind::MinusEqual => "-=",
            OperatorKind::TimesEqual => "*=",
            OperatorKind::DivideEqual => "/=",
            OperatorKind::Range => "..",
        };
        write!(f, "{representation}")
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.to_string())
    }
}
