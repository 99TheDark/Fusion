use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::ErrorCode, types::DataType};

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
    varis: HashMap<String, Rc<RefCell<Variable>>>,
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
            Rc::new(RefCell::new(Variable {
                id: unsafe { VARIABLE_ID.clone() },
                typ: None,
            })),
        );
        unsafe { VARIABLE_ID += 1 };
    }

    pub fn param(&mut self, name: String, typ: DataType) {
        self.varis.insert(
            name,
            Rc::new(RefCell::new(Variable {
                id: unsafe { VARIABLE_ID.clone() },
                typ: Some(typ),
            })),
        );
        unsafe { VARIABLE_ID += 1 };
    }

    pub fn set(&self, name: &String, typ: DataType) -> Option<ErrorCode> {
        match self.varis.get(name) {
            Some(val) => {
                val.borrow_mut().typ = Some(typ);
                None
            }
            None => match self.parent {
                Some(ref scope) => scope.as_ref().borrow().set(name, typ),
                None => Some(ErrorCode::VariableNotFound),
            },
        }
    }

    pub fn get(&self, name: &String) -> Result<Rc<RefCell<Variable>>, ErrorCode> {
        match self.varis.get(name) {
            Some(val) if val.borrow().typ.is_some() => Ok(Rc::clone(val)),
            _ => match self.parent {
                Some(ref scope) => scope.as_ref().borrow().get(name),
                None => Err(ErrorCode::VariableNotFound),
            },
        }
    }
}
