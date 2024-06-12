use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Result};

const NUMERIC_DIGITS: [bool; 256] = make_lut("0123456789");
const REAL_NUMERIC_DIGITS: [bool; 256] = make_lut(".0123456789");
const HEX_NUMERIC_DIGITS: [bool; 256] = make_lut("0123456789ABCDEFabcdef");
const BINARY_NUMERIC_DIGITS: [bool; 256] = make_lut("01");
const WHITESPACE: [bool; 256] = make_lut(" \t\n\r\x0C");
const OPERATOR_CHARACTERS: [bool; 256] = make_lut("!$%^&*+-=#@?|`/\\<>~");
const SYMBOL_CHARACTERS: [bool; 256] = make_lut("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    NewToken,
    CompleteToken,
    NumericLiteral,
    FancyNumericLiteral,
    BinaryNumericLiteral,
    HexNumericLiteral,
    StringLiteral,
    Operator,
    ParenthesisOpen,
    ParenthesisClose,
    ScopeOpen,
    ScopeClose,
    Separator,
    EndOfStatement,
    SymbolName,
}

pub struct Tokenizer {
    input: String,

    current_state: State,
    next_state: State,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    value: Option<f64>,
    id: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Operator {
    kind: OperatorKind,
    precedence: u8,
    arity: u8,
}


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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperatorKind {
    Exp,
    Product,
    Quotient,
    Difference,
    Sum,
    Negate,
    Positive,
}

pub type TokenQueue = VecDeque<Token>;

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            current_state: State::NewToken,
            next_state: State::NewToken,
        }
    }

    pub fn parse(mut self) -> Result<TokenQueue> {
        if self.input.is_empty() {
            return Err(anyhow!("[PARSER] No input provided"));
        }

        let mut output = TokenQueue::new();

        let mut current_token_string = String::new();
        let mut current_token = Token::new();
        let mut decimal_point_found = false;

        let mut chars = self.input.chars().peekable();

        let mut parentheses_balance_checker = 0;
        let mut scope_balance_checker = 0;

        while let Some(&c) = chars.peek() {
            match self.current_state {
                State::NewToken => {
                    current_token_string.clear();
                    current_token = Token::new();
                    decimal_point_found = false;

                    if WHITESPACE[c as usize] {
                        chars.next();
                        self.next_state = State::NewToken;
                    } else if NUMERIC_DIGITS[c as usize] {
                        self.next_state = if c == '0' {
                            current_token_string.push(c);
                            chars.next();
                            State::FancyNumericLiteral
                        } else {
                            State::NumericLiteral
                        };
                    } else if OPERATOR_CHARACTERS[c as usize] {
                        self.next_state = State::Operator;
                    } else if '(' == c {
                        self.next_state = State::ParenthesisOpen;
                    } else if ')' == c {
                        self.next_state = State::ParenthesisClose;
                    } else if '{' == c {
                        self.next_state = State::ScopeOpen;
                    } else if '}' == c {
                        self.next_state = State::ScopeClose;
                    } else if ',' == c {
                        self.next_state = State::Separator;
                    } else if ';' == c {
                        self.next_state = State::EndOfStatement;
                    } else if '"' == c {
                        chars.next();
                        self.next_state = State::StringLiteral;
                    } else {
                        self.next_state = State::SymbolName;
                    }
                }
                State::CompleteToken => {
                    output.push_back(current_token.clone());
                    self.next_state = State::NewToken;
                }
                State::NumericLiteral => {
                    if REAL_NUMERIC_DIGITS[c as usize] {
                        if c == '.' {
                            if decimal_point_found {
                                return Err(anyhow!("[PARSER] Multiple decimal separator found in the same numeric token"));
                            } else {
                                decimal_point_found = true;
                            }
                        }
                        current_token_string.push(c);
                        chars.next();

                        if chars.peek().is_some() {
                            self.next_state = State::NumericLiteral;
                        } else {
                            self.next_state = State::CompleteToken;
                            current_token = Token::from_digits(&current_token_string);
                        }
                    } else {
                        if SYMBOL_CHARACTERS[c as usize] {
                            return Err(anyhow!("[PARSER] Invalid number/symbol"));
                        }
                        self.next_state = State::CompleteToken;
                        current_token = Token::from_digits(&current_token_string);
                    }
                },
                State::StringLiteral => {
                    if '"' != c {
                        current_token_string.push(c);
                        chars.next();
                    } else {
                        chars.next();
                        current_token = Token::from_str(&current_token_string);
                        self.next_state = State::CompleteToken;
                    }
                }
                State::SymbolName => {
                    if SYMBOL_CHARACTERS[c as usize] {
                        current_token_string.push(c);
                        chars.next();

                        if chars.peek().is_some() {
                            self.next_state = State::SymbolName;
                        } else {
                            self.next_state = State::CompleteToken;
                            current_token = Token::symbol(&current_token_string);
                        }
                    } else {
                        // todo! Manage Keywords here
                        self.next_state = State::CompleteToken;
                        current_token = Token::symbol(&current_token_string);
                    }
                }
                State::Operator => {
                    if OPERATOR_CHARACTERS[c as usize] {
                        let mut tmp_op = current_token_string.clone();
                        tmp_op.push(c);
                        if let Ok(_) = Operator::from(&tmp_op) {
                            current_token_string.push(c);
                            chars.next();
                        } else {
                            if let Ok(operator) = Operator::from(&current_token_string) {
                                current_token = Token::from_operator(operator);
                                self.next_state = State::CompleteToken;
                            } else {
                                current_token_string.push(c);
                                chars.next();
                            }
                        }
                    } else {
                        if let Ok(operator) = Operator::from(&current_token_string) {
                            current_token = Token::from_operator(operator);
                            self.next_state = State::CompleteToken;
                        } else {
                            return Err(anyhow!("[PARSER] Unrecognized operator: {current_token_string}"));
                        }
                    }
                }
                State::FancyNumericLiteral => {
                    if 'x' == c {
                        current_token_string.push(c);
                        chars.next();
                        self.next_state = State::HexNumericLiteral;
                    } else if 'b' == c {
                        current_token_string.push(c);
                        chars.next();
                        self.next_state = State::BinaryNumericLiteral;
                    } else if REAL_NUMERIC_DIGITS[c as usize] {
                        self.next_state = State::NumericLiteral;
                    } else {
                        return Err(anyhow!("[PARSER] Bad numeric literal"));
                    }
                }
                State::BinaryNumericLiteral => {
                    if BINARY_NUMERIC_DIGITS[c as usize] {
                        current_token_string.push(c);
                        chars.next();

                        if chars.peek().is_some() {
                            self.next_state = State::BinaryNumericLiteral;
                        } else {
                            self.next_state = State::CompleteToken;
                            current_token = Token::from_bin(&current_token_string);
                        }
                    } else {
                        if SYMBOL_CHARACTERS[c as usize] || '.' == c {
                            return Err(anyhow!("[PARSER] Invalid binary number"));
                        }
                        self.next_state = State::CompleteToken;
                        current_token = Token::from_bin(&current_token_string);
                    }
                }
                State::HexNumericLiteral => {
                    if HEX_NUMERIC_DIGITS[c as usize] {
                        current_token_string.push(c);
                        chars.next();

                        if chars.peek().is_some() {
                            self.next_state = State::HexNumericLiteral;
                        } else {
                            self.next_state = State::CompleteToken;
                            current_token = Token::from_hex(&current_token_string);
                        }
                    } else {
                        if SYMBOL_CHARACTERS[c as usize] || '.' == c {
                            return Err(anyhow!("[PARSER] Invalid hex number"));
                        }
                        self.next_state = State::CompleteToken;
                        current_token = Token::from_hex(&current_token_string);
                    }
                }
                State::ParenthesisOpen => {
                    current_token_string.push(c);
                    chars.next();
                    parentheses_balance_checker += 1;
                    current_token = Token::open_parenthesis();
                    self.next_state = State::CompleteToken;
                }
                State::ParenthesisClose => {
                    current_token_string.push(c);
                    chars.next();
                    parentheses_balance_checker -= 1;
                    current_token = Token::close_parenthesis();
                    self.next_state = State::CompleteToken;
                }
                State::ScopeOpen => {
                    current_token_string.push(c);
                    chars.next();
                    scope_balance_checker += 1;
                    current_token = Token::open_scope();
                    self.next_state = State::CompleteToken;
                }
                State::ScopeClose => {
                    current_token_string.push(c);
                    chars.next();
                    scope_balance_checker -= 1;
                    current_token = Token::close_scope();
                    self.next_state = State::CompleteToken;
                }
                State::Separator => {
                    chars.next();
                    current_token = Token::separator();
                    self.next_state = State::CompleteToken;
                }
                State::EndOfStatement => {
                    chars.next();
                    current_token = Token::end_of_statement();
                    self.next_state = State::CompleteToken;
                }
            }
            self.current_state = self.next_state.clone();
        }

        if self.current_state == State::CompleteToken {
            output.push_back(current_token.clone());
        } else if self.current_state == State::StringLiteral {
            return Err(anyhow!("[PARSER] Missing quotation mark \""));
        }

        if parentheses_balance_checker != 0 {
            return Err(anyhow!("[PARSER] Parentheses '(' & ')' are not balanced"));
        }
        if scope_balance_checker != 0 {
            return Err(anyhow!("[PARSER] Scope brackets '{{' & '}}' are not balanced"));
        }

        Ok(output)
    }
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

impl Operator {
    pub fn arity(&self) -> u8 {
        self.arity
    }

    pub fn precedence(&self) -> u8 {
        self.precedence
    }

    pub fn compute_2(&self, left: f64, right: f64) -> f64 {
        match self.kind {
            OperatorKind::Exp => left.powf(right),
            OperatorKind::Product => left * right,
            OperatorKind::Quotient => left / right,
            OperatorKind::Difference => left - right,
            OperatorKind::Sum => left + right,
            _ => 0.0
        }
    }

    pub fn compute_1(&self, operand: f64) -> f64 {
        match self.kind {
            OperatorKind::Negate => -operand,
            OperatorKind::Positive => operand,
            _ => 0.0
        }
    }

    pub fn correct_arity(self, previous: &Option<Token>) -> Result<Self> {
        let unary = if let Some(previous) = previous {
            previous.kind != TokenKind::NumericLiteral && previous.kind != TokenKind::ClosingParenthesis
        } else {
            true
        };

        let out = match self.kind {
            OperatorKind::Exp | OperatorKind::Product | OperatorKind::Quotient => self,
            OperatorKind::Difference =>
                if unary {
                    Self::unary(OperatorKind::Negate, 5)
                } else {
                    self
                }
            OperatorKind::Sum =>
                if unary {
                    Self::unary(OperatorKind::Positive, 5)
                } else {
                    self
                },
            OperatorKind::Negate =>
                if unary {
                    self
                } else {
                    Self::binary(OperatorKind::Difference, 2)
                }
            OperatorKind::Positive =>
                if unary {
                    self
                } else {
                    Self::binary(OperatorKind::Sum, 2)
                }
        };

        Ok(out)
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

impl Operator {
    fn from(str: &String) -> Result<Self> {
        match str.as_str() {
            "+" => Ok(Self::binary(OperatorKind::Sum, 2)),
            "-" => Ok(Self::binary(OperatorKind::Difference, 2)),
            "*" => Ok(Self::binary(OperatorKind::Product, 3)),
            "/" => Ok(Self::binary(OperatorKind::Quotient, 3)),
            "^" | "**" => Ok(Self::binary(OperatorKind::Exp, 4)),
            str => Err(anyhow!("Unknown Operator {str}"))
        }
    }

    fn unary(kind: OperatorKind, precedence: u8) -> Self {
        Self {
            kind,
            precedence,
            arity: 1,
        }
    }

    fn binary(kind: OperatorKind, precedence: u8) -> Self {
        Self {
            kind,
            precedence,
            arity: 2,
        }
    }
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

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {} ({})", self.kind, self.id, self.value.unwrap_or(0.0))
    }
}

impl Display for OperatorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let representation = match self {
            OperatorKind::Exp => "^",
            OperatorKind::Product => "*",
            OperatorKind::Quotient => "/",
            OperatorKind::Difference | OperatorKind::Negate => "-",
            OperatorKind::Sum | OperatorKind::Positive=> "+",
        };
        write!(f, "{representation}")
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.to_string())
    }
}


// Utilities

pub fn display_queue(queue: &TokenQueue) -> String {
    queue.iter().fold(
        String::new(),
        |mut acc, x| {
            if !acc.is_empty() {
                acc.push('\n');
            }
            acc.push_str(&x.to_string());
            acc
        },
    )
}

const fn make_lut(s: &str) -> [bool; 256] {
    let mut lookup_table = [false; 256];
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        lookup_table[bytes[i] as usize] = true;
        i += 1;
    }
    lookup_table
}