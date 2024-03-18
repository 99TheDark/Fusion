use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::types::DataType;

// Variable
#[derive(Debug, Clone)]
pub struct Variable {
    pub id: u32,
    pub typ: Option<DataType>,
}

pub static mut VARIABLE_ID: u32 = 0;

// Scop3e
#[derive(Debug, Clone)]
pub struct Scope {
    pub parent: Option<Rc<RefCell<Scope>>>,
    varis: HashMap<String, Variable>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Rc<RefCell<Scope>> {
        Rc::new(RefCell::new(Scope {
            parent,
            varis: HashMap::new(),
        }))
    }

    pub fn declare(&mut self, name: String) {
        self.varis.insert(
            name,
            Variable {
                id: unsafe { VARIABLE_ID.clone() },
                typ: None,
            },
        );
        unsafe { VARIABLE_ID += 1 };
    }
}
