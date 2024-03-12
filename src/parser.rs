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
            panic!("Expected {}, instead got {}", tok.typ.to_string(), expected);
        }

        self.idx += 1;
        tok
    }

    pub fn parseScope(&mut self) -> ast::Scope {
        self.expect(Type::LeftBrace);
        let mut stmts: Vec<Box<Stmt>> = Vec::new();
        while self.tt() != Type::RightBrace {
            stmts.push(Box::new(self.parseStmt()));
        }

        ast::Scope { stmts }
    }

    pub fn parseStmt(&mut self) -> Stmt {
        let tok = self.at();
        match tok.typ {
            Type::LeftBrace => self.parseScopeStmt(),
            Type::If => self.parseIfStmt(),
            _ => panic!("Not a valid statement: {}", tok.typ),
        }
    }

    pub fn parseScopeStmt(&mut self) -> Stmt {
        let mut stmts: Vec<Box<Stmt>> = Vec::new();
        while self.tt() != Type::RightBrace {
            stmts.push(Box::new(self.parseStmt()));
        }

        Stmt::Scope(ast::Scope { stmts })
    }

    pub fn parseIfStmt(&mut self) -> Stmt {
        self.eat();
        let cond = self.parseExpr();
        let body = self.parseScope();

        Stmt::IfStmt(ast::IfStmt {
            cond: Box::new(cond),
            body: Box::new(body),
        })
    }

    pub fn parseExpr(&mut self) -> Expr {
        self.parseComparison()
    }

    pub fn parseComparison(&mut self) -> Expr {
        let left = self.parsePrimary();
        if self.tt().is(COMPARISONS) {
            let comp = self.eat();
            let right = self.parseExpr();

            return Expr::BinaryOp(ast::BinaryOp {
                op: comp.typ,
                lhs: Box::new(left),
                rhs: Box::new(right),
            });
        }

        left
    }

    pub fn parsePrimary(&mut self) -> Expr {
        let tok = self.at();
        match tok.typ {
            Type::Identifier(name) => Expr::Ident(ast::Ident { name }),
            _ => panic!("Invalid expression"),
        }
    }

    pub fn parse(&mut self) {
        println!("{}", "-".to_owned().repeat(100));
        println!("{:#?}", self.parseStmt());
    }
}
