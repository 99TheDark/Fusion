use std::{cell::RefCell, rc::Rc};

use crate::{scope::Scope, tokens::Type};

use super::{expression::Ident, meta::Meta, misc::Param, node::Node, Expr, Stmt};

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
pub struct Assign {
    pub name: Node<Ident>,
    pub op: Option<Meta<Type>>, // TODO: replace Meta<Type> (here and at binop + unop) with Token
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
    pub params: Vec<Node<Param>>,
    pub ret: Option<Node<Ident>>,
    pub body: Node<Block>,
    // TODO: Add ID, like Expr::Ident
}

#[derive(Debug, Clone)]
pub struct Return {
    pub val: Option<Node<Expr>>,
}
