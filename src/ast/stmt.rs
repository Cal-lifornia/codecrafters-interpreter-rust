use crate::ast::{
    expr::{Expr, Group},
    item::Item,
};
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Item(Item),
    Expr(Expr),
    Block(Block),
    If(Group, ControlFlowStmt, Option<ControlFlowStmt>),
    WhileLoop(Group, ControlFlowStmt),
    ForLoop(Box<ForLoopArgs>, ControlFlowStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlFlowStmt {
    Stmt(Box<Stmt>),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForLoopArgs {
    pub init: Option<Stmt>,
    pub cond: Stmt,
    pub stmt: Option<Stmt>,
}
