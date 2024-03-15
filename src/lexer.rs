use std::rc::Rc;

use crate::{
    location::Location,
    tokens::{Token, Type, KEYWORDS, SYMBOLS},
};

// Lexer
pub struct Lexer {
    source: Rc<String>,
    loc: Location,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: Rc<String>) -> Lexer {
        Lexer {
            source: code,
            loc: Location::empty(),
            tokens: Vec::new(),
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
        self.loc.idx += token.size;
        if token.typ == Type::NewLine {
            self.loc.row += 1;
            self.loc.col = 0;
        } else {
            self.loc.col += token.size;
        }

        self.tokens
            .push(Token::new(token.start, self.loc.clone(), token.typ));
    }

    fn symbol(&mut self) -> (bool, Token) {
        let mut max_len = 0;
        let mut successful_pattern = Type::EOF;
        for pattern in SYMBOLS {
            let sources = pattern.src_strings();
            for src in sources {
                if self.ahead(src.len()) == src && src.len() > max_len {
                    max_len = src.len();
                    successful_pattern = pattern.clone();
                }
            }
        }

        if max_len == 0 {
            (false, Token::empty())
        } else {
            let token = Token::open(self.loc.clone(), successful_pattern, max_len as u32);
            (true, token)
        }
    }

    fn push_identifier(&mut self, capture: &String, cap_start: Location) -> bool {
        let size = capture.len();
        if size != 0 {
            let value = capture.clone();
            let typ = match capture.clone().parse::<f32>() {
                Ok(num) => Type::Number(num), // TODO: Ignore inf, -inf, nan, etc
                Err(_) => {
                    let mut ident_typ = Type::Identifier(capture.clone());
                    'main: for keyword in KEYWORDS {
                        for src in keyword.src_strings() {
                            if value == src {
                                ident_typ = keyword.clone();
                                break 'main;
                            }
                        }
                    }

                    ident_typ
                }
            };

            let token = Token::new(cap_start, self.loc, typ);
            self.tokens.push(token);

            return true;
        }
        false
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut capture = String::new();
        let mut cap_start = Location::empty();
        while self.loc.idx < self.source.len() as u32 {
            let (is_symbol, symbol) = self.symbol();
            if !is_symbol {
                if capture.len() == 0 {
                    cap_start = self.loc.clone();
                }

                capture += &self.at();
                self.loc.next();
            } else {
                if self.push_identifier(&capture, cap_start) {
                    capture.clear();
                };

                self.push(symbol);
            }
        }
        self.push_identifier(&capture, cap_start);

        // Add EOF token
        self.push(Token::new(self.loc, self.loc, Type::EOF));

        self.tokens.clone()
    }

    pub fn filter(&self) -> Vec<Token> {
        self.tokens
            .clone()
            .into_iter()
            .filter(|tok| tok.typ != Type::Whitespace)
            .collect()
    }
}

pub fn source_lines(source: Rc<String>) -> Rc<Vec<String>> {
    Rc::new(source.split("\n").map(|s| s.to_owned()).collect())
}
