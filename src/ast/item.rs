use crate::ast::Expr;

pub enum Item {
    Base(BaseItem),
}

pub enum BaseItem {
    Block(Block),
    Stmt(Stmt),
}

pub struct Block {
    pub stmts: Vec<Stmt>,
}

pub enum Stmt {
    // No leading semicolon
    Expr(Expr),
    // Leading Semicolon
    ExprSemi(Expr),
}
