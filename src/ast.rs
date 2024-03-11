use crate::tokens::Type;

pub enum Node {
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    IfStmt(IfStmt),
}

pub struct BinaryOp {
    pub op: Type,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

pub struct UnaryOp {
    pub op: Type,
    pub val: Box<Node>,
}

pub struct IfStmt {
    pub cond: Box<BinaryOp>,
    pub body: Vec<Box<Node>>,
}
