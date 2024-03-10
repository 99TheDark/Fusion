use crate::tokens::Token;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    idx: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser {
        Parser { tokens, idx: 0 }
    }

    pub fn parse(&mut self) {}
}
