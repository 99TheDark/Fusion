pub(crate) mod expression;
pub(crate) mod meta;
pub(crate) mod misc;
pub(crate) mod node;
pub(crate) mod statement;

pub use self::{
    expression::{BinaryOp, BoolLit, Ident, NumLit, UnaryOp},
    meta::Meta,
    misc::Param,
    node::Node,
    statement::{Assign, Block, Decl, DoWhileLoop, Func, IfStmt, Return, WhileLoop},
};

#[derive(Clone)]
pub enum Stmt {
    Block(Block),
    Decl(Decl),
    Assign(Assign),
    IfStmt(IfStmt),
    WhileLoop(WhileLoop),
    DoWhileLoop(DoWhileLoop),
    Func(Func),
    Continue,
    Return(Return),
}

impl std::fmt::Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // I also hate this implementation...
        match self {
            Stmt::Block(x) => write!(f, "{:#?}", x),
            Stmt::Decl(x) => write!(f, "{:#?}", x),
            Stmt::Assign(x) => write!(f, "{:#?}", x),
            Stmt::IfStmt(x) => write!(f, "{:#?}", x),
            Stmt::WhileLoop(x) => write!(f, "{:#?}", x),
            Stmt::DoWhileLoop(x) => write!(f, "{:#?}", x),
            Stmt::Func(x) => write!(f, "{:#?}", x),
            Stmt::Continue => write!(f, "Continue"),
            Stmt::Return(x) => write!(f, "{:#?}", x),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    NumLit(NumLit),
    BoolLit(BoolLit),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
}
