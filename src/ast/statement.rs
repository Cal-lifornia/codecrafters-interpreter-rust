use crate::ast::Expr;

pub enum Stmt {
    // No leading semicolon
    Expr(Expr),
    // Leading Semicolon
    ExprSemi(Expr),
}
