use std::rc::Rc;

use crate::location::Location;

pub struct Error {
    lines: Rc<Vec<String>>,
    msg: String,
    start: Location,
    end: Location,
    id: u32, // TODO: Convert to enum value
}

fn size(num: u32) -> usize {
    num.to_string().len()
}

impl Error {
    pub fn new(
        lines: Rc<Vec<String>>,
        msg: String,
        start: Location,
        end: Location,
        id: u32,
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
        let Location { idx: end_idx, .. } = self.end;

        let num_size = size(row + 1);

        let mut err = String::new();
        err += &format!("E{:0>4}:\n", self.id);
        for idx in u32::max(row - 4, 0)..=row {
            let line_num = idx + 1;
            err += &format!("{}. {}", line_num, " ".repeat(num_size - size(line_num)));
            err += &format!("{}\n", self.lines.get(idx as usize).unwrap());
        }

        println!("{}", num_size + col as usize + 2);
        err += &format!(
            "{}{}\n",
            " ".repeat(usize::max(num_size + col as usize + 2, 0)),
            "^".repeat((end_idx - idx) as usize + 1),
        );

        err += &format!("{} ({}:{})\n", self.msg, row + 1, col + 1);

        println!("{}", err);
        panic!("");
    }
}
