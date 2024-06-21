use std::fmt::{Debug, Display, Formatter};
use crate::lexer::shared_types::keywords::Keyword;
use crate::lexer::shared_types::operators::Operator;


#[derive(PartialEq, Clone)]
pub enum Token {
    NumericLiteral(f64),
    StringLiteral(String),
    Symbol(String),
    Operator(Operator),
    Colon,
    Separator,
    ScopeSeparator,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningScope,
    ClosingScope,
    OpeningBracket,
    ClosingBracket,
    EndOfStatement,
    Keyword(Keyword),
    Unknown,
}


// Display Implementation
impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Token::NumericLiteral(value) => format!("[LITERAL, NUMERIC  ] {value}"),
            Token::StringLiteral(value) => format!("[LITERAL, STRING   ] {value}"),
            Token::Symbol(name) => format!("[SYMBOL            ] {name}"),
            Token::Operator(op) => format!("[OPERATOR          ] {op}"),
            Token::Separator => "[SEPARATOR, GENERAL] ,".to_string(),
            Token::ScopeSeparator => "[SEPARATOR, SCOPE  ] .".to_string(),
            Token::Colon => "[COLON             ] :".to_string(),
            Token::OpeningParenthesis => "[PARENTHESIS, OPEN ] (".to_string(),
            Token::ClosingParenthesis => "[PARENTHESIS, CLOSE] )".to_string(),
            Token::OpeningBracket => "[BRACKET, OPEN     ] [".to_string(),
            Token::ClosingBracket => "[BRACKET, CLOSE    ] ]".to_string(),
            Token::Unknown => "[UNKNOWN           ]".to_string(),
            Token::OpeningScope => "[SCOPE, OPEN       ] {".to_string(),
            Token::ClosingScope => "[SCOPE, CLOSE      ] }".to_string(),
            Token::EndOfStatement => "[END OF STATEMENT  ] ;".to_string(),
            Token::Keyword(keyword) => format!("[KEYWORD           ] {keyword}"),
        };
        write!(f, "{str}")
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Token::NumericLiteral(value) => format!("{value}"),
            Token::StringLiteral(value) => format!("\"{value}\""),
            Token::Symbol(name) => format!("{name}"),
            Token::Operator(op) => format!("{op}"),
            Token::Colon => ':'.to_string(),
            Token::Separator => ','.to_string(),
            Token::ScopeSeparator => '.'.to_string(),
            Token::OpeningParenthesis => '('.to_string(),
            Token::ClosingParenthesis => ')'.to_string(),
            Token::OpeningScope => '{'.to_string(),
            Token::ClosingScope => '}'.to_string(),
            Token::OpeningBracket => '['.to_string(),
            Token::ClosingBracket => ']'.to_string(),
            Token::EndOfStatement => ';'.to_string(),
            Token::Keyword(keyword) => format!("{keyword}"),
            Token::Unknown => '?'.to_string(),
        };
        write!(f, "{str}")
    }
}