use core::fmt;

use crate::location::Location;

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
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Let,
    Colon,
    If,
    For,
    While,
    Do,
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
                    Type::GreaterThanOrEqual => ">=",
                    Type::LessThan => "<",
                    Type::LessThanOrEqual => "<=",
                    Type::Let => "let",
                    Type::Colon => ":",
                    Type::If => "if",
                    Type::For => "for",
                    Type::While => "while",
                    Type::Do => "do",
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
            if self == typ {
                return true;
            }
        }
        false
    }
}

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
    Type::GreaterThanOrEqual,
    Type::LessThan,
    Type::LessThanOrEqual,
    Type::Colon,
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
    Type::Function,
    Type::Class,
    Type::Public,
    Type::Private,
    Type::Inner,
    Type::Operator,
];

pub const COMPARISONS: &[Type] = &[
    Type::Equal,
    Type::NotEqual,
    Type::GreaterThan,
    Type::GreaterThanOrEqual,
    Type::LessThan,
    Type::LessThanOrEqual,
];

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
