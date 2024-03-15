use std::rc::Rc;

use crate::{
    ast::{self, Expr, Meta, Node, Stmt},
    error::{Error, ErrorCode},
    program::Program,
    types::{self, DataType},
};

// TODO: Implement
pub struct Checker {
    pub lines: Rc<Vec<String>>,
    pub prog: Program,
}

impl Checker {
    pub fn new(lines: Rc<Vec<String>>, prog: Program) -> Checker {
        Checker { lines, prog }
    }

    fn panic<T>(&self, message: String, node: &Meta<T>, id: ErrorCode) {
        Error::new(Rc::clone(&self.lines), message, node.start, node.end, id).panic();
    }

    // Statements
    fn check_stmt(&mut self, node: &Node<Stmt>) {
        match &node.src {
            Stmt::Decl(x) => self.check_decl(&x),
            _ => self.panic(
                "Invalid statement".to_owned(),
                node,
                ErrorCode::InvalidStatement,
            ),
        }
    }

    fn check_decl(&mut self, decl: &ast::Decl) {
        let val = self.check_expr(&decl.val);
        match &decl.annot {
            Some(annot) => {
                let (annot_typ, val_typ) = (annot.src.name.clone(), val.to_string());
                if annot_typ != val_typ {
                    self.panic(
                        format!(
                            "'{}' is defined to be type {}, but assigned {}",
                            decl.name.src.name, annot_typ, val_typ,
                        ),
                        &decl.val,
                        ErrorCode::TypeMismatch,
                    )
                }
            }
            None => (),
        };
    }

    // Expressions
    fn check_expr(&mut self, node: &Node<Expr>) -> DataType {
        match &node.src {
            Expr::NumLit(_) => types::Int::new(None), // Floats aren't real, they can't hurt you
            Expr::BoolLit(_) => types::Bool::new(),
            Expr::BinaryOp(binop) => self.check_binop(binop),
            Expr::UnaryOp(unop) => self.check_unop(unop),
            _ => {
                self.panic(
                    "Invalid expression".to_owned(),
                    node,
                    ErrorCode::InvalidExpression,
                );
                panic!()
            }
        }
    }

    fn check_binop(&mut self, binop: &ast::BinaryOp) -> DataType {
        let left_typ = self.check_expr(&binop.lhs);
        let right_typ = self.check_expr(&binop.rhs);

        if !left_typ.eq(&right_typ) {
            self.panic(
                format!(
                    "Cannot use the {} operator on {} and {}",
                    binop.op.src.src_strings().get(0).unwrap(),
                    left_typ.to_string(),
                    right_typ.to_string(),
                ),
                &binop.op,
                ErrorCode::TypeMismatch,
            )
        }

        left_typ // Since left_typ == right_typ
    }

    fn check_unop(&mut self, unop: &ast::UnaryOp) -> DataType {
        let typ = self.check_expr(&unop.val);
        typ
    }

    pub fn check(&mut self) {
        for stmt in &self.prog.stmts.clone() {
            self.check_stmt(stmt);
        }
    }
}
