use std::fmt::{Display, Write};

use hashbrown::HashMap;
use lox_ast::{
    ast::{Attribute, Expr, Ident, NodeId},
    parser::{
        Parser,
        token::{Lexer, TokenKind, generate_token_stream},
    },
};
use lox_shared::error::LoxError;

use crate::{
    Resolver,
    environment::Environment,
    std_lib::{Clock, NativeFunction},
    value::{ClassInstance, Value},
};

#[derive(Default)]
pub struct Interpreter {
    env: Environment,
    globals: HashMap<Ident, Value>,
    pub locals: HashMap<NodeId, usize>,
    pub(crate) native_functions: HashMap<Ident, Box<dyn NativeFunction>>,
    within_class: Option<Expr>,
    pub this: Option<ClassInstance>,
}

impl Interpreter {
    pub fn run(&mut self, filename: &str) -> Result<(), LoxError> {
        let mut ast = vec![];
        let mut lexer = Lexer::new(filename)?;
        let stream = generate_token_stream(&mut lexer)?;
        let mut parser = Parser::new(stream);
        loop {
            if parser.current_token == TokenKind::EOF {
                break;
            } else {
                ast.push(parser.parse_stmt()?);
            }
        }
        self.native_functions = HashMap::new();
        self.native_functions
            .insert(Ident("clock".into()), Box::new(Clock {}));

        let mut resolver = Resolver::new(self);
        for stmt in ast.clone() {
            resolver.resolve_stmt(&stmt)?;
        }
        for stmt in ast.clone() {
            self.evaluate_stmt(&stmt)?;
        }
        Ok(())
    }

    pub fn within_class(&self) -> &Option<Expr> {
        &self.within_class
    }

    pub fn enter_class(&mut self, class: Option<Expr>) -> Option<Expr> {
        std::mem::replace(&mut self.within_class, class)
    }

    pub fn exit_class(&mut self, prev: Option<Expr>) {
        self.within_class = prev;
    }

    pub fn enter_scope(&mut self) {
        self.env.enter_scope();
    }

    pub fn exit_scope(&mut self) {
        self.env.exit_scope();
    }

    pub fn capture_env(&self) -> Environment {
        self.env.clone()
    }

    pub fn enter_closure(&mut self, closure: Environment) {
        self.env = closure;
        self.env.enter_scope();
    }

    pub fn exit_closure(&mut self, previous: Environment) {
        self.env.exit_scope();
        self.env = previous;
    }

    pub fn insert(&mut self, ident: Ident, val: Value) {
        if self.env.global_scope() {
            self.globals.insert(ident, val);
        } else {
            self.env.insert(ident, val);
        }
    }

    pub fn find(&self, ident: &Ident, id: &NodeId) -> Option<Value> {
        if let Some(dist) = self.locals.get(id) {
            self.env.find(ident, *dist)
        } else {
            self.globals.get(ident).cloned()
        }
    }

    pub fn update(&mut self, ident: &Ident, id: &NodeId, val: Value) -> Result<(), LoxError> {
        if let Some(dist) = self.locals.get(id) {
            if self.env.update(ident, val, *dist).is_some() {
                return Ok(());
            }
        } else if let Some(var) = self.globals.get_mut(ident) {
            *var = val;
            return Ok(());
        }
        Err(LoxError::Runtime(format!(
            "could not find value for variable {ident}"
        )))
    }

    pub fn print_locals(&self) -> impl Display {
        let mut out = String::new();
        for (id, local) in self.locals.iter() {
            writeln!(out, "{id}: {local}").unwrap();
        }
        out
    }
    pub fn debug_display(&self) -> impl Display {
        let mut out = String::new();
        writeln!(out, "GLOBALS").unwrap();
        for (ident, global) in self.globals.iter() {
            writeln!(out, "{ident}: {global}").unwrap();
        }
        writeln!(out, "LOCALS").unwrap();
        for (id, local) in self.locals.iter() {
            writeln!(out, "{id}: {local}").unwrap();
        }
        writeln!(out, "STACK\n{}", self.env.debug_display()).unwrap();
        out
    }
}

pub(crate) fn runtime_err(attr: &Attribute, out: impl Display) -> LoxError {
    LoxError::Runtime(format!("Error; {}; {out}", attr.as_display()))
}
