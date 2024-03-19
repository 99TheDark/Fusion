use crate::{
    ast::{self, Meta, Node, Stmt},
    error::ErrorCode,
    tokens::{Type, ORDERED_BINARY_OPERATORS},
};

use super::Parser;

impl Parser {
    pub(crate) fn parse_stmt(&mut self) -> Node<Stmt> {
        let tok = self.at();
        match tok.typ {
            Type::Identifier(_) => self.parse_assign(),
            Type::LeftBrace => self.parse_block_stmt(),
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

    pub(crate) fn parse_block_stmt(&mut self) -> Node<Stmt> {
        let block = self.parse_block();
        self.node(Stmt::Block(block.src.clone()), block.start)
    }

    pub(crate) fn parse_decl(&mut self) -> Node<Stmt> {
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

        self.top.borrow_mut().declare(ident.src.name.clone());

        self.node(
            Stmt::Decl(ast::Decl {
                name: ident,
                annot: annotation,
                val: value,
            }),
            start,
        )
    }

    pub(crate) fn parse_assign(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();

        let name = self.parse_ident();

        let mut has_op = false;
        for ops in ORDERED_BINARY_OPERATORS {
            if self.tt().is(ops) {
                has_op = true;
                break;
            }
        }

        let op = if has_op {
            let tok = self.eat();
            Some(Meta::new(tok.typ, tok.start, tok.end))
        } else {
            None
        };

        self.expect(Type::Assignment);
        let val = self.parse_expr();

        self.node(Stmt::Assign(ast::Assign { name, op, val }), start)
    }

    pub(crate) fn parse_if_stmt(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();

        self.eat();
        let cond = self.parse_expr();
        let body = self.parse_block();

        self.node(Stmt::IfStmt(ast::IfStmt { cond, body }), start)
    }

    pub(crate) fn parse_while_loop(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();

        self.eat();
        let cond = self.parse_expr();
        let body = self.parse_block();

        self.node(Stmt::WhileLoop(ast::WhileLoop { cond, body }), start)
    }

    pub(crate) fn parse_do_while_loop(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();

        self.eat();
        let body = self.parse_block();
        self.expect(Type::While);
        let cond = self.parse_expr();

        self.node(Stmt::DoWhileLoop(ast::DoWhileLoop { body, cond }), start)
    }

    pub(crate) fn parse_continue(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();
        self.expect(Type::Continue);

        self.node(Stmt::Continue, start)
    }

    pub(crate) fn parse_return(&mut self) -> Node<Stmt> {
        let start = self.cur_loc();

        self.eat();
        let val = if self.tt().is_line_ending() {
            None
        } else {
            Some(self.parse_expr())
        };

        self.node(Stmt::Return(ast::Return { val }), start)
    }

    pub(crate) fn parse_func(&mut self) -> Node<Stmt> {
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

        let body = self.parse_block();

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
}
