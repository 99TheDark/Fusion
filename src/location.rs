use core::fmt;

#[derive(Debug, Copy, Clone)]
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

impl Location {
    pub fn new(row: u32, col: u32, idx: u32) -> Location {
        Location { row, col, idx }
    }

    pub fn empty() -> Location {
        Location {
            row: 0,
            col: 0,
            idx: 0,
        }
    }

    pub fn shift(&mut self, count: u32) -> Location {
        self.idx += count;
        self.col += count;

        self.clone()
    }

    pub fn next(&mut self) -> Location {
        self.shift(1);

        self.clone()
    }
}
