use crate::location;

pub enum TokenType {
    Identifier,
    WhiteSpace,
    NewLine,
    Number,
    Assignment,
    LeftParen,
    RightParen,
    Addition,
    Subtraction,
    Multiplication,
    Dividion,
    Exponentiation,
    Modulus,
    Let,
    Colon,
    EOF,
}

pub struct Token {
    pub loc: location::Location,
    pub typ: TokenType,
}
