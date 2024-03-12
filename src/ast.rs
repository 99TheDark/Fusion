use crate::location::Location;
use crate::tokens::Type;

// TODO: Implement for stmts and exprs
#[derive(Debug)]
pub struct Metadata {
    pub start: Location,
    pub end: Location,
}

// Statements
#[derive(Debug)]
pub enum Stmt {
    Scope(Scope),
    IfStmt(IfStmt),
    WhileLoop(WhileLoop),
    DoWhileLoop(DoWhileLoop),
}

#[derive(Debug)]
pub struct Scope {
    pub stmts: Vec<Box<Stmt>>,
}

#[derive(Debug)]
pub struct IfStmt {
    pub cond: Box<Expr>,
    pub body: Box<Scope>,
}

#[derive(Debug)]
pub struct WhileLoop {
    pub cond: Box<Expr>,
    pub body: Box<Scope>,
}

#[derive(Debug)]
pub struct DoWhileLoop {
    pub cond: Box<BinaryOp>,
    pub body: Box<Scope>,
}

// Expressions
#[derive(Debug)]
pub enum Expr {
    Ident(Ident),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
}

#[derive(Debug)]
pub struct Ident {
    pub name: String,
    // TODO: Add ID
}

#[derive(Debug)]
pub struct BinaryOp {
    pub op: Type,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Debug)]
pub struct UnaryOp {
    pub op: Type,
    pub val: Box<Expr>,
}
