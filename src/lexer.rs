use crate::{
    location::Location,
    tokens::{Token, Type, OPERATORS},
};

// Lexer
pub struct Lexer {
    loc: Location,
    tokens: Vec<Token>,
    source: String,
    successes: Vec<Token>,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            loc: Location {
                row: 0,
                col: 0,
                idx: 0,
            },
            tokens: Vec::new(),
            source: code,
            successes: Vec::new(),
        }
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

    pub fn token(&self, token_type: Type, value: String) -> Token {
        Token::new(self.loc, token_type, value)
    }

    pub fn test(&mut self, patterns: &[&str], success: impl Fn(String) -> Type) -> &mut Lexer {
        for pattern in patterns {
            if self.ahead(pattern.len()) == pattern.to_string() {
                let token_type = success(pattern.to_string());
                let token = self.token(token_type, pattern.to_string());
                self.successes.push(token);
            }
        }
        self
    }

    pub fn succeed(&mut self, user: impl Fn(&mut Lexer, Token)) -> &mut Lexer {
        if self.successes.len() != 0 {
            let mut longest = self.successes.get(0).unwrap();
            for success in &self.successes {
                if success.size > longest.size {
                    longest = &success;
                }
            }

            user(self, longest.clone());
        }

        self
    }

    pub fn fail(&mut self, user: impl Fn(&mut Lexer)) {
        if self.successes.len() == 0 {
            self.loc.idx += 1;
            user(self);
        }

        self.successes.clear();
    }

    pub fn lex(&mut self) -> Vec<Token> {
        while self.loc.idx < self.source.len() as u32 {
            self.test(OPERATORS, |s| Type::Operator(s))
                .test(&["let"], |_| Type::Let)
                .succeed(|lexer: &mut Lexer, token: Token| {
                    //    ^^^^^^^^^^^^^^^^^ There has to be a way to get around this...
                    println!("{}", token.to_string());
                    lexer.push(token);
                })
                .fail(|lexer: &mut Lexer| {
                    // TODO: Make identifiers longer than 1 character lol
                    lexer.push(Token::new(
                        lexer.loc,
                        Type::Identifier(lexer.ahead(1)),
                        lexer.ahead(1),
                    ));
                });
        }

        self.tokens.clone()
    }
}
