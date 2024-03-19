use crate::tokens::Type;

use super::{meta::Meta, node::Node, Expr};

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    // TODO: Add ID
}

#[derive(Debug, Clone)]
pub struct NumLit {
    pub val: f32,
}

#[derive(Debug, Clone)]
pub struct BoolLit {
    pub val: bool,
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub op: Meta<Type>,
    pub lhs: Node<Expr>,
    pub rhs: Node<Expr>,
}

#[derive(Debug, Clone)]
pub struct UnaryOp {
    pub op: Meta<Type>,
    pub val: Node<Expr>,
}
