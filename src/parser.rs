use std::rc::Rc;

use crate::ast::{self, Expr, Meta, Stmt};
use crate::error::{Error, ErrorCode};
use crate::location::Location;
use crate::program::Program;
use crate::tokens::{Token, Type, KEYWORDS, ORDERED_BINARY_OPERATORS, ORDERED_UNARY_OPERATORS};

pub struct Parser {
    pub lines: Rc<Vec<String>>,
    pub tokens: Vec<Token>,
    pub prog: Program,
    idx: usize,
}

// Parsing
impl Parser {
    pub fn new(lines: Rc<Vec<String>>, tokens: &Vec<Token>) -> Parser {
        Parser {
            lines,
            tokens: tokens.clone(),
            prog: Program::new(),
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

    fn cur_stop(&self) -> Location {
        self.at().end
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
            self.cur_loc().shift(self.at().size), // Later to use Metadata
            id,
        )
        .panic();
    }

    // Misc
    fn parse_param(&mut self) -> ast::Param {
        let name = self.parse_ident();
        self.expect(Type::Colon);
        let annot = self.parse_ident();

        ast::Param {
            name: Box::new(name),
            annot: Box::new(annot),
        }
    }

    // Raw parses
    fn parse_scope(&mut self) -> ast::Scope {
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

    fn parse_list<T>(&mut self, parse: fn(&mut Self) -> T) -> Vec<Box<T>> {
        let mut vals = vec![Box::new(parse(self))];
        while self.tt() == Type::Comma {
            self.eat();
            vals.push(Box::new(parse(self)));
        }

        vals
    }

    fn parse_ident(&mut self) -> ast::Ident {
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

    fn parse_num_lit(&mut self) -> ast::NumLit {
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

    fn parse_bool_lit(&mut self) -> ast::BoolLit {
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
    fn parse_stmt(&mut self) -> Stmt {
        let tok = self.at();
        match tok.typ {
            Type::LeftBrace => self.parse_scope_stmt(),
            Type::Let => self.parse_decl(),
            Type::If => self.parse_if_stmt(),
            Type::While => self.parse_while_loop(),
            Type::Do => self.parse_do_while_loop(),
            Type::Return => self.parse_return(),
            Type::Function => self.parse_func(),
            _ => {
                self.panic("Invalid statement".to_owned(), ErrorCode::InvalidStatement);
                panic!();
            }
        }
        // TODO: Expect semicolon, newline or eof at the end of each statement
    }

    fn parse_scope_stmt(&mut self) -> Stmt {
        let start = self.cur_loc();
        let scope = self.parse_scope();

        Stmt::Scope(Meta::new(scope, start, self.prev_stop()))
    }

    fn parse_decl(&mut self) -> Stmt {
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

    fn parse_if_stmt(&mut self) -> Stmt {
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

    fn parse_while_loop(&mut self) -> Stmt {
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

    fn parse_do_while_loop(&mut self) -> Stmt {
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

    fn parse_return(&mut self) -> Stmt {
        let start = self.cur_loc();

        self.eat();
        let val = if self.tt().is_line_ending() {
            None
        } else {
            Some(Box::new(self.parse_expr()))
        };

        Stmt::Return(Meta::new(ast::Return { val }, start, self.prev_stop()))
    }

    fn parse_func(&mut self) -> Stmt {
        let start = self.cur_loc();

        self.eat();
        let name = self.parse_ident();

        self.expect(Type::LeftParen);
        let args = self.parse_list(Parser::parse_param);
        self.expect(Type::RightParen);

        let ret = if self.tt() == Type::Colon {
            self.eat();
            Some(Box::new(self.parse_ident()))
        } else {
            None
        };

        Stmt::Func(Meta::new(
            ast::Func {
                name: Box::new(name),
                args,
                ret,
                body: Box::new(ast::Scope { stmts: Vec::new() }),
            },
            start,
            self.prev_stop(),
        ))
    }

    // Expressions
    fn parse_expr(&mut self) -> Expr {
        self.parse_binop(None)
    }

    fn parse_binop(&mut self, depth: Option<usize>) -> Expr {
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

    fn parse_unop(&mut self) -> Expr {
        if self.tt().is(ORDERED_UNARY_OPERATORS) {
            let start = self.cur_loc();

            let op = self.eat().typ;
            let val = Box::new(self.parse_primary());

            Expr::UnaryOp(Meta::new(ast::UnaryOp { op, val }, start, self.prev_stop()))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Expr {
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

        let mut prog = Program::new();
        while self.tt() != Type::EOF {
            if self.tt().is_line_ending() {
                self.eat();
                continue;
            }

            prog.stmts.push(Box::new(self.parse_stmt()));
        }

        self.prog = prog;
    }
}
