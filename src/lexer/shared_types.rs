use std::fmt::{Display, Formatter};
use crate::lexer::shared_types::keywords::Keyword;
use crate::lexer::shared_types::operators::Operator;
pub use crate::lexer::shared_types::token_kinds::TokenKind;

pub mod states;
pub mod token_kinds;
pub mod operators;
pub mod keywords;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    value: Option<f64>,
    id: String,
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn update_kind(&mut self, new_kind: TokenKind) {
        self.kind = new_kind;
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn as_string(&self) -> String {
        self.id.clone()
    }
}

// Private Methods
impl Token {
    fn new() -> Self {
        Self {
            kind: TokenKind::Unknown,
            value: None,
            id: "".to_string(),
        }
    }
    fn from_digits(str: &String) -> Self {
        Self {
            kind: TokenKind::NumericLiteral,
            value: Some(str.parse::<f64>().unwrap()),
            id: str.clone(),
        }
    }

    fn from_hex(str: &String) -> Self {
        let hex_representation = str.trim_start_matches("0x");
        let int_value = u64::from_str_radix(hex_representation, 16).unwrap_or(0);
        let value = Some(int_value as f64);

        Self {
            kind: TokenKind::NumericLiteral,
            value,
            id: str.clone(),
        }
    }

    fn from_bin(str: &String) -> Self {
        let hex_representation = str.trim_start_matches("0b");
        let int_value = u64::from_str_radix(hex_representation, 2).unwrap_or(0);
        let value = Some(int_value as f64);

        Self {
            kind: TokenKind::NumericLiteral,
            value,
            id: str.clone(),
        }
    }

    fn from_str(str: &String) -> Self {
        Self {
            kind: TokenKind::StringLiteral,
            value: None,
            id: str.clone(),
        }
    }

    fn from_operator(op: Operator) -> Self {
        Self {
            kind: TokenKind::Operator(op),
            value: None,
            id: op.to_string(),
        }
    }

    fn from_keyword(keyword: Keyword) -> Self {
        Self {
            kind: TokenKind::Keyword(keyword),
            value: None,
            id: keyword.to_string(),
        }
    }

    fn open_parenthesis() -> Self {
        Self {
            kind: TokenKind::OpeningParenthesis,
            value: None,
            id: "(".to_string(),
        }
    }

    fn close_parenthesis() -> Self {
        Self {
            kind: TokenKind::ClosingParenthesis,
            value: None,
            id: ")".to_string(),
        }
    }

    fn open_bracket() -> Self {
        Self {
            kind: TokenKind::OpeningBracket,
            value: None,
            id: "[".to_string(),
        }
    }

    fn close_bracket() -> Self {
        Self {
            kind: TokenKind::ClosingBracket,
            value: None,
            id: "]".to_string(),
        }
    }

    fn open_scope() -> Self {
        Self {
            kind: TokenKind::OpeningScope,
            value: None,
            id: "{".to_string(),
        }
    }

    fn close_scope() -> Self {
        Self {
            kind: TokenKind::ClosingScope,
            value: None,
            id: "}".to_string(),
        }
    }

    fn separator() -> Self {
        Self {
            kind: TokenKind::Separator,
            value: None,
            id: ",".to_string(),
        }
    }

    fn colon() -> Self {
        Self {
            kind: TokenKind::Colon,
            value: None,
            id: ":".to_string(),
        }
    }

    fn end_of_statement() -> Self {
        Self {
            kind: TokenKind::EndOfStatement,
            value: None,
            id: ";".to_string(),
        }
    }

    fn symbol(name: &String) -> Self {
        Self {
            kind: TokenKind::Symbol,
            value: None,
            id: name.clone(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {} ({})", self.kind, self.id, self.value.unwrap_or(0.0))
    }
}
