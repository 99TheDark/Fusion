use std::{cell::RefCell, rc::Rc};

pub(crate) mod expressions;
pub(crate) mod misc;
pub(crate) mod raw;
pub(crate) mod statements;

use crate::program::Program;
pub use crate::{
    ast::{self, Node},
    error::{Error, ErrorCode},
    location::Location,
    scope::Scope,
    tokens::{Token, Type},
};

pub struct Parser {
    pub lines: Rc<Vec<String>>,
    pub tokens: Vec<Token>,
    pub prog: Program,
    top: Rc<RefCell<Scope>>,
    idx: usize,
}

// Parsing
impl Parser {
    pub fn new(lines: Rc<Vec<String>>, tokens: &Vec<Token>) -> Parser {
        let prog = Program::new(Scope::new(None));
        let top = Rc::clone(&prog.block.scope);

        Parser {
            lines,
            tokens: tokens.clone(),
            prog,
            top,
            idx: 0,
        }
    }

    fn prev(&self) -> Token {
        self.tokens.get(self.idx - 1).unwrap().clone()
    }

    fn at(&self) -> Token {
        self.tokens.get(self.idx).unwrap().clone()
    }

    fn tt(&self) -> Type {
        self.at().typ
    }

    fn cur_loc(&self) -> Location {
        self.at().start
    }

    fn prev_stop(&self) -> Location {
        self.prev().end
    }

    fn eat(&mut self) -> Token {
        let tok = self.at();
        self.idx += 1;
        tok
    }

    fn expect(&mut self, expected: Type) -> Token {
        let tok = self.at();
        if !tok.typ.eq(&expected) {
            self.panic(
                format!("Expected {}, instead got {}", expected, tok.typ.to_string()),
                ErrorCode::UnexpectedToken,
            );
        }

        self.idx += 1;
        tok
    }

    fn panic(&self, message: String, id: ErrorCode) {
        Error::new(
            Rc::clone(&self.lines),
            message,
            self.cur_loc(),
            self.cur_loc().shift(self.at().size), // TODO: Use Metadata
            id,
        )
        .panic();
    }

    fn node<T>(&self, node: T, start: Location) -> Node<T> {
        Node::new(node, start, self.prev_stop())
    }

    pub fn parse(&mut self) {
        println!("{}.", ". ".to_owned().repeat(60));

        while self.tt() != Type::EOF {
            if self.tt().is_line_ending() {
                self.eat();
                continue;
            }

            let stmt = self.parse_stmt();
            self.prog.block.stmts.push(stmt);
        }
    }
}
