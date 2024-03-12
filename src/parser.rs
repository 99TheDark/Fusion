use std::rc::Rc;

use crate::ast::{self, Expr, Stmt};
use crate::error::Error;
use crate::location::Location;
use crate::tokens::{Token, Type, ORDERED_BINARY_OPERATORS};

pub struct Parser {
    lines: Rc<Vec<String>>,
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    pub fn new(source: Rc<String>, tokens: &Vec<Token>) -> Parser {
        Parser {
            lines: Rc::new(source.split("\n").map(|s| s.to_owned()).collect()),
            tokens: tokens.clone(),
            idx: 0,
        }
    }

    pub fn at(&self) -> Token {
        self.tokens.get(self.idx).unwrap().clone()
    }

    pub fn tt(&self) -> Type {
        self.at().typ
    }

    pub fn cur_loc(&self) -> Location {
        self.at().loc
    }

    pub fn eat(&mut self) -> Token {
        let tok = self.at();
        self.idx += 1;
        tok
    }

    pub fn expect(&mut self, expected: Type) -> Token {
        let tok = self.at();
        if tok.typ != expected {
            Error::new(
                Rc::clone(&self.lines),
                format!("Expected {}, instead got {}", expected, tok.typ.to_string()),
                self.cur_loc().next(),
                self.cur_loc().shift(self.at().size + 1), // Later to use Metadata
                0,
            )
            .panic();
        }

        self.idx += 1;
        tok
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
        match self.eat().typ {
            Type::Identifier(name) => ast::Ident { name },
            _ => panic!("Not an identifier"),
        }
    }

    pub fn parse_num_lit(&mut self) -> ast::NumLit {
        match self.eat().typ {
            Type::Number(val) => ast::NumLit { val },
            _ => panic!("Not a number"),
        }
    }

    pub fn parse_bool_lit(&mut self) -> ast::BoolLit {
        match self.eat().typ {
            Type::Boolean(val) => ast::BoolLit { val },
            _ => panic!("Not a boolean"),
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
            _ => panic!("Not a valid statement: {}", tok.typ),
        }
    }

    pub fn parse_scope_stmt(&mut self) -> Stmt {
        let mut stmts: Vec<Box<Stmt>> = Vec::new();
        while self.tt() != Type::RightBrace {
            stmts.push(Box::new(self.parse_stmt()));
        }

        Stmt::Scope(ast::Scope { stmts })
    }

    pub fn parse_decl(&mut self) -> Stmt {
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

        Stmt::Decl(ast::Decl {
            name: Box::new(ident),
            annot: annotation,
            val: Box::new(value),
        })
    }

    pub fn parse_if_stmt(&mut self) -> Stmt {
        self.eat();
        let cond = self.parse_expr();
        let body = self.parse_scope();

        Stmt::IfStmt(ast::IfStmt {
            cond: Box::new(cond),
            body: Box::new(body),
        })
    }

    pub fn parse_while_loop(&mut self) -> Stmt {
        self.eat();
        let cond = self.parse_expr();
        let body = self.parse_scope();

        Stmt::WhileLoop(ast::WhileLoop {
            cond: Box::new(cond),
            body: Box::new(body),
        })
    }

    pub fn parse_do_while_loop(&mut self) -> Stmt {
        self.eat();
        let body = self.parse_scope();
        self.expect(Type::While);
        let cond = self.parse_expr();

        Stmt::DoWhileLoop(ast::DoWhileLoop {
            body: Box::new(body),
            cond: Box::new(cond),
        })
    }

    // Expressions
    pub fn parse_expr(&mut self) -> Expr {
        self.parse_binop(None)
    }

    pub fn parse_binop(&mut self, depth: Option<usize>) -> Expr {
        let idx = depth.unwrap_or(0);
        if idx < ORDERED_BINARY_OPERATORS.len() {
            let mut left = self.parse_binop(Some(idx + 1));
            while self.tt().is(ORDERED_BINARY_OPERATORS[idx]) {
                let op = self.eat().typ;
                let right = self.parse_binop(Some(idx + 1));

                left = Expr::BinaryOp(ast::BinaryOp {
                    op,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                });
            }

            left
        } else {
            self.parse_primary()
        }
    }

    pub fn parse_primary(&mut self) -> Expr {
        let tok = self.at();

        let expr = match tok.typ {
            Type::Identifier(_) => Expr::Ident(self.parse_ident()),
            Type::Number(_) => Expr::NumLit(self.parse_num_lit()),
            Type::Boolean(_) => Expr::BoolLit(self.parse_bool_lit()),
            _ => panic!("Invalid expression"),
        };

        expr
    }

    pub fn parse(&mut self) {
        println!("{}", "-".to_owned().repeat(100));

        let mut stmts: Vec<Stmt> = Vec::new();
        while self.tt() != Type::EOF {
            if self.tt().is_line_ending() {
                self.eat();
                continue;
            }

            stmts.push(self.parse_stmt());
        }

        println!("{:#?}", stmts);
    }
}
