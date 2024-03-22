use std::rc::Rc;

use crate::{
    ast::{self, Expr, Node, Stmt},
    scope::Scope,
    tokens::Type,
};

use super::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_param(&mut self) -> Node<ast::Param> {
        let start = self.cur_loc();

        let name = self.parse_ident();
        self.expect(Type::Colon);
        let annot = self.parse_ident();

        self.node(ast::Param { name, annot }, start)
    }

    pub(crate) fn parse_list<T>(&mut self, parse: fn(&mut Self) -> T) -> Vec<T> {
        let mut vals = vec![parse(self)];
        while self.tt() == Type::Comma {
            self.eat();
            vals.push(parse(self));
        }

        vals
    }

    pub(crate) fn parse_group(&mut self) -> Node<Expr> {
        self.eat();
        let body = self.parse_expr();
        self.expect(Type::RightParen);

        body
    }

    pub(crate) fn parse_block(&mut self) -> Node<ast::Block> {
        let start = self.cur_loc();

        let top = Rc::clone(&self.top);

        let scope = Scope::new(Some(Rc::clone(&top)));
        self.top = Rc::clone(&scope);

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

        self.top = Rc::clone(&top);

        self.node(ast::Block { stmts, scope }, start)
    }
}
