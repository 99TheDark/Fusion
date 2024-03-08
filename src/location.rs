use core::fmt;

#[derive(Copy, Clone)]
pub struct Location {
    pub row: u32,
    pub col: u32,
    pub idx: u32,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) #{}", self.row + 1, self.col + 1, self.idx)
    }
}
