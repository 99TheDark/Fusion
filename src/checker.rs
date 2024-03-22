use std::{cell::RefCell, rc::Rc};

pub(crate) mod expressions;
pub(crate) mod statements;

use crate::{
    ast::{Expr, Meta, Node},
    error::{Error, ErrorCode},
    program::Program,
    scope::Scope,
    types::{self, DataType},
};

pub struct Checker {
    pub lines: Rc<Vec<String>>,
    pub prog: Program,
    top: Rc<RefCell<Scope>>,
    fn_ret: Option<DataType>,
}

impl Checker {
    pub fn new(lines: Rc<Vec<String>>, prog: Program) -> Checker {
        let top = Rc::clone(&prog.block.scope);
        Checker {
            lines,
            prog,
            top,
            fn_ret: None,
        }
    }

    fn panic<T>(&self, message: String, node: &Meta<T>, id: ErrorCode) {
        Error::new(Rc::clone(&self.lines), message, node.start, node.end, id).panic();
    }

    fn verify_cond(&mut self, cond: &mut Node<Expr>) {
        let cond_typ = self.check_expr(cond);
        if !cond_typ.eq(&types::Bool::new()) {
            self.panic(
                format!("Expected bool, but instead found {}", cond_typ.to_string()),
                &cond,
                ErrorCode::TypeMismatch,
            );
        }
    }

    pub fn check(&mut self) {
        // I really wish there was a better way of doing this...
        let mut prog = self.prog.clone();
        for stmt in &mut prog.block.stmts {
            self.check_stmt(stmt);
            self.prog.block.stmts.push(stmt.clone());
        }
        self.prog = prog;
    }
}
