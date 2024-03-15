use crate::ast::Stmt;

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Box<Stmt>>,
}

impl Program {
    pub fn new() -> Program {
        Program { stmts: Vec::new() }
    }

    pub fn print(&self) {
        println!("{}", format!("{:#?}", self).replace("  ", " "));
    }
}
