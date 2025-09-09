use crate::{ast::Expr, error::InterpreterError, program::Program};

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
    pub fn run(&self, program: &mut Program) -> Result<(), InterpreterError> {
        match self {
            Stmt::Expr(expr) => {
                expr.evaluate(program)?;
            }
            Stmt::Block(block) => {
                for stmt in block.clone().stmts {
                    stmt.run(program)?;
                }
            }
        }
        Ok(())
    }
}
