use crate::location::Location;
use crate::tokens::Type;
use crate::types::DataType;

// TODO: Implement for stmts and exprs
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Param {
    pub name: Box<Ident>,
    pub annot: Box<Ident>,
}

// Statements
#[derive(Debug)]
pub enum Stmt {
    Scope(Meta<Scope>),
    Decl(Meta<Decl>),
    IfStmt(Meta<IfStmt>),
    WhileLoop(Meta<WhileLoop>),
    DoWhileLoop(Meta<DoWhileLoop>),
    Func(Meta<Func>),
}

#[derive(Debug)]
pub struct Scope {
    pub stmts: Vec<Box<Stmt>>,
}

#[derive(Debug)]
pub struct Decl {
    pub name: Box<Ident>,
    pub annot: Option<Box<Ident>>,
    pub val: Box<Expr>,
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
    pub body: Box<Scope>,
    pub cond: Box<Expr>,
}

#[derive(Debug)]
pub struct Func {
    pub name: Box<Ident>,
    pub args: Vec<Box<Param>>,
    pub ret: Box<Ident>,
    pub body: Box<Scope>,
    // TODO: Add ID, like Expr::Ident
}

// Expressions
#[derive(Debug)]
pub enum Expr {
    Ident(Meta<Ident>),
    NumLit(Meta<NumLit>),
    BoolLit(Meta<BoolLit>),
    BinaryOp(Meta<BinaryOp>),
    UnaryOp(Meta<UnaryOp>),
}

#[derive(Debug)]
pub struct Ident {
    pub name: String,
    // TODO: Add ID
}

#[derive(Debug)]
pub struct NumLit {
    pub val: f32,
}

#[derive(Debug)]
pub struct BoolLit {
    pub val: bool,
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
