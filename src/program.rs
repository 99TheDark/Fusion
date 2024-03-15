use crate::ast::{Node, Stmt};

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Node<Stmt>>,
}

impl Program {
    pub fn new() -> Program {
        Program { stmts: Vec::new() }
    }

    pub fn print(&self) {
        println!("{}", format!("{:#?}", self).replace("  ", " "));
    }
}
