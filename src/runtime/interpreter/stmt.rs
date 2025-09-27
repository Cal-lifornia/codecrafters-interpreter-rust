use crate::{
    ast::stmt::{Block, ControlFlowStmt, Stmt},
    error::InterpreterError,
    runtime::{
        interpreter::{EvaluateResult, Interpreter},
        loxtype::LoxType,
    },
};

impl Interpreter {
    pub fn evaluate_stmt(&mut self, stmt: &Stmt) -> EvaluateResult {
        match stmt {
            Stmt::Item(item) => {
                self.evaluate_item(item)?;
                Ok(None)
            }
            Stmt::Expr(expr) => self.evaluate_expr(expr),
            Stmt::Block(block) => self.evaluate_block(block),
            Stmt::If(cond, if_kind, if_else) => {
                if let Some(cond_eval) = self.evaluate_expr(&cond.0)? {
                    // println!("cond_eval: {cond_eval}");
                    // println!("truthy cond_eval: {}", cond_eval.is_truthy());
                    // println!("if block; cond: {cond:#?}, if_kind: {if_kind:#?}");
                    if cond_eval.is_truthy() {
                        self.evaluate_control_flow_stmt(if_kind)
                    } else if let Some(else_kind) = if_else {
                        self.evaluate_control_flow_stmt(else_kind)
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
                while self
                    .evaluate_expr(&cond.0)?
                    .unwrap_or(LoxType::Nil)
                    .is_truthy()
                {
                    res = self.evaluate_control_flow_stmt(kind)?;
                    if matches!(res, Some(LoxType::Return(_))) {
                        break;
                    }
                }
                Ok(res)
            }
            Stmt::ForLoop(args, kind) => {
                self.env.enter_scope();
                let mut res: Option<LoxType> = None;
                if let Some(init) = &args.init {
                    self.evaluate_stmt(init)?;
                }
                if let Stmt::Expr(cond) = &args.cond {
                    while self
                        .evaluate_expr(cond)?
                        .unwrap_or(LoxType::Nil)
                        .is_truthy()
                    {
                        res = self.evaluate_control_flow_stmt(kind)?;
                        if matches!(res, Some(LoxType::Return(_))) {
                            break;
                        }
                        if let Some(stmt) = &args.stmt {
                            self.evaluate_stmt(stmt)?;
                        }
                    }
                }
                self.env.exit_scope();
                Ok(res)
            }
        }
    }
    pub fn evaluate_block(&mut self, block: &Block) -> EvaluateResult {
        self.env.enter_scope();
        for stmt in block.stmts.clone() {
            let result = self.evaluate_stmt(&stmt)?;
            if matches!(result, Some(LoxType::Return(_))) {
                self.env.exit_scope();
                return Ok(result);
            }
        }
        self.env.exit_scope();
        Ok(Some(LoxType::Nil))
    }

    pub fn evaluate_control_flow_stmt(&mut self, stmt: &ControlFlowStmt) -> EvaluateResult {
        match stmt {
            ControlFlowStmt::Stmt(stmt) => self.evaluate_stmt(stmt),
            ControlFlowStmt::Block(block) => self.evaluate_block(block),
        }
    }
}
