use std::rc::Rc;

use crate::{
    ast::{self, Expr, Meta, Node, Stmt},
    error::{Error, ErrorCode},
    program::Program,
    types::{self, DataType, IntegralSize},
};

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

    fn verify_cond(&mut self, cond: &mut Node<Expr>) {
        let cond_typ = self.check_expr(cond);
        if !cond_typ.eq(&types::Bool::new()) {
            self.panic(
                format!("Expected bool, but instead found {}", cond_typ.to_string()),
                &cond,
                ErrorCode::TypeMismatch,
            )
        }
    }

    // Statements
    fn check_stmt(&mut self, node: &mut Node<Stmt>) {
        match &mut node.src {
            Stmt::Scope(ref mut x) => self.check_scope(x),
            Stmt::Decl(ref mut x) => self.check_decl(x),
            Stmt::IfStmt(ref mut x) => self.check_if_stmt(x),
            Stmt::WhileLoop(ref mut x) => self.check_while_loop(x),
            Stmt::DoWhileLoop(ref mut x) => self.check_do_while_loop(x),
            Stmt::Continue => (),
            _ => self.panic(
                "Invalid statement".to_owned(),
                node,
                ErrorCode::InvalidStatement,
            ),
        }
    }

    fn check_scope(&mut self, scope: &mut ast::Scope) {
        for stmt in &mut scope.stmts {
            self.check_stmt(stmt);
        }
    }

    fn check_decl(&mut self, decl: &mut ast::Decl) {
        let val = self.check_expr(&mut decl.val);
        match &mut decl.annot {
            Some(annot) => {
                let (annot_typ, val_typ) = (annot.src.name.clone(), val.to_string());
                if annot_typ != val_typ {
                    self.panic(
                        format!(
                            "'{}' is defined to be type {}, but assigned {}",
                            decl.name.src.name, annot_typ, val_typ,
                        ),
                        &mut decl.val,
                        ErrorCode::TypeMismatch,
                    )
                }
            }
            None => (),
        };
    }

    fn check_if_stmt(&mut self, if_stmt: &mut ast::IfStmt) {
        self.verify_cond(&mut if_stmt.cond);
        self.check_scope(&mut if_stmt.body.src);
    }

    fn check_while_loop(&mut self, while_loop: &mut ast::WhileLoop) {
        self.verify_cond(&mut while_loop.cond);
        self.check_scope(&mut while_loop.body.src);
    }

    fn check_do_while_loop(&mut self, do_while_loop: &mut ast::DoWhileLoop) {
        self.check_scope(&mut do_while_loop.body.src);
        self.verify_cond(&mut do_while_loop.cond);
    }

    // Expressions
    fn check_expr(&mut self, node: &mut Node<Expr>) -> DataType {
        let typ = match &mut node.src {
            Expr::NumLit(_) => types::Int::new(IntegralSize::Int32), // Floats aren't real, they can't hurt you
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
        };

        node.typ = Some(typ.clone());
        typ
    }

    fn check_binop(&mut self, binop: &mut ast::BinaryOp) -> DataType {
        let left_typ = self.check_expr(&mut binop.lhs);
        let right_typ = self.check_expr(&mut binop.rhs);

        if !left_typ.eq(&right_typ) {
            self.panic(
                format!(
                    "Cannot use the {} operator on {} and {}",
                    binop.op.src.src_strings().get(0).unwrap(),
                    left_typ.to_string(),
                    right_typ.to_string(),
                ),
                &mut binop.op,
                ErrorCode::TypeMismatch,
            )
        }

        left_typ // Since left_typ == right_typ
    }

    fn check_unop(&mut self, unop: &mut ast::UnaryOp) -> DataType {
        let typ = self.check_expr(&mut unop.val);
        typ
    }

    pub fn check(&mut self) {
        // Gotta figure out a better way of doing this
        let mut prog = Program::new();
        for stmt in &mut self.prog.stmts.clone().iter_mut() {
            self.check_stmt(stmt);
            prog.stmts.push(stmt.clone());
        }

        self.prog = prog;
    }
}
