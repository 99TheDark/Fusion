use crate::tokens::Token;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&mut self) {}
}
