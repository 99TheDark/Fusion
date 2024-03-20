use std::{cell::RefCell, rc::Rc};

pub(crate) mod expressions;
pub(crate) mod statements;

use crate::{
    ast::{Expr, Meta, Node},
    error::{Error, ErrorCode},
    program::Program,
    scope::Scope,
    types,
};

pub struct Checker {
    pub lines: Rc<Vec<String>>,
    pub prog: Program,
    top: Rc<RefCell<Scope>>,
}

impl Checker {
    pub fn new(lines: Rc<Vec<String>>, prog: Program) -> Checker {
        let top = Rc::clone(&prog.block.scope);
        Checker { lines, prog, top }
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
        // Gotta figure out a better way of doing this
        let mut prog = Program::new(Rc::clone(&self.prog.block.scope));
        for stmt in &mut self.prog.block.stmts.clone().iter_mut() {
            self.check_stmt(stmt);
            prog.block.stmts.push(stmt.clone());
        }

        self.prog = prog;
    }
}
