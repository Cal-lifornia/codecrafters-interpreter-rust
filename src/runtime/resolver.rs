use hashbrown::HashMap;

use crate::{
    ast::{
        expr::Expr,
        ident::Ident,
        item::Item,
        stmt::{Block, ControlFlowStmt, Stmt},
    },
    error::InterpreterError,
};

#[derive(Default)]
pub struct Resolver {
    pub scopes: Vec<HashMap<Ident, bool>>,
    locals: HashMap<Ident, usize>,
}

impl Resolver {
    pub fn take_locals(&mut self) -> HashMap<Ident, usize> {
        std::mem::take(&mut self.locals)
    }
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn resolve_local(&mut self, ident: &Ident) -> Result<(), InterpreterError> {
        for (idx, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(ident) {
                match scope.get(ident) {
                    Some(false) => {
                        return Err(InterpreterError::Runtime("Found empty var".to_string()))
                    }
                    Some(true) => {
                        self.locals.insert(ident.clone(), idx);
                        break;
                    }
                    None => {
                        continue;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn declare(&mut self, ident: Ident) {
        if let Some(last) = self.scopes.last_mut() {
            last.insert(ident, false);
        }
    }
    pub fn define(&mut self, ident: Ident) {
        if let Some(last) = self.scopes.last_mut() {
            last.insert(ident, true);
        }
    }

    pub fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), InterpreterError> {
        match stmt {
            Stmt::Item(item) => self.resolve_item(item),
            Stmt::Expr(expr) => self.resolve_expr(expr),
            Stmt::Block(block) => self.resolve_block(block),
            Stmt::If(group, if_stmt, else_stmt) => {
                self.resolve_expr(&group.0)?;
                self.resolve_ctrl_flow_stmt(if_stmt)?;
                if let Some(else_stmt) = else_stmt {
                    self.resolve_ctrl_flow_stmt(else_stmt)?;
                }
                Ok(())
            }
            Stmt::WhileLoop(group, control_flow_stmt) => {
                self.resolve_expr(&group.0)?;
                self.resolve_ctrl_flow_stmt(control_flow_stmt)
            }
            Stmt::ForLoop(for_loop_args, control_flow_stmt) => {
                self.enter_scope();
                if let Some(init) = &for_loop_args.init {
                    self.resolve_stmt(init)?;
                }
                self.resolve_stmt(&for_loop_args.cond)?;
                if let Some(stmt) = &for_loop_args.stmt {
                    self.resolve_stmt(stmt)?;
                }
                self.resolve_ctrl_flow_stmt(control_flow_stmt)?;
                self.exit_scope();
                Ok(())
            }
        }
    }

    pub fn resolve_ctrl_flow_stmt(
        &mut self,
        stmt: &ControlFlowStmt,
    ) -> Result<(), InterpreterError> {
        match stmt {
            ControlFlowStmt::Stmt(stmt) => self.resolve_stmt(stmt),
            ControlFlowStmt::Block(block) => self.resolve_block(block),
        }
    }

    pub fn resolve_item(&mut self, item: &Item) -> Result<(), InterpreterError> {
        match item {
            Item::Fun(function) => {
                // self.resolve_local(&function.sig.ident)?;
                self.enter_scope();
                for input in function.sig.inputs.clone() {
                    self.declare(input.clone());
                    self.define(input.clone());
                }
                self.resolve_block(&function.body)?;
                self.exit_scope();
                Ok(())
            }
        }
    }

    pub fn resolve_block(&mut self, block: &Block) -> Result<(), InterpreterError> {
        self.enter_scope();
        for stmt in block.stmts.iter() {
            self.resolve_stmt(stmt)?;
        }
        self.exit_scope();
        Ok(())
    }

    pub fn resolve_expr(&mut self, expr: &Expr) -> Result<(), InterpreterError> {
        match expr {
            Expr::Literal(_) => Ok(()),
            Expr::Group(group) => self.resolve_expr(&group.0),
            Expr::Unary(_, expr) => self.resolve_expr(expr),
            Expr::Arithmetic(_, left, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            }
            Expr::Conditional(_, left, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            }
            Expr::Variable(ident) => self.resolve_local(ident),
            Expr::InitVar(ident, expr) => {
                self.declare(ident.clone());
                self.define(ident.clone());
                self.resolve_expr(expr)?;
                self.resolve_local(ident)
            }
            Expr::UpdateVar(_, expr) => self.resolve_expr(expr),
            Expr::Print(expr) => self.resolve_expr(expr),
            Expr::MethodCall(_, exprs) => {
                self.enter_scope();
                for expr in exprs {
                    self.resolve_expr(expr)?;
                }
                Ok(())
            }
            Expr::Return(Some(val)) => self.resolve_expr(val),
            Expr::Return(None) => Ok(()),
        }
    }
}
