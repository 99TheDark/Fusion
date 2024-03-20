use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{Block, Func},
    scope::Scope,
};

#[derive(Debug, Clone)]
pub struct Program {
    pub block: Block,
    pub funcs: Vec<Func>,
}

impl Program {
    pub fn new(scope: Rc<RefCell<Scope>>) -> Program {
        Program {
            block: Block {
                stmts: Vec::new(),
                scope,
            },
            funcs: Vec::new(),
        }
    }
}
