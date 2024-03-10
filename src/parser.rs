use crate::tokens::Token;

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.clone(),
            idx: 0,
        }
    }

    pub fn parse(&mut self) {
        println!("{:#?}", self.tokens);
    }
}
