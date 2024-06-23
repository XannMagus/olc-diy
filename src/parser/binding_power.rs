use crate::lexer::{OperatorKind, Token};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum BindingPower {
    Default,
    Separator,
    Assignment,
    Logical,
    Relational,
    Additive,
    Multiplicative,
    Exponent,
    Unary,
    Call,
    Member,
    Primary,
}

impl From<OperatorKind> for BindingPower {
    fn from(value: OperatorKind) -> Self {
        match value {
            OperatorKind::Increment | OperatorKind::Decrement | OperatorKind::LogicalNot => {
                Self::Unary
            }

            OperatorKind::Exp => Self::Exponent,

            OperatorKind::Product | OperatorKind::Quotient | OperatorKind::Modulo => {
                Self::Multiplicative
            }

            OperatorKind::Difference | OperatorKind::Sum => Self::Additive,

            OperatorKind::Equals
            | OperatorKind::Different
            | OperatorKind::GreaterThan
            | OperatorKind::GreaterThanEqual
            | OperatorKind::LessThan
            | OperatorKind::LessThanEqual => Self::Relational,

            OperatorKind::LogicalOr | OperatorKind::LogicalAnd | OperatorKind::Range => {
                Self::Logical
            }

            OperatorKind::Assignment
            | OperatorKind::PlusEqual
            | OperatorKind::MinusEqual
            | OperatorKind::TimesEqual
            | OperatorKind::DivideEqual => Self::Assignment,
        }
    }
}

impl From<Token> for BindingPower {
    fn from(value: Token) -> Self {
        match value {
            Token::NumericLiteral(_) | Token::StringLiteral(_) | Token::Symbol(_) => Self::Primary,
            Token::Operator(op) => op.into(),
            _ => Self::Default,
        }
    }
}

impl From<Option<Token>> for BindingPower {
    fn from(value: Option<Token>) -> Self {
        match value {
            None => Self::Default,
            Some(token) => token.into(),
        }
    }
}
