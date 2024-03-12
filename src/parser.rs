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

        ast::Scope { stmts }
    }

    pub fn parse_ident(&mut self) -> ast::Ident {
        match self.eat().typ {
            Type::Identifier(name) => ast::Ident { name },
            _ => panic!("Not an identifier"),
        }
    }

    pub fn parse_numlit(&mut self) -> ast::NumLit {
        match self.eat().typ {
            Type::Number(val) => ast::NumLit { val },
            _ => panic!("Not a number"),
        }
    }

    // Statements
    pub fn parse_stmt(&mut self) -> Stmt {
        let tok = self.at();
        match tok.typ {
            Type::LeftBrace => self.parse_scope_stmt(),
            Type::Let => self.parse_decl(),
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

    pub fn parse_decl(&mut self) -> Stmt {
        self.eat();

        let ident = self.parse_ident();
        self.expect(Type::Assignment);
        let value = self.parse_expr();

        Stmt::Decl(ast::Decl {
            name: Box::new(ident),
            annot: None,
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

    // Expressions
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
        let tok = self.at();

        let expr = match tok.typ {
            Type::Identifier(_) => Expr::Ident(self.parse_ident()),
            Type::Number(_) => Expr::NumLit(self.parse_numlit()),
            _ => panic!("Invalid expression"),
        };

        expr
    }

    pub fn parse(&mut self) {
        println!("{}", "-".to_owned().repeat(100));
        println!("{:#?}", self.parse_stmt());
    }
}
