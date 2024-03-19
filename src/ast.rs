use std::cell::RefCell;
use std::rc::Rc;

use crate::location::Location;
use crate::scope::Scope;
use crate::tokens::Type;
use crate::types::DataType;

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

// Partial
#[derive(Debug, Clone)]
pub struct Param {
    pub name: Node<Ident>,
    pub annot: Node<Ident>,
}

// Statements
#[derive(Clone)]
pub enum Stmt {
    Block(Block),
    Decl(Decl),
    IfStmt(IfStmt),
    WhileLoop(WhileLoop),
    DoWhileLoop(DoWhileLoop),
    Func(Func),
    Continue,
    Return(Return),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Node<Stmt>>,
    pub scope: Rc<RefCell<Scope>>,
}

impl Block {
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
    pub body: Node<Block>,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub cond: Node<Expr>,
    pub body: Node<Block>,
}

#[derive(Debug, Clone)]
pub struct DoWhileLoop {
    pub body: Node<Block>,
    pub cond: Node<Expr>,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: Node<Ident>,
    pub args: Vec<Node<Param>>,
    pub ret: Option<Node<Ident>>,
    pub body: Node<Block>,
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

impl std::fmt::Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // I also hate this implementation...
        match self {
            Stmt::Block(x) => write!(f, "{:#?}", x),
            Stmt::Decl(x) => write!(f, "{:#?}", x),
            Stmt::IfStmt(x) => write!(f, "{:#?}", x),
            Stmt::WhileLoop(x) => write!(f, "{:#?}", x),
            Stmt::DoWhileLoop(x) => write!(f, "{:#?}", x),
            Stmt::Func(x) => write!(f, "{:#?}", x),
            Stmt::Continue => write!(f, "Continue"),
            Stmt::Return(x) => write!(f, "{:#?}", x),
        }
    }
}
