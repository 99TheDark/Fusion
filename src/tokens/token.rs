use core::fmt;

use crate::location::Location;

use super::types::Type;

#[derive(Debug, Clone)]
pub struct Token {
    pub start: Location,
    pub end: Location,
    pub typ: Type,
    pub size: u32,
}

impl Token {
    pub fn new(start: Location, end: Location, typ: Type) -> Token {
        Token {
            start,
            end,
            typ,
            size: end.idx - start.idx,
        }
    }

    pub fn empty() -> Token {
        Token {
            start: Location::empty(),
            end: Location::empty(),
            typ: Type::EOF,
            size: 0,
        }
    }

    pub fn open(start: Location, typ: Type, size: u32) -> Token {
        Token {
            start,
            end: start,
            typ,
            size,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {} to {}", self.typ, self.start, self.end)
    }
}
