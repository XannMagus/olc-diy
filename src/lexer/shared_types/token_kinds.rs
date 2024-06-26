use std::fmt::{Display, Formatter};
use crate::lexer::shared_types::operators::Operator;


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenKind {
    NumericLiteral,
    StringLiteral,
    Symbol,
    Operator(Operator),
    Separator,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningScope,
    ClosingScope,
    EndOfStatement,
    Keyword,
    Unknown,
}


// Display Implementation
impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenKind::NumericLiteral => "[LITERAL, NUMERIC  ]",
            TokenKind::StringLiteral => "[LITERAL, STRING   ]",
            TokenKind::Symbol => "[SYMBOL            ]",
            TokenKind::Operator { .. } => "[OPERATOR          ]",
            TokenKind::Separator => "[SEPARATOR         ]",
            TokenKind::OpeningParenthesis => "[PARENTHESIS, OPEN ]",
            TokenKind::ClosingParenthesis => "[PARENTHESIS, CLOSE]",
            TokenKind::Unknown => "[UNKNOWN           ]",
            TokenKind::OpeningScope => "[SCOPE, OPEN       ]",
            TokenKind::ClosingScope => "[SCOPE, CLOSE      ]",
            TokenKind::EndOfStatement => "[END OF STATEMENT  ]",
            TokenKind::Keyword => "[KEYWORD           ]",
        };
        write!(f, "{str}")
    }
}
