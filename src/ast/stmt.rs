use crate::{
    ast::{item::Item, Expr, Group},
    error::InterpreterError,
    runtime::{loxtype::LoxType, program::Runtime},
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

impl Stmt {
    pub fn run(&self, runtime: &mut Runtime) -> Result<Option<LoxType>, InterpreterError> {
        match self {
            Stmt::Item(item) => {
                item.run(runtime)?;
                Ok(None)
            }
            Stmt::Expr(expr) => expr.evaluate(runtime),
            Stmt::Block(block) => block.run(runtime),
            Stmt::If(cond, if_kind, if_else) => {
                if let Some(cond_eval) = cond.0.evaluate(runtime)? {
                    if cond_eval.is_truthy() {
                        if_kind.run(runtime)
                    } else if let Some(else_kind) = if_else {
                        else_kind.run(runtime)
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(InterpreterError::Runtime(
                        "can't evaluate empty statement".to_string(),
                    ))
                }
            }
            Stmt::WhileLoop(cond, kind) => {
                let mut res: Option<LoxType> = None;
                while cond
                    .0
                    .evaluate(runtime)?
                    .unwrap_or(LoxType::Nil)
                    .is_truthy()
                {
                    res = kind.run(runtime)?;
                    if matches!(res, Some(LoxType::Return(_))) {
                        break;
                    }
                }
                Ok(res)
            }
            Stmt::ForLoop(args, kind) => {
                runtime.scope.add_local();
                let mut res: Option<LoxType> = None;
                if let Some(init) = &args.init {
                    init.run(runtime)?;
                }
                if let Stmt::Expr(cond) = &args.cond {
                    while cond.evaluate(runtime)?.unwrap_or(LoxType::Nil).is_truthy() {
                        res = kind.run(runtime)?;
                        if matches!(res, Some(LoxType::Return(_))) {
                            break;
                        }
                        if let Some(stmt) = &args.stmt {
                            stmt.run(runtime)?;
                        }
                    }
                }
                runtime.scope.drop_local();
                Ok(res)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn run(&self, runtime: &mut Runtime) -> Result<Option<LoxType>, InterpreterError> {
        runtime.scope.add_local();
        for stmt in self.stmts.clone() {
            let result = stmt.run(runtime)?;
            if matches!(result, Some(LoxType::Return(_))) {
                runtime.scope.drop_local();
                return Ok(result);
            }
        }
        runtime.scope.drop_local();
        Ok(Some(LoxType::Nil))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlFlowStmt {
    Stmt(Box<Stmt>),
    Block(Block),
}

impl ControlFlowStmt {
    pub fn run(&self, runtime: &mut Runtime) -> Result<Option<LoxType>, InterpreterError> {
        match self {
            ControlFlowStmt::Stmt(stmt) => stmt.run(runtime),
            ControlFlowStmt::Block(block) => block.run(runtime),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForLoopArgs {
    pub init: Option<Stmt>,
    pub cond: Stmt,
    pub stmt: Option<Stmt>,
}
