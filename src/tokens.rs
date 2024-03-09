use core::fmt;

use crate::location::Location;

#[derive(Debug, Clone)]
pub enum Type {
    Identifier(String),
    Whitespace,
    NewLine,
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
        // TODO: Delete this mess
        let string = match self {
            Type::Number(n) => n.to_string(),
            simple => match simple {
                Type::Identifier(s) => s,
                Type::Whitespace => "whitespace",
                Type::NewLine => "new line",
                Type::Module => "module",
                Type::Assignment => "equals sign",
                Type::LeftParen => "left parenthesis",
                Type::RightParen => "right parenthesis",
                Type::LeftBrace => "left brace",
                Type::RightBrace => "right brace",
                Type::LeftBracket => "left bracket",
                Type::RightBracket => "right bracket",
                Type::Addition => "addition sign",
                Type::Subtraction => "subtraction sign",
                Type::Multiplication => "multiplication sign",
                Type::Division => "division sig",
                Type::Exponentiation => "exponentiation sign",
                Type::Modulo => "modulo sign",
                Type::And => "and",
                Type::Or => "or",
                Type::Nand => "nand",
                Type::Nor => "nor",
                Type::Xand => "xand",
                Type::Xor => "xor",
                Type::Not => "not",
                Type::Equal => "equals sign",
                Type::NotEqual => "not equals sign",
                Type::GreaterThan => "greater than",
                Type::GreaterThanOrEqual => "greater than or equal",
                Type::LessThan => "less than",
                Type::LessThanOrEqual => "less than or equal",
                Type::Let => "let",
                Type::Colon => ":",
                Type::If => "if",
                Type::For => "for",
                Type::While => "while",
                Type::Do => "do",
                Type::Function => "function",
                Type::Class => "class",
                Type::Public => "public",
                Type::Private => "private",
                Type::Inner => "Inner",
                Type::Operator => "operator",
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
        let single;
        let src: &[&str] = match self {
            Type::Whitespace => &[" ", "\t"],
            Type::Boolean(_) => &["true", "false"],
            _ => {
                single = [match self {
                    Type::NewLine => "\n",
                    Type::Module => "mod",
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
        src.to_vec()
    }
}

pub const SYMBOLS: &[Type] = &[
    Type::Whitespace,
    Type::NewLine,
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
