use hashbrown::HashMap;
use lox_ast::ast::{
    Block, ControlFlowStmt, Expr, ExprKind, Ident, Item, ItemKind, NodeId, Stmt, StmtKind,
};
use lox_shared::error::LoxError;

#[derive(Default)]
pub struct Resolver {
    pub scopes: Vec<HashMap<Ident, bool>>,
    locals: HashMap<NodeId, usize>,
}

impl Resolver {
    pub fn take_locals(&mut self) -> HashMap<NodeId, usize> {
        std::mem::take(&mut self.locals)
    }
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
                    self.locals.insert(id, idx);
                    break;
                }
                None => {
                    continue;
                }
            }
        }
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
            StmtKind::Block(block) => self.resolve_block(block, true),
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
            ControlFlowStmt::Stmt(stmt) => self.resolve_stmt(stmt),
            ControlFlowStmt::Block(block) => self.resolve_block(block, true),
        }
    }

    pub fn resolve_item(&mut self, item: &Item) -> Result<(), LoxError> {
        match item.kind() {
            ItemKind::Fun(function) => {
                self.declare(function.sig.ident.clone())?;
                self.define(function.sig.ident.clone());
                self.enter_scope();
                for input in function.sig.inputs.clone() {
                    self.declare(input.clone())?;
                    self.define(input.clone());
                }
                self.resolve_block(&function.body, false)?;
                self.exit_scope();
                Ok(())
            }
        }
    }

    pub fn resolve_block(&mut self, block: &Block, enter_scope: bool) -> Result<(), LoxError> {
        if enter_scope {
            self.enter_scope();
        }
        for stmt in block.stmts.iter() {
            self.resolve_stmt(stmt)?;
        }
        if enter_scope {
            self.exit_scope();
        }
        Ok(())
    }

    pub fn resolve_expr(&mut self, expr: &Expr) -> Result<(), LoxError> {
        match expr.kind() {
            ExprKind::Literal(_) => Ok(()),
            ExprKind::Group(group) => self.resolve_expr(&group.0),
            ExprKind::Unary(_, expr) => self.resolve_expr(expr),
            ExprKind::Arithmetic(_, left, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            }
            ExprKind::Conditional(_, left, right) => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            }
            ExprKind::Variable(ident) => self.resolve_local(ident, expr.attr().id().clone()),
            ExprKind::InitVar(ident, sub_expr) => {
                self.declare(ident.clone())?;
                self.resolve_expr(sub_expr)?;
                self.define(ident.clone());
                self.resolve_local(ident, expr.attr.id().clone())
            }
            ExprKind::UpdateVar(ident, expr) => {
                self.resolve_expr(expr)?;
                self.resolve_local(ident, expr.attr().id().clone())
            }
            ExprKind::Print(expr) => self.resolve_expr(expr),
            ExprKind::MethodCall(ident, exprs) => {
                self.resolve_local(ident, expr.attr().id().clone())?;
                for expr in exprs {
                    self.resolve_expr(expr)?;
                }
                Ok(())
            }
            ExprKind::Return(Some(val)) => self.resolve_expr(val),
            ExprKind::Return(None) => Ok(()),
        }
    }
}
