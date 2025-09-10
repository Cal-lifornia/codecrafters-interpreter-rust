use crate::{
    ast::{evaluate::EvaluateValue, Expr, Group},
    error::InterpreterError,
    runtime::scope::Scope,
};
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Block(Block),
    If(Group, IfKind),
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
            Stmt::If(group, kind) => {
                if matches!(group.0.evaluate(scope)?, EvaluateValue::Boolean(true)) {
                    match kind {
                        IfKind::Expr(expr) => {
                            expr.evaluate(scope)?;
                        }
                        IfKind::Block(block) => {
                            block.run(scope)?;
                        }
                    }
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
    Expr(Box<Expr>),
    Block(Block),
}
