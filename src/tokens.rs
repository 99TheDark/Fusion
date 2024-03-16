use core::fmt;
use std::mem::Discriminant;

use crate::location::Location;

// Types
#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Identifier(String),
    Whitespace,
    NewLine,
    Semicolon,
    Module,
    Number(f32),
    Boolean(bool),
    Assignment,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    Modulo,
    And,
    Or,
    Nand,
    Nor,
    Xand,
    Xor,
    Not,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    LeftShift,
    RightShift,
    ZeroFillRightShift,
    CountLeadingZeros,
    CountTrailingZeros,
    Let,
    Colon,
    If,
    For,
    While,
    Do,
    Break,
    Continue,
    Return,
    Comma,
    Function,
    Class,
    Public,
    Private,
    Inner,
    Operator,
    EOF,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Type {
    pub fn src_strings(&self) -> Vec<String> {
        let single;
        let src: &[&str] = match self {
            Type::Whitespace => &[" ", "\t"],
            _ => {
                single = [match self {
                    Type::NewLine => "\n",
                    Type::Semicolon => ";",
                    Type::Module => "mod",
                    Type::Boolean(true) => "true",
                    Type::Boolean(false) => "false",
                    Type::Assignment => "=",
                    Type::LeftParen => "(",
                    Type::RightParen => ")",
                    Type::LeftBrace => "{",
                    Type::RightBrace => "}",
                    Type::LeftBracket => "[",
                    Type::RightBracket => "]",
                    Type::Addition => "+",
                    Type::Subtraction => "-",
                    Type::Multiplication => "*",
                    Type::Division => "/",
                    Type::Exponentiation => "^",
                    Type::Modulo => "%",
                    Type::And => "&",
                    Type::Or => "|",
                    Type::Nand => "!&",
                    Type::Nor => "!|",
                    Type::Xand => "^&",
                    Type::Xor => "^|",
                    Type::Not => "!",
                    Type::Equal => "==",
                    Type::NotEqual => "!=",
                    Type::GreaterThan => ">",
                    Type::LessThan => "<",
                    Type::GreaterThanOrEqual => ">=",
                    Type::LessThanOrEqual => "<=",
                    Type::LeftShift => "<<",
                    Type::RightShift => ">>",
                    Type::ZeroFillRightShift => ">>>",
                    Type::CountLeadingZeros => "<..",
                    Type::CountTrailingZeros => ">..",
                    Type::Let => "let",
                    Type::Colon => ":",
                    Type::If => "if",
                    Type::For => "for",
                    Type::While => "while",
                    Type::Do => "do",
                    Type::Break => "break",
                    Type::Continue => "continue",
                    Type::Return => "return",
                    Type::Comma => ",",
                    Type::Function => "func",
                    Type::Class => "class",
                    Type::Public => "pub",
                    Type::Private => "pri",
                    Type::Inner => "inn",
                    Type::Operator => "operator",
                    _ => "",
                }];
                &single
            }
        };

        src.to_vec().into_iter().map(|s| s.to_owned()).collect()
    }

    pub fn is(&self, types: &[Type]) -> bool {
        for typ in types {
            if self.eq(typ) {
                return true;
            }
        }
        false
    }

    pub fn disc(&self) -> Discriminant<Type> {
        std::mem::discriminant(self)
    }

    pub fn eq(&self, typ: &Type) -> bool {
        self.disc() == typ.disc()
    }

    pub fn is_line_ending(self) -> bool {
        self == Type::NewLine || self == Type::Semicolon || self == Type::EOF
    }
}

// Constants
pub const SYMBOLS: &[Type] = &[
    Type::Whitespace,
    Type::NewLine,
    Type::Semicolon,
    Type::Assignment,
    Type::LeftParen,
    Type::RightParen,
    Type::LeftBrace,
    Type::RightBrace,
    Type::LeftBracket,
    Type::RightBracket,
    Type::Addition,
    Type::Subtraction,
    Type::Multiplication,
    Type::Division,
    Type::Exponentiation,
    Type::Modulo,
    Type::And,
    Type::Or,
    Type::Nand,
    Type::Nor,
    Type::Xand,
    Type::Xor,
    Type::Not,
    Type::Equal,
    Type::NotEqual,
    Type::GreaterThan,
    Type::LessThan,
    Type::GreaterThanOrEqual,
    Type::LessThanOrEqual,
    Type::LeftShift,
    Type::RightShift,
    Type::ZeroFillRightShift,
    Type::CountLeadingZeros,
    Type::CountTrailingZeros,
    Type::Colon,
    Type::Comma,
];

pub const KEYWORDS: &[Type] = &[
    Type::Module,
    Type::Boolean(true),
    Type::Boolean(false),
    Type::Let,
    Type::If,
    Type::For,
    Type::While,
    Type::Do,
    Type::Break,
    Type::Continue,
    Type::Return,
    Type::Function,
    Type::Class,
    Type::Public,
    Type::Private,
    Type::Inner,
    Type::Operator,
];

pub const ORDERED_BINARY_OPERATORS: &[&[Type]] = &[
    &[
        Type::Xand,
        Type::Xor,
        Type::Nand,
        Type::Nor,
        Type::And,
        Type::Or,
    ],
    &[
        Type::LessThanOrEqual,
        Type::GreaterThanOrEqual,
        Type::LessThan,
        Type::GreaterThan,
        Type::NotEqual,
        Type::Equal,
    ],
    &[Type::RightShift, Type::LeftShift, Type::ZeroFillRightShift],
    &[Type::Modulo],
    &[Type::Subtraction, Type::Addition],
    &[Type::Division, Type::Multiplication],
    &[Type::Exponentiation],
];

pub const ORDERED_UNARY_OPERATORS: &[Type] = &[
    Type::Not,
    Type::CountTrailingZeros,
    Type::CountLeadingZeros,
    Type::Subtraction,
];

// Tokens
#[derive(Debug, Clone)]
pub struct Token {
    pub start: Location,
    pub end: Location,
    pub typ: Type,
    pub size: u32,
}

impl Token {
    pub fn new(start: Location, end: Location, typ: Type) -> Token {
        Token {
            start,
            end,
            typ,
            size: end.idx - start.idx,
        }
    }

    pub fn empty() -> Token {
        Token {
            start: Location::empty(),
            end: Location::empty(),
            typ: Type::EOF,
            size: 0,
        }
    }

    pub fn open(start: Location, typ: Type, size: u32) -> Token {
        Token {
            start,
            end: start,
            typ,
            size,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {} to {}", self.typ, self.start, self.end)
    }
}
