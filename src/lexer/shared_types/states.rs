use std::iter::Peekable;
use std::str::Chars;

use anyhow::{anyhow, Result};
use crate::lexer::shared_types::operators::Operator;
use crate::lexer::shared_types::Token;
use crate::lexer::TokenQueue;

pub trait State {
    fn handle<'a>(self: Box<Self>, temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)>;
    fn is_final(&self) -> bool;
}

pub struct StartState;

pub struct EndState;

struct NewToken;

struct CompleteToken;

struct NumericLiteral;

struct FancyNumericLiteral;

struct BinaryNumericLiteral;

struct HexNumericLiteral;

struct StringLiteral;

struct OperatorState;

struct ParenthesisOpen;

struct ParenthesisClose;

struct ScopeOpen;

struct ScopeClose;

struct Separator;

struct EndOfStatement;

struct SymbolName;

#[derive(Debug)]
pub struct TemporaryData<'a> {
    input: String,
    output: TokenQueue,
    chars: Peekable<Chars<'a>>,
    current_token_string: String,
    current_token: Token,

    decimal_point_found: bool,
    paren_balance_check: u8,
    scope_balance_check: u8,
}

impl State for StartState {
    fn handle<'a>(self: Box<StartState>, temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        if temporary_data.input.is_empty() {
            Err(anyhow!("[PARSER] No input provided"))
        } else {
            Ok((Box::new(NewToken), temporary_data))
        }
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for NewToken {
    fn handle<'a>(self: Box<NewToken>, mut temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        temporary_data.current_token_string.clear();
        temporary_data.current_token = Token::new();
        temporary_data.decimal_point_found = false;

        if let Some(&c) = temporary_data.chars.peek() {
            if WHITESPACE[c as usize] {
                temporary_data.chars.next();
                Ok((Box::new(Self), temporary_data))
            } else if NUMERIC_DIGITS[c as usize] {
                Ok((if c == '0' {
                    temporary_data.current_token_string.push(c);
                    temporary_data.chars.next();
                    Box::new(FancyNumericLiteral)
                } else {
                    Box::new(NumericLiteral)
                }, temporary_data))
            } else if OPERATOR_CHARACTERS[c as usize] {
                Ok((Box::new(OperatorState), temporary_data))
            } else if '(' == c {
                Ok((Box::new(ParenthesisOpen), temporary_data))
            } else if ')' == c {
                Ok((Box::new(ParenthesisClose), temporary_data))
            } else if '{' == c {
                Ok((Box::new(ScopeOpen), temporary_data))
            } else if '}' == c {
                Ok((Box::new(ScopeClose), temporary_data))
            } else if ',' == c {
                Ok((Box::new(Separator), temporary_data))
            } else if ';' == c {
                Ok((Box::new(EndOfStatement), temporary_data))
            } else if '"' == c {
                temporary_data.chars.next();
                Ok((Box::new(StringLiteral), temporary_data))
            } else {
                Ok((Box::new(SymbolName), temporary_data))
            }
        } else {
            Ok((Box::new(EndState), temporary_data))
        }
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for CompleteToken {
    fn handle<'a>(self: Box<CompleteToken>, mut temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        temporary_data.output.push_back(temporary_data.current_token.clone());
        if let Some(_) = temporary_data.chars.peek() {
            Ok((Box::new(NewToken), temporary_data))
        } else {
            Ok((Box::new(EndState), temporary_data))
        }
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for NumericLiteral {
    fn handle<'a>(self: Box<NumericLiteral>, mut temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        if let Some(&c) = temporary_data.chars.peek() {
            if REAL_NUMERIC_DIGITS[c as usize] {
                if '.' == c {
                    if temporary_data.decimal_point_found {
                        return Err(anyhow!("[PARSER] Multiple decimal separator found in the same numeric token"));
                    } else {
                        temporary_data.decimal_point_found = true;
                    }
                }
                temporary_data.current_token_string.push(c);
                temporary_data.chars.next();

                Ok((Box::new(Self), temporary_data))
            } else {
                if SYMBOL_CHARACTERS[c as usize] {
                    Err(anyhow!("[PARSER] Invalid number/symbol"))
                } else {
                    temporary_data.current_token = Token::from_digits(&temporary_data.current_token_string);
                    Ok((Box::new(CompleteToken), temporary_data))
                }
            }
        } else {
            temporary_data.current_token = Token::from_digits(&temporary_data.current_token_string);
            Ok((Box::new(CompleteToken), temporary_data))
        }
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for StringLiteral {
    fn handle<'a>(self: Box<StringLiteral>, mut temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        if let Some(&c) = temporary_data.chars.peek() {
            if '"' != c {
                temporary_data.current_token_string.push(c);
                temporary_data.chars.next();
                Ok((Box::new(Self), temporary_data))
            } else {
                temporary_data.chars.next();
                temporary_data.current_token = Token::from_str(&temporary_data.current_token_string);
                Ok((Box::new(CompleteToken), temporary_data))
            }
        } else {
            Err(anyhow!("[PARSER] Missing quotation mark \""))
        }
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for SymbolName {
    fn handle<'a>(self: Box<SymbolName>, mut temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        if let Some(&c) = temporary_data.chars.peek() {
            if SYMBOL_CHARACTERS[c as usize] {
                temporary_data.current_token_string.push(c);
                temporary_data.chars.next();
                Ok((Box::new(Self), temporary_data))
            } else {
                // todo! Handle Keywords
                temporary_data.current_token = Token::symbol(&temporary_data.current_token_string);
                Ok((Box::new(CompleteToken), temporary_data))
            }
        } else {
            temporary_data.current_token = Token::symbol(&temporary_data.current_token_string);
            Ok((Box::new(CompleteToken), temporary_data))
        }
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for OperatorState {
    fn handle<'a>(self: Box<OperatorState>, mut temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        if let Some(&c) = temporary_data.chars.peek() {
            if OPERATOR_CHARACTERS[c as usize] {
                let mut tmp_op = temporary_data.current_token_string.clone();
                tmp_op.push(c);
                if let Ok(_) = Operator::from(&tmp_op) {
                    temporary_data.current_token_string.push(c);
                    temporary_data.chars.next();
                    Ok((Box::new(Self), temporary_data))
                } else {
                    if let Ok(op) = Operator::from(&temporary_data.current_token_string) {
                        temporary_data.current_token = Token::from_operator(op);
                        Ok((Box::new(CompleteToken), temporary_data))
                    } else {
                        temporary_data.current_token_string.push(c);
                        temporary_data.chars.next();
                        Ok((Box::new(Self), temporary_data))
                    }
                }
            } else {
                if let Ok(op) = Operator::from(&temporary_data.current_token_string) {
                    temporary_data.current_token = Token::from_operator(op);
                    Ok((Box::new(CompleteToken), temporary_data))
                } else {
                    Err(anyhow!("[PARSER] unrecognized operator: {}", temporary_data.current_token_string))
                }
            }
        } else {
            Err(anyhow!("[PARSER] Operators should always be followed by another token"))
        }
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for FancyNumericLiteral {
    fn handle<'a>(self: Box<FancyNumericLiteral>, mut temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        if let Some(&c) = temporary_data.chars.peek() {
            if 'x' == c {
                temporary_data.current_token_string.push(c);
                temporary_data.chars.next();
                Ok((Box::new(HexNumericLiteral), temporary_data))
            } else if 'b' == c {
                temporary_data.current_token_string.push(c);
                temporary_data.chars.next();
                Ok((Box::new(BinaryNumericLiteral), temporary_data))
            } else if REAL_NUMERIC_DIGITS[c as usize] {
                Ok((Box::new(NumericLiteral), temporary_data))
            } else {
                Err(anyhow!("[PARSER] Bad numeric literal"))
            }
        } else {
            temporary_data.current_token = Token::from_digits(&temporary_data.current_token_string);
            Ok((Box::new(CompleteToken), temporary_data))
        }
    }

    fn is_final(&self) -> bool {
        false
    }
}

fn fancy_numeric_handler<'a, S: State + 'static>(mut temporary_data: TemporaryData<'a>, digits: [bool; 256], state: S, kind: &str, token_builder: fn(&String) -> Token) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
    if let Some(&c) = temporary_data.chars.peek() {
        if digits[c as usize] {
            temporary_data.current_token_string.push(c);
            temporary_data.chars.next();
            Ok((Box::new(state), temporary_data))
        } else if SYMBOL_CHARACTERS[c as usize] || '.' == c {
            Err(anyhow!("[PARSER] Invalid {kind} number"))
        } else {
            temporary_data.current_token = token_builder(&temporary_data.current_token_string);
            Ok((Box::new(CompleteToken), temporary_data))
        }
    } else {
        temporary_data.current_token = token_builder(&temporary_data.current_token_string);
        Ok((Box::new(CompleteToken), temporary_data))
    }
}

impl State for BinaryNumericLiteral {
    fn handle<'a>(self: Box<BinaryNumericLiteral>, temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        fancy_numeric_handler(temporary_data, BINARY_NUMERIC_DIGITS, Self, "binary", Token::from_bin)
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for HexNumericLiteral {
    fn handle<'a>(self: Box<HexNumericLiteral>, temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        fancy_numeric_handler(temporary_data, HEX_NUMERIC_DIGITS, Self, "hexadecimal", Token::from_hex)
    }

    fn is_final(&self) -> bool {
        false
    }
}

fn single_character_handler(mut temporary_data: TemporaryData, balancer: fn(temporary_data: &mut TemporaryData) -> (), token_builder: fn() -> Token) -> Result<(Box<dyn State>, TemporaryData)> {
    balancer(&mut temporary_data);
    temporary_data.chars.next();
    temporary_data.current_token = token_builder();
    Ok((Box::new(CompleteToken), temporary_data))
}

impl State for ParenthesisOpen {
    fn handle<'a>(self: Box<ParenthesisOpen>, temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        single_character_handler(temporary_data, |temp: &mut TemporaryData| { temp.paren_balance_check += 1; }, Token::open_parenthesis)
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for ParenthesisClose {

    fn handle<'a>(self: Box<ParenthesisClose>, temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        single_character_handler(temporary_data, |temp: &mut TemporaryData| { temp.paren_balance_check -= 1; }, Token::close_parenthesis)
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for ScopeOpen {

    fn handle<'a>(self: Box<ScopeOpen>, temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
   single_character_handler(temporary_data, |temp: &mut TemporaryData| { temp.scope_balance_check += 1; }, Token::open_scope)
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for ScopeClose {
    fn handle<'a>(self: Box<ScopeClose>, temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
            single_character_handler(temporary_data, |temp: &mut TemporaryData| { temp.scope_balance_check -= 1; }, Token::close_scope)
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for Separator {
    fn handle<'a>(self: Box<Separator>, temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        single_character_handler(temporary_data, |_: &mut TemporaryData| {}, Token::separator)
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for EndOfStatement {
    fn handle<'a>(self: Box<EndOfStatement>, temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        single_character_handler(temporary_data, |_: &mut TemporaryData| {}, Token::end_of_statement)
    }

    fn is_final(&self) -> bool {
        false
    }
}

impl State for EndState {
    fn handle<'a>(self: Box<EndState>, temporary_data: TemporaryData<'a>) -> Result<(Box<dyn State>, TemporaryData<'a>)> {
        if temporary_data.paren_balance_check != 0 {
            Err(anyhow!("[PARSER] Parentheses are not balanced"))
        } else if temporary_data.scope_balance_check != 0 {
            Err(anyhow!("[PARSER] Scope brackets are not balanced"))
        } else {
            Ok((Box::new(Self), temporary_data))
        }
    }

    fn is_final(&self) -> bool {
        true
    }
}


impl<'a> TemporaryData<'a> {
    pub fn new(input: String, chars: Peekable<Chars<'a>>) -> TemporaryData<'a> {
        Self {
            input,
            chars,
            output: TokenQueue::new(),
            current_token_string: String::new(),
            current_token: Token::new(),
            decimal_point_found: false,
            paren_balance_check: 0,
            scope_balance_check: 0,
        }
    }

    pub fn output(self) -> TokenQueue {
        self.output
    }
}

pub const NUMERIC_DIGITS: [bool; 256] = make_lut("0123456789");
pub const REAL_NUMERIC_DIGITS: [bool; 256] = make_lut(".0123456789");
pub const HEX_NUMERIC_DIGITS: [bool; 256] = make_lut("0123456789ABCDEFabcdef");
pub const BINARY_NUMERIC_DIGITS: [bool; 256] = make_lut("01");
pub const WHITESPACE: [bool; 256] = make_lut(" \t\n\r\x0C");
pub const OPERATOR_CHARACTERS: [bool; 256] = make_lut("!$%^&*+-=#@?|`/\\<>~");
pub const SYMBOL_CHARACTERS: [bool; 256] = make_lut("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_0123456789");

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
