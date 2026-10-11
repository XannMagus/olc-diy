use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperatorKind {
    // Arithmetic
    Exp,
    Product,
    Quotient,
    Difference,
    Sum,
    Increment,
    Decrement,
    Modulo,
    // Logical
    LogicalOr,
    LogicalAnd,
    LogicalNot,
    //Relational
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

impl Display for OperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let representation = match self {
            OperatorKind::Exp => "^",
            OperatorKind::Product => "*",
            OperatorKind::Quotient => "/",
            OperatorKind::Difference => "-",
            OperatorKind::Sum => "+",
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
            OperatorKind::Modulo => "%",
        };
        write!(f, "{representation}")
    }
}
