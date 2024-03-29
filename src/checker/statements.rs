use std::rc::Rc;

use crate::{
    ast::{self, Node, Stmt},
    error::ErrorCode,
    types::DataType,
};

use super::Checker;

impl Checker {
    pub(crate) fn check_stmt(&mut self, node: &mut Node<Stmt>) {
        let copy = node.clone();
        match &mut node.src {
            Stmt::Block(ref mut x) => self.check_block(x),
            Stmt::Decl(ref mut x) => self.check_decl(x),
            Stmt::Assign(ref mut x) => self.check_assign(x),
            Stmt::IfStmt(ref mut x) => self.check_if_stmt(x),
            Stmt::WhileLoop(ref mut x) => self.check_while_loop(x),
            Stmt::DoWhileLoop(ref mut x) => self.check_do_while_loop(x),
            Stmt::Continue => (),
            Stmt::Return(ref mut x) => self.check_return(copy, x),
            Stmt::Func(ref mut x) => self.check_func(x),

            // In case any other statements are added
            #[allow(unreachable_patterns)]
            _ => self.panic(
                "Invalid statement".to_owned(),
                node,
                ErrorCode::InvalidStatement,
            ),
        }
    }

    pub(crate) fn check_block(&mut self, block: &mut ast::Block) {
        let top = Rc::clone(&self.top);
        self.top = Rc::clone(&block.scope);

        for stmt in &mut block.stmts {
            self.check_stmt(stmt);
        }

        self.top = top;
    }

    pub(crate) fn check_decl(&mut self, decl: &mut ast::Decl) {
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
                        &decl.val,
                        ErrorCode::TypeMismatch,
                    );
                }
            }
            None => (),
        };

        // Set type in scope
        let name = &decl.name.src.name;
        if let Some(err) = self.top.borrow_mut().set(&name, val) {
            self.panic(
                format!("The variable '{}' does not exist", name),
                &decl.name,
                err,
            );
        }
    }

    pub(crate) fn check_assign(&mut self, assign: &mut ast::Assign) {
        let val = self.check_expr(&mut assign.val);
        // TODO: Check if operator is legal for the type

        let name = &assign.name.src.name;

        match self.top.borrow().get(&name) {
            Ok(vari) => {
                let typ = vari.borrow().typ.as_ref().unwrap().clone();
                if typ != val {
                    self.panic(
                        format!(
                            "Tried to assign type {}, expected type {}",
                            val.to_string(),
                            typ.to_string()
                        ),
                        &assign.name,
                        ErrorCode::TypeMismatch,
                    );
                }
            }
            Err(err) => self.panic(
                format!("The variable '{}' does not exist", name),
                &assign.name,
                err,
            ),
        }
    }

    pub(crate) fn check_if_stmt(&mut self, if_stmt: &mut ast::IfStmt) {
        self.verify_cond(&mut if_stmt.cond);
        self.check_block(&mut if_stmt.body.src);
    }

    pub(crate) fn check_while_loop(&mut self, while_loop: &mut ast::WhileLoop) {
        self.verify_cond(&mut while_loop.cond);
        self.check_block(&mut while_loop.body.src);
    }

    pub(crate) fn check_do_while_loop(&mut self, do_while_loop: &mut ast::DoWhileLoop) {
        self.check_block(&mut do_while_loop.body.src);
        self.verify_cond(&mut do_while_loop.cond);
    }

    pub(crate) fn check_return(&mut self, node: Node<Stmt>, ret: &mut ast::Return) {
        let val = match ret.val {
            Some(ref mut x) => Some(self.check_expr(x)),
            None => None,
        };

        match (&val, &self.fn_ret) {
            (Some(x), Some(y)) => {
                if x != y {
                    self.panic(
                        format!(
                            "Expected a return type of {}, but got type {} instead",
                            x.to_string(),
                            y.to_string(),
                        ),
                        &node,
                        ErrorCode::TypeMismatch,
                    );
                }
            }
            (Some(x), None) => self.panic(
                format!(
                    "Expected no return type, but got type {} instead",
                    x.to_string()
                ),
                &node,
                ErrorCode::TypeMismatch,
            ),
            (None, Some(y)) => self.panic(
                format!(
                    "Expected a return type of {}, but got no type instead",
                    y.to_string()
                ),
                &node,
                ErrorCode::TypeMismatch,
            ),
            _ => (),
        };
    }

    pub(crate) fn check_func(&mut self, func: &mut ast::Func) {
        let prev_ret = self.fn_ret.clone();

        let ret_typ = match func.ret {
            Some(ref mut ret) => {
                ret.typ = DataType::from(&ret.src.name);
                ret.typ.clone()
            }
            None => None,
        };
        self.fn_ret = ret_typ;

        for param in &func.params {
            let name = param.src.name.src.name.clone();
            let typ = DataType::from(&param.src.annot.src.name).unwrap();
            func.body.src.scope.borrow_mut().param(name, typ);
        }

        self.check_block(&mut func.body.src);

        self.fn_ret = prev_ret;
    }
}
