use core::fmt;

use crate::location::Location;

#[derive(Debug)]
pub enum Type {
    Identifier(String),
    WhiteSpace,
    NewLine,
    Number(f32),
    Assignment,
    LeftParen,
    RightParen,
    Operator(String),
    Let,
    Colon,
    EOF,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const OPERATORS: &[&str] = &["+", "-", "*", "/", "^", "%"];
pub const KEYWORDS: &[Type] = &[Type::Let];

pub struct Token {
    pub loc: Location,
    pub typ: Type,
    pub size: u32,
}

impl Token {
    pub fn new(location: Location, token_type: Type, value: String) -> Token {
        Token {
            loc: location,
            typ: token_type,
            size: value.len() as u32,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.typ, self.loc)
    }
}
