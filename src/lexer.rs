use crate::{
    location::Location,
    tokens::{Token, Type, OPERATORS},
};

// Matcher
pub struct Matcher<'a> {
    lexer: &'a Lexer,
    successes: Vec<Token>,
}

impl<'a> Matcher<'a> {
    pub fn test(
        &mut self,
        patterns: &[&str],
        success: impl Fn(String) -> Type,
    ) -> &mut Matcher<'a> {
        for pattern in patterns {
            if self.lexer.ahead(pattern.len()) == pattern.to_string() {
                let token_type = success(pattern.to_string());
                let token = self.lexer.token(token_type, pattern.to_string());
                self.successes.push(token);
            }
        }
        self
    }

    pub fn succeed(&mut self, user: impl Fn(&Token)) -> &mut Matcher<'a> {
        if self.successes.len() != 0 {
            let mut longest = self.successes.get(0).unwrap();
            for success in &self.successes {
                if success.size > longest.size {
                    longest = &success;
                }
            }

            user(longest);
        }

        self
    }
}

// Lexer
pub struct Lexer {
    loc: Location,
    source: String,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            loc: Location {
                row: 0,
                col: 0,
                idx: 0,
            },
            source: code,
        }
    }

    pub fn ahead(&self, count: usize) -> String {
        let start = self.loc.idx as usize;
        let length = start + count;

        // Take caps out at the length, so no need to implement anything there
        self.source.chars().skip(start).take(length).collect()
    }

    pub fn pattern_match(&self) -> Matcher<'_> {
        Matcher {
            lexer: self,
            successes: vec![],
        }
    }

    pub fn token(&self, token_type: Type, value: String) -> Token {
        Token::new(self.loc, token_type, value)
    }

    pub fn lex(&mut self) {
        let chars: Vec<char> = self.source.chars().collect();
        for ch in chars {
            self.pattern_match()
                .test(OPERATORS, |s| Type::Operator(s))
                .test(&["let"], |_| Type::Let)
                .succeed(|token| println!("{}", token.to_string()));
        }
    }
}
