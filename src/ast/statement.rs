use crate::{ast::Expr, context::Context, program::Program};

pub enum Stmt {
    // No leading semicolon
    Expr(Expr),
    // Leading Semicolon
    ExprSemi(Expr),
}
