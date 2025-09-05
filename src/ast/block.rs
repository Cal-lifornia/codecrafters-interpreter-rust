use crate::{ast::Stmt, context::Context};

pub struct Block {
    pub stmts: Vec<Stmt>,
}
