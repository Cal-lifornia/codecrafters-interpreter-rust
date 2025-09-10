use crate::{
    ast::{Expr, Group},
    error::InterpreterError,
    runtime::scope::Scope,
};
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Block(Block),
    If {
        cond: Group,
        if_kind: IfKind,
        if_else: Option<IfKind>,
    },
}

impl Stmt {
    pub fn run(&self, scope: &mut Scope) -> Result<(), InterpreterError> {
        match self {
            Stmt::Expr(expr) => {
                expr.evaluate(scope)?;
            }
            Stmt::Block(block) => {
                block.run(scope)?;
            }
            Stmt::If {
                cond,
                if_kind,
                if_else,
            } => {
                if cond.0.evaluate(scope)?.is_truthy() {
                    if_kind.run(scope)?;
                } else if let Some(else_kind) = if_else {
                    else_kind.run(scope)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn run(&self, scope: &mut Scope) -> Result<(), InterpreterError> {
        scope.add_local();
        for stmt in self.clone().stmts {
            stmt.run(scope)?;
        }
        scope.drop_local();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfKind {
    Stmt(Box<Stmt>),
    Block(Block),
}

impl IfKind {
    pub fn run(&self, scope: &mut Scope) -> Result<(), InterpreterError> {
        match self {
            IfKind::Stmt(stmt) => {
                stmt.run(scope)?;
            }
            IfKind::Block(block) => {
                block.run(scope)?;
            }
        }
        Ok(())
    }
}
