use crate::{
    location::Location,
    tokens::{Token, Type, SYMBOLS},
};

// Lexer
pub struct Lexer {
    loc: Location,
    tokens: Vec<Token>,
    source: String,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            loc: Location::empty(),
            tokens: Vec::new(),
            source: code,
        }
    }

    fn at(&self) -> String {
        self.source
            .chars()
            .nth(self.loc.idx as usize)
            .unwrap()
            .to_string()
    }

    fn ahead(&self, count: usize) -> String {
        // Take caps out at the length, so no need to implement anything there
        self.source
            .chars()
            .skip(self.loc.idx as usize)
            .take(count)
            .collect()
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token.clone());

        self.loc.idx += token.size;
        match token.typ {
            Type::NewLine => {
                self.loc.row += 1;
                self.loc.col = 0;
            }
            _ => {
                self.loc.col += token.size;
            }
        }
    }

    fn token(&self, token_type: Type, value: String) -> Token {
        Token::new(self.loc, token_type, value)
    }

    fn symbol(&mut self) -> (bool, Token) {
        let mut success = String::new();
        let mut successful_pattern = Type::EOF;
        for pattern in SYMBOLS {
            let sources = pattern.src_strings();
            for src in sources {
                if self.ahead(src.len()) == src && src.len() > success.len() {
                    success = src.to_string();
                    successful_pattern = pattern.clone();
                }
            }
        }

        if success.len() == 0 {
            (false, Token::empty())
        } else {
            let token = self.token(successful_pattern, success);
            (true, token)
        }
    }

    fn push_identifier(&mut self, capture: &String) -> bool {
        let size = capture.len();
        if size != 0 {
            let loc = Location::new(self.loc.idx - size as u32, self.loc.row, self.loc.col);

            let value = capture.clone();
            let typ = match capture.clone().parse::<f32>() {
                Ok(num) => Type::Number(num), // TODO: Ignore inf, -inf, nan, etc
                Err(_) => Type::Identifier(capture.clone()),
            };

            let token = Token::new(loc, typ, value);
            self.tokens.push(token);

            return true;
        }
        false
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut capture = String::new();
        while self.loc.idx < self.source.len() as u32 {
            let (is_symbol, symbol) = self.symbol();
            if !is_symbol {
                capture += &self.at();
                self.loc.idx += 1;
            } else {
                if self.push_identifier(&capture) {
                    capture.clear();
                };

                self.push(symbol);
            }
        }
        self.push_identifier(&capture);

        self.tokens.clone()
    }
}
