use super::{expression::Ident, node::Node};

#[derive(Debug, Clone)]
pub struct Param {
    pub name: Node<Ident>,
    pub annot: Node<Ident>,
}
