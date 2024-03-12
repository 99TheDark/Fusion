use crate::ast::{self, Expr, Stmt};
use crate::tokens::{Token, Type, COMPARISONS};

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

    pub fn at(&self) -> Token {
        self.tokens.get(self.idx).unwrap().clone()
    }

    pub fn tt(&self) -> Type {
        self.at().typ
    }

    pub fn eat(&mut self) -> Token {
        let tok = self.at();
        self.idx += 1;
        tok
    }

    pub fn expect(&mut self, expected: Type) -> Token {
        let tok = self.at();
        if tok.typ != expected {
            panic!("Expected {}, instead got {}", expected, tok.typ.to_string());
        }

        self.idx += 1;
        tok
    }

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

        ast::Scope { stmts }
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        let tok = self.at();
        match tok.typ {
            Type::LeftBrace => self.parse_scope_stmt(),
            Type::If => self.parse_if_stmt(),
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

    pub fn parse_if_stmt(&mut self) -> Stmt {
        self.eat();
        let cond = self.parse_expr();
        let body = self.parse_scope();

        Stmt::IfStmt(ast::IfStmt {
            cond: Box::new(cond),
            body: Box::new(body),
        })
    }

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_comparison()
    }

    pub fn parse_comparison(&mut self) -> Expr {
        let left = self.parse_primary();
        if self.tt().is(COMPARISONS) {
            let comp = self.eat();
            let right = self.parse_expr();

            return Expr::BinaryOp(ast::BinaryOp {
                op: comp.typ,
                lhs: Box::new(left),
                rhs: Box::new(right),
            });
        }

        left
    }

    pub fn parse_primary(&mut self) -> Expr {
        let tok = self.eat();
        match tok.typ {
            Type::Identifier(name) => Expr::Ident(ast::Ident { name }),
            _ => panic!("Invalid expression"),
        }
    }

    pub fn parse(&mut self) {
        println!("{}", "-".to_owned().repeat(100));
        println!("{:#?}", self.parse_stmt());
    }
}
