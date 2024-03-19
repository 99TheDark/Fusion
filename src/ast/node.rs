use crate::location::Location;

use super::meta::Meta;

#[derive(Clone)]
pub struct Node<T>(pub Box<Meta<T>>);

impl<T> Node<T> {
    pub fn new(src: T, start: Location, end: Location) -> Node<T> {
        Node(Box::new(Meta::new(src, start, end)))
    }
}

impl<T> std::ops::Deref for Node<T> {
    type Target = Box<Meta<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> std::fmt::Debug for Node<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}
