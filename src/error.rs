use core::fmt;
use std::rc::Rc;

use crate::location::Location;

pub struct Error {
    lines: Rc<Vec<String>>,
    msg: String,
    start: Location,
    end: Location,
    id: ErrorCode,
}

fn size(num: u32) -> usize {
    num.to_string().len()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorCode {
    Unknown,
    UnexpectedToken,
    IncorrectParsingType,
    InvalidStatement,
    InvalidExpression,
    ReservedNameUsed,
    TypeMismatch,
    VariableNotFound,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error {
    pub fn new(
        lines: Rc<Vec<String>>,
        msg: String,
        start: Location,
        end: Location,
        id: ErrorCode,
    ) -> Error {
        Error {
            lines,
            msg,
            start,
            end,
            id,
        }
    }

    pub fn panic(&self) {
        let Location { row, col, idx } = self.start;
        let Location {
            row: end_row,
            idx: ending,
            ..
        } = self.end;

        let end_idx = if end_row == row {
            ending
        } else {
            let mut sum = 0;
            for row in &self.lines[0..=row as usize] {
                // +1 for the new line
                sum += row.len() as u32 + 1;
            }
            sum - 1
        };

        let num_size = size(row + 1);

        let mut err = String::new();
        err += &format!("E{:0>4} {}:\n", self.id as u32, self.id);
        for idx in (i32::max(row as i32 - 4, 0) as u32)..=row {
            let line_num = idx + 1;
            err += &format!("{}. {}", line_num, " ".repeat(num_size - size(line_num)));
            err += &format!("{}\n", self.lines.get(idx as usize).unwrap());
        }

        err += &format!(
            "{}{}\n",
            " ".repeat(usize::max(num_size + col as usize + 2, 0)),
            "^".repeat(u32::max(end_idx - idx, 1) as usize),
        );

        err += &format!("{} ({}:{})\n", self.msg, row + 1, col + 1);

        println!("{}", err);
        panic!("");
    }
}
