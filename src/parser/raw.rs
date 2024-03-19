use crate::{
    ast,
    error::ErrorCode,
    tokens::{Type, KEYWORDS},
};

use super::Parser;

impl Parser {
    pub(crate) fn parse_raw_ident(&mut self) -> ast::Ident {
        let cur_tok = self.at();
        if cur_tok.typ.is(KEYWORDS) {
            self.panic(
                format!(
                    "Cannot use {} for a name, because it is a reserved keyword",
                    cur_tok.typ.src_strings().get(0).unwrap(),
                ),
                ErrorCode::ReservedNameUsed,
            );
        }

        let tok = self.expect(Type::Identifier("".to_string()));
        match tok.typ {
            Type::Identifier(name) => ast::Ident { name },
            _ => {
                self.panic(
                    format!("{} is not an identifier", tok),
                    ErrorCode::IncorrectParsingType,
                );
                panic!(); // So no need to return ast::Ident { name: "" }, takes a lot of lines of code
            }
        }
    }

    pub(crate) fn parse_raw_num_lit(&mut self) -> ast::NumLit {
        let tok = self.eat();
        match tok.typ {
            Type::Number(val) => ast::NumLit { val },
            _ => {
                self.panic(
                    format!("{} is not a number", tok),
                    ErrorCode::IncorrectParsingType,
                );
                panic!();
            }
        }
    }

    pub(crate) fn parse_raw_bool_lit(&mut self) -> ast::BoolLit {
        let tok = self.eat();
        match tok.typ {
            Type::Boolean(val) => ast::BoolLit { val },
            _ => {
                self.panic(
                    format!("{} is not a boolean", tok),
                    ErrorCode::IncorrectParsingType,
                );
                panic!();
            }
        }
    }
}
