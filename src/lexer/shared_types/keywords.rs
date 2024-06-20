use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Keyword {
    True,
    False,
    Null,
    Let,
    Const,
    Class,
    New,
    Import,
    From,
    Function,
    If,
    Else,
    Foreach,
    While,
    For,
    Export,
    Typeof,
    In,
}

impl Keyword {
    pub fn new(str: &String) -> Option<Self> {
        match str.as_str() {
            "true" => Some(Self::True),
            "false" => Some(Self::False),
            "null" => Some(Self::Null),
            "let" => Some(Self::Let),
            "const" => Some(Self::Const),
            "class" => Some(Self::Class),
            "new" => Some(Self::New),
            "import" => Some(Self::Import),
            "from" => Some(Self::From),
            "fn" => Some(Self::Function),
            "if" => Some(Self::If),
            "else" => Some(Self::Else),
            "foreach" => Some(Self::Foreach),
            "while" => Some(Self::While),
            "for" => Some(Self::For),
            "export" => Some(Self::Export),
            "typeof" => Some(Self::Typeof),
            "in" => Some(Self::In),
            &_ => None
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}