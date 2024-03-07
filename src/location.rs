pub struct Location {
    pub row: u32,
    pub col: u32,
    pub idx: u32,
}

impl Location {
    pub fn to_string(&self) -> String {
        format!("({}, {}) #{}", self.row, self.col + 1, self.idx)
    }
}
