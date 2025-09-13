use crate::{
    ast::{evaluate::EvaluateValue, Expr, Group},
    error::InterpreterError,
    runtime::program::Runtime,
};
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // Item(Item),
    Expr(Expr),
    Block(Block),
    If(Group, ControlFlowStmt, Option<ControlFlowStmt>),
    WhileLoop(Group, ControlFlowStmt),
    ForLoop(Box<ForLoopArgs>, ControlFlowStmt),
}

impl Stmt {
    pub fn run(&self, runtime: &mut Runtime) -> Result<(), InterpreterError> {
        match self {
            Stmt::Expr(expr) => {
                expr.evaluate(runtime)?;
            }
            Stmt::Block(block) => {
                block.run(runtime)?;
            }
            Stmt::If(cond, if_kind, if_else) => {
                if cond.0.evaluate(runtime)?.is_truthy() {
                    if_kind.run(runtime)?;
                } else if let Some(else_kind) = if_else {
                    else_kind.run(runtime)?;
                }
            }
            Stmt::WhileLoop(cond, kind) => {
                while cond.0.evaluate(runtime)?.is_truthy() {
                    kind.run(runtime)?;
                }
            }
            Stmt::ForLoop(args, kind) => {
                runtime.scope.add_local();
                if let Some(init) = &args.init {
                    init.run(runtime)?;
                }
                if let Stmt::Expr(cond) = &args.cond {
                    while cond.evaluate(runtime)?.is_truthy() {
                        kind.run(runtime)?;
                        if let Some(stmt) = &args.stmt {
                            stmt.run(runtime)?;
                        }
                    }
                }
                runtime.scope.drop_local();
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
    pub fn run(&self, runtime: &mut Runtime) -> Result<EvaluateValue, InterpreterError> {
        runtime.scope.add_local();
        let mut stmts = self.stmts.clone();
        let last = stmts.pop_if(|stmt| matches!(stmt, Stmt::Expr(Expr::Return(_))));

        for stmt in stmts {
            stmt.run(runtime)?;
        }
        let res = if let Some(Stmt::Expr(expr)) = last {
            expr.evaluate(runtime)?
        } else {
            EvaluateValue::Empty
        };
        runtime.scope.drop_local();
        Ok(res)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlFlowStmt {
    Stmt(Box<Stmt>),
    Block(Block),
}

impl ControlFlowStmt {
    pub fn run(&self, runtime: &mut Runtime) -> Result<(), InterpreterError> {
        match self {
            ControlFlowStmt::Stmt(stmt) => {
                stmt.run(runtime)?;
            }
            ControlFlowStmt::Block(block) => {
                block.run(runtime)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForLoopArgs {
    pub init: Option<Stmt>,
    pub cond: Stmt,
    pub stmt: Option<Stmt>,
}
