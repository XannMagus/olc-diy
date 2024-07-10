use anyhow::Result;
use crate::lexer::{Keyword, OperatorKind, Token};
use crate::parser::ast::{Expr, Stmt, Type};
use crate::parser::binding_power::BindingPower;
use crate::parser::Parser;

type NudHandler = fn(&mut Parser) -> Result<Expr>;
type LedHandler = fn(&mut Parser, Expr, BindingPower) -> Result<Expr>;
type StmtHandler = fn(&mut Parser) -> Result<Stmt>;
type TypeNudHandler = fn(&mut Parser) -> Result<Type>;
type TypeLedHandler = fn(&mut Parser, Type, BindingPower) -> Result<Type>;

impl Parser {
    pub(super) fn nud(token: &Token) -> Option<NudHandler> {
        match token {
            Token::NumericLiteral(_)
            | Token::StringLiteral(_)
            | Token::Symbol(_) => Some(Self::parse_primary_expr),
            Token::Operator(OperatorKind::Difference) => Some(Self::parse_prefix_expression),
            Token::OpeningParenthesis => Some(Self::parse_grouping_expression),
            _ => None,
        }
    }

    pub(super) fn led(token: &Token) -> Option<LedHandler> {
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

    pub(super) fn stmt(token: &Token) -> Option<StmtHandler> {
        match token {
            Token::Keyword(keyword) => match keyword {
                Keyword::Const | Keyword::Let => Some(Self::parse_variable_declaration_stmt),
                Keyword::Struct => Some(Self::parse_struct_definition_stmt),
                _ => None,
            },
            _ => None,
        }
    }

    pub(super) fn type_nud(token: &Token) -> Option<TypeNudHandler> {
        match token {
            Token::Symbol(_) => Some(Self::parse_symbol_type),
            Token::OpeningBracket => Some(Self::parse_array_type),
            _ => None,
        }
    }

    pub(super) fn type_led(_token: &Token) -> Option<TypeLedHandler> {
        None
    }
}