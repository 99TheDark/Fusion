use std::rc::Rc;

use crate::{
    ast::{self, Expr, Meta, Node, Stmt},
    error::{Error, ErrorCode},
    location::Location,
    tokens::{Token, Type, KEYWORDS, ORDERED_BINARY_OPERATORS, ORDERED_UNARY_OPERATORS},
};

pub struct Parser {
    pub lines: Rc<Vec<String>>,
    pub tokens: Vec<Token>,
    pub prog: ast::Block,
    idx: usize,
}

// Parsing
impl Parser {
    pub fn new(lines: Rc<Vec<String>>, tokens: &Vec<Token>) -> Parser {
        Parser {
            lines,
            tokens: tokens.clone(),
            prog: ast::Block { stmts: Vec::new() },
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
        Box::new(Meta::new(node, start, self.prev_stop()))
    }

    // Misc
    fn parse_param(&mut self) -> Node<ast::Param> {
        let start = self.cur_loc();

        let name = self.parse_ident();
        self.expect(Type::Colon);
        let annot = self.parse_ident();

        self.node(ast::Param { name, annot }, start)
    }

    fn parse_list<T>(&mut self, parse: fn(&mut Self) -> T) -> Vec<T> {
        let mut vals = vec![parse(self)];
        while self.tt() == Type::Comma {
            self.eat();
            vals.push(parse(self));
        }

        vals
    }

    fn parse_group(&mut self) -> Node<Expr> {
        self.eat();
        let body = self.parse_expr();
        self.expect(Type::RightParen);

        body
    }

    fn parse_scope(&mut self) -> Node<ast::Block> {
        let start = self.cur_loc();

        self.expect(Type::LeftBrace);
        let mut stmts: Vec<Node<Stmt>> = Vec::new();
        while self.tt() != Type::RightBrace {
            if self.tt().is_line_ending() {
                self.eat();
                continue;
            }

            stmts.push(self.parse_stmt());
        }
        self.expect(Type::RightBrace);

        self.node(ast::Block { stmts }, start)
    }

    // Raw parses
    fn parse_raw_ident(&mut self) -> ast::Ident {
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

    fn parse_raw_num_lit(&mut self) -> ast::NumLit {
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

    fn parse_raw_bool_lit(&mut self) -> ast::BoolLit {
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
    fn parse_stmt(&mut self) -> Node<Stmt> {
        let tok = self.at();
        match tok.typ {
            Type::LeftBrace => self.parse_scope_stmt(),
            Type::Let => self.parse_decl(),
            Type::If => self.parse_if_stmt(),
            Type::While => self.parse_while_loop(),
            Type::Do => self.parse_do_while_loop(),
            Type::Continue => self.parse_continue(),
            Type::Return => self.parse_return(),
            Type::Function => self.parse_func(),
            _ => {
                self.panic("Invalid statement".to_owned(), ErrorCode::InvalidStatement);
                panic!();
            }
        }
        // TODO: Expect Semicolon, NewLine or EOF at the end of each statement
        // Also, maybe give warning for unneeded newlines after this is implemented
    }

    fn parse_scope_stmt(&mut self) -> Node<Stmt> {
        let block = self.parse_scope();
        self.node(Stmt::Block(block.src), block.start)
    }

    fn parse_decl(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();

        self.eat();
        let ident = self.parse_ident();

        let annotation = if self.tt() == Type::Colon {
            self.eat();
            Some(self.parse_ident())
        } else {
            None
        };

        self.expect(Type::Assignment);
        let value = self.parse_expr();

        self.node(
            Stmt::Decl(ast::Decl {
                name: ident,
                annot: annotation,
                val: value,
            }),
            start,
        )
    }

    fn parse_if_stmt(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();

        self.eat();
        let cond = self.parse_expr();
        let body = self.parse_scope();

        self.node(Stmt::IfStmt(ast::IfStmt { cond, body }), start)
    }

    fn parse_while_loop(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();

        self.eat();
        let cond = self.parse_expr();
        let body = self.parse_scope();

        self.node(Stmt::WhileLoop(ast::WhileLoop { cond, body }), start)
    }

    fn parse_do_while_loop(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();

        self.eat();
        let body = self.parse_scope();
        self.expect(Type::While);
        let cond = self.parse_expr();

        self.node(Stmt::DoWhileLoop(ast::DoWhileLoop { body, cond }), start)
    }

    fn parse_continue(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();
        self.expect(Type::Continue);

        self.node(Stmt::Continue, start)
    }

    fn parse_return(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();

        self.eat();
        let val = if self.tt().is_line_ending() {
            None
        } else {
            Some(self.parse_expr())
        };

        self.node(Stmt::Return(ast::Return { val }), start)
    }

    fn parse_func(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();

        self.eat();
        let name = self.parse_ident();

        self.expect(Type::LeftParen);
        let args = self.parse_list(Parser::parse_param);
        self.expect(Type::RightParen);

        let ret = if self.tt() == Type::Colon {
            self.eat();
            Some(self.parse_ident())
        } else {
            None
        };

        let body = self.parse_scope();

        self.node(
            Stmt::Func(ast::Func {
                name,
                args,
                ret,
                body,
            }),
            start,
        )
    }

    // Expressions
    fn parse_expr(&mut self) -> Node<Expr> {
        self.parse_binop(None)
    }

    fn parse_binop(&mut self, depth: Option<usize>) -> Node<Expr> {
        let idx = depth.unwrap_or(0);
        if idx < ORDERED_BINARY_OPERATORS.len() {
            let start = self.cur_loc();

            let mut left = self.parse_binop(Some(idx + 1));
            while self.tt().is(ORDERED_BINARY_OPERATORS[idx]) {
                let op = self.eat();
                let binop = self.parse_binop(Some(idx + 1));

                left = self.node(
                    Expr::BinaryOp(ast::BinaryOp {
                        op: Meta::new(op.typ, op.start, op.end),
                        lhs: left,
                        rhs: binop,
                    }),
                    start,
                );
            }

            left
        } else {
            self.parse_unop()
        }
    }

    fn parse_unop(&mut self) -> Node<Expr> {
        if self.tt().is(ORDERED_UNARY_OPERATORS) {
            let start = self.cur_loc();

            let op = self.eat();
            let val = self.parse_primary();

            self.node(
                Expr::UnaryOp(ast::UnaryOp {
                    op: Meta::new(op.typ, op.start, op.end),
                    val,
                }),
                start,
            )
        } else {
            self.parse_primary()
        }
    }

    // Primaries
    fn parse_primary(&mut self) -> Node<Expr> {
        let tok = self.at();
        let start = self.cur_loc();
        match tok.typ {
            // I find it really irritating you can't call a mut method who's parameter is a mut method
            Type::Identifier(_) => {
                let expr = Expr::Ident(self.parse_raw_ident());
                self.node(expr, start)
            }
            Type::Number(_) => {
                let expr = Expr::NumLit(self.parse_raw_num_lit());
                self.node(expr, start)
            }
            Type::Boolean(_) => {
                let expr = Expr::BoolLit(self.parse_raw_bool_lit());
                self.node(expr, start)
            }
            Type::LeftParen => self.parse_group(),
            _ => {
                self.panic(
                    format!("Invalid expression {}", tok.typ),
                    ErrorCode::InvalidExpression,
                );
                panic!();
            }
        }
    }

    fn parse_ident(&mut self) -> Node<ast::Ident> {
        let start = self.cur_loc();
        let raw = self.parse_raw_ident();

        self.node(raw, start)
    }

    pub fn parse(&mut self) {
        println!("{}.", ". ".to_owned().repeat(60));

        while self.tt() != Type::EOF {
            if self.tt().is_line_ending() {
                self.eat();
                continue;
            }

            let stmt = self.parse_stmt();
            self.prog.stmts.push(stmt);
        }
    }
}
