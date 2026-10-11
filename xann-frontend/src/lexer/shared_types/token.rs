use std::fmt::{Debug, Display, Formatter};

use crate::lexer::shared_types::keywords::Keyword;
use crate::lexer::shared_types::operators::OperatorKind;

#[derive(PartialEq, Clone)]
pub enum Token {
    NumericLiteral(f64),
    StringLiteral(String),
    Symbol(String),
    Operator(OperatorKind),
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
    EndOfFile,
    Unknown,
}

impl Token {
    pub fn is_eof(&self) -> bool {
        *self == Self::EndOfFile
    }

    pub fn match_variant(&self, expected: &Token) -> bool {
        match (self, expected) {
            (Token::NumericLiteral(_), Token::NumericLiteral(_))
            | (Token::StringLiteral(_), Token::StringLiteral(_))
            | (Token::Symbol(_), Token::Symbol(_))
            | (Token::Colon, Token::Colon)
            | (Token::Separator, Token::Separator)
            | (Token::ScopeSeparator, Token::ScopeSeparator)
            | (Token::OpeningParenthesis, Token::OpeningParenthesis)
            | (Token::ClosingParenthesis, Token::ClosingParenthesis)
            | (Token::OpeningScope, Token::OpeningScope)
            | (Token::ClosingScope, Token::ClosingScope)
            | (Token::OpeningBracket, Token::OpeningBracket)
            | (Token::ClosingBracket, Token::ClosingBracket)
            | (Token::EndOfStatement, Token::EndOfStatement)
            | (Token::EndOfFile, Token::EndOfFile)
            | (Token::Unknown, Token::Unknown) => true,
            (Token::Operator(op1), Token::Operator(op2)) => op1 == op2,
            (Token::Keyword(kw1), Token::Keyword(kw2)) => kw1 == kw2,
            _ => false,
        }
    }
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
            Token::EndOfFile => "[END OF FILE       ]",
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
            Token::EndOfFile => "EOF".to_string(),
        };
        write!(f, "{str}")
    }
}
