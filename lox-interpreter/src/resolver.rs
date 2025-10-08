use std::fmt::Display;

use hashbrown::HashMap;
use lox_ast::ast::{
    Attribute, Block, ControlFlowStmt, Expr, ExprKind, Function, Ident, Item, ItemKind, NodeId,
    Stmt, StmtKind,
};
use lox_shared::error::LoxError;

use crate::Interpreter;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    pub scopes: Vec<HashMap<Ident, bool>>,
    within_function: bool,
    within_initialiser: bool,
    in_class: bool,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
            within_function: false,
            in_class: false,
            within_initialiser: false,
        }
    }
}

impl<'a> Resolver<'a> {
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn resolve_local(&mut self, ident: &Ident, id: NodeId) -> Result<(), LoxError> {
        for (idx, scope) in self.scopes.iter().rev().enumerate() {
            match scope.get(ident) {
                Some(false) => {
                    return Err(LoxError::Compile(format!(
                        "Error at '{ident}'; Can't read local variable in it's own initialiser",
                    )));
                }
                Some(true) => {
                    tracing::debug!("inserted {ident} with id {id} with scope num {idx}");

                    self.interpreter.locals.insert(id, idx);
                    break;
                }
                None => {
                    continue;
                }
            }
        }

        // if self.scopes.is_empty() && cfg!(debug_assertions) {
        //     println!("Scopes empty when looking for '{ident}'");
        // }
        Ok(())
    }

    pub fn declare(&mut self, ident: Ident) -> Result<(), LoxError> {
        if let Some(last) = self.scopes.last_mut()
            && last.insert(ident.clone(), false).is_some()
        {
            return Err(LoxError::Compile(format!(
                "Error at '{ident}; Already a variable with this name in this scope"
            )));
        }
        Ok(())
    }

    pub fn define(&mut self, ident: Ident) {
        if let Some(last) = self.scopes.last_mut() {
            last.insert(ident, true);
        }
    }

    pub fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        match stmt.kind() {
            StmtKind::Item(item) => self.resolve_item(item),
            StmtKind::Expr(expr) => self.resolve_expr(expr),
            StmtKind::Block(block) => self.resolve_block(block, false),
            StmtKind::If(group, if_stmt, else_stmt) => {
                self.resolve_expr(&group.0)?;
                self.resolve_ctrl_flow_stmt(if_stmt)?;
                if let Some(else_stmt) = else_stmt {
                    self.resolve_ctrl_flow_stmt(else_stmt)?;
                }
                Ok(())
            }
            StmtKind::WhileLoop(group, control_flow_stmt) => {
                self.resolve_expr(&group.0)?;
                self.resolve_ctrl_flow_stmt(control_flow_stmt)
            }
            StmtKind::ForLoop(for_loop_args, control_flow_stmt) => {
                self.enter_scope();
                if let Some(init) = &for_loop_args.init {
                    self.resolve_stmt(init)?;
                }
                self.resolve_stmt(&for_loop_args.cond)?;
                self.resolve_ctrl_flow_stmt(control_flow_stmt)?;
                if let Some(stmt) = &for_loop_args.stmt {
                    self.resolve_stmt(stmt)?;
                }
                self.exit_scope();
                Ok(())
            }
        }
    }

    pub fn resolve_ctrl_flow_stmt(&mut self, stmt: &ControlFlowStmt) -> Result<(), LoxError> {
        match stmt {
            ControlFlowStmt::Stmt(ctrl_stmt) => self.resolve_stmt(ctrl_stmt),
            ControlFlowStmt::Block(block) => self.resolve_block(block, false),
        }
    }

    pub fn resolve_item(&mut self, item: &Item) -> Result<(), LoxError> {
        match item.kind() {
            ItemKind::Fun(fun) => self.resolve_function(fun),
            ItemKind::Class(class) => {
                let ident = &class.ident;
                self.declare(ident.clone())?;

                if let Some(super_class) = &class.super_class {
                    if ident == super_class {
                        return Err(compile_error(
                            item.attr(),
                            "Can't use own class as super class",
                        ));
                    }
                    self.resolve_local(super_class, item.attr().id().clone())?;
                }

                self.define(ident.clone());

                self.enter_scope();

                self.declare(Ident("this".into()))?;
                self.define(Ident("this".into()));

                self.declare(Ident("super".into()))?;
                self.define(Ident("super".into()));

                self.in_class = true;
                for fun in &class.methods {
                    self.resolve_function(fun)?;
                }
                self.exit_scope();
                self.in_class = false;
                Ok(())
            }
        }
    }

    pub fn resolve_function(&mut self, fun: &Function) -> Result<(), LoxError> {
        let already_in_function = self.within_function;
        if fun.sig.ident.0 == "init" {
            self.within_initialiser = true;
        }
        self.declare(fun.sig.ident.clone())?;
        self.define(fun.sig.ident.clone());
        self.within_function = true;
        self.enter_scope();
        for input in fun.sig.inputs.clone() {
            self.declare(input.clone())?;
            self.define(input.clone());
        }
        self.resolve_block(&fun.body, true)?;
        self.exit_scope();
        if !already_in_function {
            self.within_function = false;
        }
        if fun.sig.ident.0 == "init" {
            self.within_initialiser = false;
        }
        Ok(())
    }

    pub fn resolve_block(
        &mut self,
        block: &Block,
        in_function_scope: bool,
    ) -> Result<(), LoxError> {
        if !in_function_scope {
            self.enter_scope();
        }
        for stmt in block.stmts.iter() {
            self.resolve_stmt(stmt)?;
        }
        if !in_function_scope {
            self.exit_scope();
        }
        Ok(())
    }

    pub fn resolve_expr(&mut self, expr: &Expr) -> Result<(), LoxError> {
        match expr.kind() {
            ExprKind::Literal(_) => {}
            ExprKind::Group(group) => self.resolve_expr(&group.0)?,
            ExprKind::Unary(_, expr) => self.resolve_expr(expr)?,
            ExprKind::Arithmetic(_, left, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            ExprKind::Conditional(_, left, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            ExprKind::Variable(ident) => self.resolve_local(ident, expr.attr().id().clone())?,
            ExprKind::InitVar(ident, sub_expr) => {
                self.declare(ident.clone())?;
                self.resolve_expr(sub_expr)?;
                self.define(ident.clone());
                self.resolve_local(ident, expr.attr().id().clone())?;
            }
            ExprKind::UpdateVar(ident, sub_expr) => {
                self.resolve_local(ident, expr.attr().id().clone())?;
                self.resolve_expr(sub_expr)?;
            }
            ExprKind::Print(expr) => self.resolve_expr(expr)?,
            ExprKind::FunctionCall(expr, exprs) => {
                self.resolve_expr(expr)?;
                for expr in exprs {
                    self.resolve_expr(expr)?;
                }
            }
            ExprKind::Get(left, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            ExprKind::Set(expr, _, sub_expr) => {
                self.resolve_expr(expr)?;
                self.resolve_expr(sub_expr)?;
            }
            ExprKind::Return(opt) => {
                if self.within_function {
                    if let Some(val) = opt {
                        if self.within_initialiser {
                            return Err(LoxError::Compile(format!(
                                "{}; Return statements must be empty within Class initialisers",
                                expr.attr().as_display()
                            )));
                        }
                        self.resolve_expr(val)?
                    }
                } else {
                    return Err(LoxError::Compile(format!(
                        "{}; Error at 'return': Can't return from non-function scopes",
                        expr.attr().as_display()
                    )));
                }
            }
            ExprKind::This => {
                if self.in_class && self.within_function {
                    self.resolve_local(&Ident("this".into()), expr.attr().id().clone())?;
                } else {
                    return Err(LoxError::Compile(format!(
                        "{}; keyword 'this' can only be used within class methods",
                        expr.attr().as_display()
                    )));
                }
            }
            ExprKind::Super => {
                if self.in_class && self.within_function {
                    self.resolve_local(&Ident("super".into()), expr.attr().id().clone())?;
                } else {
                    return Err(LoxError::Compile(format!(
                        "{}; keyword 'super' can only be used within class methods",
                        expr.attr().as_display()
                    )));
                }
            }
        }
        Ok(())
    }
}

fn compile_error(attr: &Attribute, out: impl Display) -> LoxError {
    LoxError::Compile(format!("{}; {out}", attr.as_display()))
}
