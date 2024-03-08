use core::fmt;

use crate::location::Location;

#[derive(Debug, Clone)]
pub enum Type {
    Identifier(String),
    Whitespace,
    NewLine,
    Number(f32),
    Assignment,
    LeftParen,
    RightParen,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    Modulo,
    Let,
    Colon,
    EOF,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Type::Number(n) => n.to_string(),
            simple => match simple {
                Type::Identifier(s) => s,
                Type::Whitespace => "whitespace",
                Type::NewLine => "new line",
                Type::Assignment => "=",
                Type::LeftParen => "(",
                Type::RightParen => ")",
                Type::Addition => "+",
                Type::Subtraction => "-",
                Type::Multiplication => "*",
                Type::Division => "/",
                Type::Exponentiation => "^",
                Type::Modulo => "%",
                Type::Let => "let",
                Type::Colon => ":",
                Type::EOF => "end of file",
                _ => "", // This should never be reached
            }
            .to_owned(),
        };

        write!(f, "{}", string)
    }
}

impl Type {
    pub fn src_strings(&self) -> Vec<&str> {
        let src: &[&str] = match self {
            Type::Whitespace => &[" ", "\t"],
            Type::NewLine => &["\n"],
            Type::Assignment => &["="],
            Type::LeftParen => &["("],
            Type::RightParen => &[")"],
            Type::Addition => &["+"],
            Type::Subtraction => &["-"],
            Type::Multiplication => &["*"],
            Type::Division => &["/"],
            Type::Exponentiation => &["^"],
            Type::Modulo => &["%"],
            Type::Let => &["let"],
            Type::Colon => &[":"],
            _ => &[""],
        };
        src.to_vec()
    }
}

pub const SYMBOLS: &[Type] = &[
    Type::Whitespace,
    Type::NewLine,
    Type::Assignment,
    Type::LeftParen,
    Type::RightParen,
    Type::Addition,
    Type::Subtraction,
    Type::Multiplication,
    Type::Division,
    Type::Modulo,
    Type::Colon,
];
pub const KEYWORDS: &[Type] = &[Type::Let];

#[derive(Debug, Clone)]
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

    pub fn empty() -> Token {
        Token {
            loc: Location::empty(),
            typ: Type::EOF,
            size: 0,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.typ, self.loc)
    }
}
