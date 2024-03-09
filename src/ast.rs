use crate::tokens::Type;

pub enum Node {
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Condition(Condition),
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

pub struct Condition {
    pub comp: Type,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

pub struct IfStmt {
    pub cond: Box<Condition>,
    pub body: Vec<Box<Node>>,
}
