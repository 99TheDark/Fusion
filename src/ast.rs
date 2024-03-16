use crate::location::Location;
use crate::tokens::Type;
use crate::types::DataType;

#[derive(Debug, Clone)]
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

pub type Node<T> = Box<Meta<T>>;

// Partial
#[derive(Debug, Clone)]
pub struct Param {
    pub name: Node<Ident>,
    pub annot: Node<Ident>,
}

// Statements
#[derive(Debug, Clone)]
pub enum Stmt {
    Scope(Scope),
    Decl(Decl),
    IfStmt(IfStmt),
    WhileLoop(WhileLoop),
    DoWhileLoop(DoWhileLoop),
    Func(Func),
    Continue,
    Return(Return),
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub stmts: Vec<Node<Stmt>>,
}

impl Scope {
    pub fn print(&self) {
        println!("{}", format!("{:#?}", self).replace("  ", " "));
    }
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub name: Node<Ident>,
    pub annot: Option<Node<Ident>>,
    pub val: Node<Expr>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub cond: Node<Expr>,
    pub body: Node<Scope>,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub cond: Node<Expr>,
    pub body: Node<Scope>,
}

#[derive(Debug, Clone)]
pub struct DoWhileLoop {
    pub body: Node<Scope>,
    pub cond: Node<Expr>,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: Node<Ident>,
    pub args: Vec<Node<Param>>,
    pub ret: Option<Node<Ident>>,
    pub body: Node<Scope>,
    // TODO: Add ID, like Expr::Ident
}

#[derive(Debug, Clone)]
pub struct Return {
    pub val: Option<Node<Expr>>,
}

// Expressions
#[derive(Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    NumLit(NumLit),
    BoolLit(BoolLit),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
}

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
