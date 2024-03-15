use std::rc::Rc;

use crate::ast::Stmt;

// TODO: Implement
pub struct Checker {
    lines: Rc<Vec<String>>,
    prog: Vec<Stmt>,
}
