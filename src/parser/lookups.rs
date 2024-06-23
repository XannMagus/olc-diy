use crate::lexer::{Keyword, OperatorKind, Token};
use crate::parser::ast::{Expr, Stmt};
use crate::parser::binding_power::BindingPower;
use crate::parser::Parser;

impl Parser {
    pub(super) fn nud(token: &Token) -> Option<fn(&mut Self) -> anyhow::Result<Expr>> {
        match token {
            Token::NumericLiteral(_)
            | Token::StringLiteral(_)
            | Token::Symbol(_)
            | Token::Keyword(_) => Some(Self::parse_primary_expr),
            Token::Operator(OperatorKind::Difference) => Some(Self::parse_prefix_expression),
            Token::OpeningParenthesis => Some(Self::parse_grouping_expression),
            _ => None,
        }
    }

    pub(super) fn led(token: &Token) -> Option<fn(&mut Self, Expr, BindingPower) -> anyhow::Result<Expr>> {
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
                | OperatorKind::LessThanEqual => Some(Self::parse_binary_expression),

                OperatorKind::Assignment
                | OperatorKind::PlusEqual
                | OperatorKind::MinusEqual
                | OperatorKind::TimesEqual
                | OperatorKind::DivideEqual => Some(Self::parse_assignment_expression),
                _ => None,
            },
            _ => None,
        }
    }

    pub(super) fn stmt(token: &Token) -> Option<fn(&mut Self) -> anyhow::Result<Stmt>> {
        match token {
            Token::Keyword(keyword) => match keyword {
                Keyword::Const | Keyword::Let => Some(Self::parse_variable_declaration_stmt),
                _ => None,
            },
            _ => None,
        }
    }

}