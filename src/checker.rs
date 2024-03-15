use std::rc::Rc;

use crate::program::Program;

// TODO: Implement
pub struct Checker {
    pub lines: Rc<Vec<String>>,
    pub prog: Program,
}

impl Checker {
    pub fn new(lines: Rc<Vec<String>>, prog: Program) -> Checker {
        Checker { lines, prog }
    }
}
