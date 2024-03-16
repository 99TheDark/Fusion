use std::rc::Rc;

use crate::types::DataType;

// Variable
pub struct Variable {
    pub name: String,
    pub id: u32,
    pub typ: DataType,
}

pub static VARIABLE_ID: u32 = 0;

// Scope
pub struct Scope {
    pub parent: Rc<Option<Scope>>,
}

impl Scope {
    pub fn new(parent: Rc<Option<Scope>>) -> Scope {
        Scope {
            parent: Rc::clone(&parent),
        }
    }
}
