use std::rc::Rc;

use crate::ast::{self, Expr, Meta, Stmt};
use crate::error::{Error, ErrorCode};
use crate::location::Location;
use crate::tokens::{Token, Type, ORDERED_BINARY_OPERATORS, ORDERED_UNARY_OPERATORS};

pub struct Parser {
    lines: Rc<Vec<String>>,
    tokens: Vec<Token>,
    prog: Vec<Stmt>,
    idx: usize,
}

// Parsing
impl Parser {
    pub fn new(source: Rc<String>, tokens: &Vec<Token>) -> Parser {
        Parser {
            lines: Rc::new(source.split("\n").map(|s| s.to_owned()).collect()),
            tokens: tokens.clone(),
            prog: Vec::new(),
            idx: 0,
        }
    }

    pub fn prev(&self) -> Token {
        self.tokens.get(self.idx - 1).unwrap().clone()
    }

    pub fn at(&self) -> Token {
        self.tokens.get(self.idx).unwrap().clone()
    }

    pub fn tt(&self) -> Type {
        self.at().typ
    }

    pub fn cur_loc(&self) -> Location {
        self.at().start
    }

    pub fn cur_stop(&self) -> Location {
        self.at().end
    }

    pub fn prev_stop(&self) -> Location {
        self.prev().end
    }

    pub fn eat(&mut self) -> Token {
        let tok = self.at();
        self.idx += 1;
        tok
    }

    pub fn expect(&mut self, expected: Type) -> Token {
        let tok = self.at();
        if tok.typ != expected {
            self.panic(
                format!("Expected {}, instead got {}", expected, tok.typ.to_string()),
                ErrorCode::UnexpectedToken,
            );
        }

        self.idx += 1;
        tok
    }

    pub fn panic(&self, message: String, id: ErrorCode) {
        Error::new(
            Rc::clone(&self.lines),
            message,
            self.cur_loc(),
            self.cur_loc().shift(self.at().size), // Later to use Metadata
            id,
        )
        .panic();
    }

    // Raw parses
    pub fn parse_scope(&mut self) -> ast::Scope {
        self.expect(Type::LeftBrace);
        let mut stmts: Vec<Box<Stmt>> = Vec::new();
        while self.tt() != Type::RightBrace {
            if self.tt().is_line_ending() {
                self.eat();
                continue;
            }

            stmts.push(Box::new(self.parse_stmt()));
        }
        self.expect(Type::RightBrace);

        ast::Scope { stmts }
    }

    pub fn parse_ident(&mut self) -> ast::Ident {
        let tok = self.eat();
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

    pub fn parse_num_lit(&mut self) -> ast::NumLit {
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

    pub fn parse_bool_lit(&mut self) -> ast::BoolLit {
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

    // Statements
    pub fn parse_stmt(&mut self) -> Stmt {
        let tok = self.at();
        match tok.typ {
            Type::LeftBrace => self.parse_scope_stmt(),
            Type::Let => self.parse_decl(),
            Type::If => self.parse_if_stmt(),
            Type::While => self.parse_while_loop(),
            Type::Do => self.parse_do_while_loop(),
            _ => {
                self.panic("Invalid statement".to_owned(), ErrorCode::InvalidStatement);
                panic!();
            }
        }
        // TODO: Expect semicolon, newline or eof at the end of each statement
    }

    pub fn parse_scope_stmt(&mut self) -> Stmt {
        let start = self.cur_loc();

        let mut stmts: Vec<Box<Stmt>> = Vec::new();
        while self.tt() != Type::RightBrace {
            stmts.push(Box::new(self.parse_stmt()));
        }

        Stmt::Scope(Meta::new(ast::Scope { stmts }, start, self.prev_stop()))
    }

    pub fn parse_decl(&mut self) -> Stmt {
        let start = self.cur_loc();

        self.eat();
        let ident = self.parse_ident();

        let annotation = if self.tt() == Type::Colon {
            self.eat();
            Some(Box::new(self.parse_ident()))
        } else {
            None
        };

        self.expect(Type::Assignment);
        let value = self.parse_expr();

        Stmt::Decl(Meta::new(
            ast::Decl {
                name: Box::new(ident),
                annot: annotation,
                val: Box::new(value),
            },
            start,
            self.prev_stop(),
        ))
    }

    pub fn parse_if_stmt(&mut self) -> Stmt {
        let start = self.cur_loc();

        self.eat();
        let cond = self.parse_expr();
        let body = self.parse_scope();

        Stmt::IfStmt(Meta::new(
            ast::IfStmt {
                cond: Box::new(cond),
                body: Box::new(body),
            },
            start,
            self.prev_stop(),
        ))
    }

    pub fn parse_while_loop(&mut self) -> Stmt {
        let start = self.cur_loc();

        self.eat();
        let cond = self.parse_expr();
        let body = self.parse_scope();

        Stmt::WhileLoop(Meta::new(
            ast::WhileLoop {
                cond: Box::new(cond),
                body: Box::new(body),
            },
            start,
            self.prev_stop(),
        ))
    }

    pub fn parse_do_while_loop(&mut self) -> Stmt {
        let start = self.cur_loc();

        self.eat();
        let body = self.parse_scope();
        self.expect(Type::While);
        let cond = self.parse_expr();

        Stmt::DoWhileLoop(Meta::new(
            ast::DoWhileLoop {
                body: Box::new(body),
                cond: Box::new(cond),
            },
            start,
            self.prev_stop(),
        ))
    }

    // Expressions
    pub fn parse_expr(&mut self) -> Expr {
        self.parse_binop(None)
    }

    pub fn parse_binop(&mut self, depth: Option<usize>) -> Expr {
        let idx = depth.unwrap_or(0);
        if idx < ORDERED_BINARY_OPERATORS.len() {
            let start = self.cur_loc();

            let mut left = self.parse_binop(Some(idx + 1));
            while self.tt().is(ORDERED_BINARY_OPERATORS[idx]) {
                let op = self.eat().typ;

                left = Expr::BinaryOp(Meta::new(
                    ast::BinaryOp {
                        op,
                        lhs: Box::new(left),
                        rhs: Box::new(self.parse_binop(Some(idx + 1))),
                    },
                    start,
                    self.prev_stop(),
                ));
            }

            left
        } else {
            self.parse_unop()
        }
    }

    pub fn parse_unop(&mut self) -> Expr {
        if self.tt().is(ORDERED_UNARY_OPERATORS) {
            let start = self.cur_loc();

            let op = self.eat().typ;
            let val = Box::new(self.parse_primary());

            Expr::UnaryOp(Meta::new(ast::UnaryOp { op, val }, start, self.prev_stop()))
        } else {
            self.parse_primary()
        }
    }

    pub fn parse_primary(&mut self) -> Expr {
        let tok = self.at();
        let start = self.cur_loc();
        match tok.typ {
            Type::Identifier(_) => {
                Expr::Ident(Meta::new(self.parse_ident(), start, self.cur_stop()))
            }
            Type::Number(_) => {
                Expr::NumLit(Meta::new(self.parse_num_lit(), start, self.cur_stop()))
            }
            Type::Boolean(_) => {
                Expr::BoolLit(Meta::new(self.parse_bool_lit(), start, self.cur_stop()))
            }
            _ => {
                self.panic(
                    format!("Invalid expression {}", tok.typ),
                    ErrorCode::InvalidExpression,
                );
                panic!();
            }
        }
    }

    pub fn parse(&mut self) {
        println!("{}.", ". ".to_owned().repeat(60));

        let mut stmts: Vec<Stmt> = Vec::new();
        while self.tt() != Type::EOF {
            if self.tt().is_line_ending() {
                self.eat();
                continue;
            }

            stmts.push(self.parse_stmt());
        }

        self.prog = stmts;
    }

    pub fn print(&self) {
        println!("{}", format!("{:#?}", self.prog).replace("  ", " "));
    }
}
