use std::rc::Rc;

use crate::{
    ast::{self, Expr, Node, Stmt},
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

    fn panic<T>(&self, message: String, node: &Node<T>, id: ErrorCode) {
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
            Expr::NumLit(_) => types::Int::new(Some(32)), // Floats aren't real, they can't hurt you
            Expr::BoolLit(_) => types::Bool::new(),
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

    pub fn check(&mut self) {
        for stmt in &self.prog.stmts.clone() {
            self.check_stmt(stmt);
        }
    }
}
