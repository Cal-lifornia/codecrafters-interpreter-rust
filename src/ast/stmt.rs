use crate::{ast::Expr, error::InterpreterError, runtime::scope::Scope};

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Block(Block),
}

impl Stmt {
    pub fn run(&self, scope: &mut Scope) -> Result<(), InterpreterError> {
        match self {
            Stmt::Expr(expr) => {
                expr.evaluate(scope)?;
            }
            Stmt::Block(block) => {
                scope.add_local();
                for stmt in block.clone().stmts {
                    stmt.run(scope)?;
                }
                scope.drop_local();
            }
        }
        Ok(())
    }
}
