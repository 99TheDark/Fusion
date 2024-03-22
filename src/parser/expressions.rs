use crate::{
    ast::{self, Expr, Meta, Node},
    error::ErrorCode,
    tokens::{Type, ORDERED_BINARY_OPERATORS, ORDERED_UNARY_OPERATORS},
};

use super::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_expr(&mut self) -> Node<Expr> {
        self.parse_binop(None)
    }

    pub(crate) fn parse_binop(&mut self, depth: Option<usize>) -> Node<Expr> {
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

    pub(crate) fn parse_unop(&mut self) -> Node<Expr> {
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
    pub(crate) fn parse_primary(&mut self) -> Node<Expr> {
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

    pub(crate) fn parse_ident(&mut self) -> Node<ast::Ident> {
        let start = self.cur_loc();
        let raw = self.parse_raw_ident();

        self.node(raw, start)
    }
}
