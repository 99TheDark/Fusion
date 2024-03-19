use crate::{location::Location, types::DataType};

#[derive(Clone)]
pub struct Meta<T> {
    pub src: T,
    pub start: Location,
    pub end: Location,
    pub typ: Option<DataType>,
}

impl<T> Meta<T> {
    pub fn new(src: T, start: Location, end: Location) -> Meta<T> {
        Meta {
            src,
            start,
            end,
            typ: None,
        }
    }
}

impl<T> std::fmt::Debug for Meta<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#?}, {{ start: {}, end: {} }}",
            self.src, self.start, self.end
        )
    }
}
