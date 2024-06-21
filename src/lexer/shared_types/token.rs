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
            Token::NumericLiteral(_) => "[LITERAL, NUMERIC  ]",
            Token::StringLiteral(_) => "[LITERAL, STRING   ]",
            Token::Symbol(_) => "[SYMBOL            ]",
            Token::Operator(_) => "[OPERATOR          ]",
            Token::Separator => "[SEPARATOR, GENERAL]",
            Token::ScopeSeparator => "[SEPARATOR, SCOPE  ]",
            Token::Colon => "[COLON             ]",
            Token::OpeningParenthesis => "[PARENTHESIS, OPEN ]",
            Token::ClosingParenthesis => "[PARENTHESIS, CLOSE]",
            Token::OpeningBracket => "[BRACKET, OPEN     ]",
            Token::ClosingBracket => "[BRACKET, CLOSE    ]",
            Token::Unknown => "[UNKNOWN           ]",
            Token::OpeningScope => "[SCOPE, OPEN       ]",
            Token::ClosingScope => "[SCOPE, CLOSE      ]",
            Token::EndOfStatement => "[END OF STATEMENT  ]",
            Token::Keyword(_) => "[KEYWORD           ]",
        };
        write!(f, "{str} {self}")
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