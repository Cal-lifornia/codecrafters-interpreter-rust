use hashbrown::HashMap;
use lox_ast::{
    ast::{Ident, NodeId, StmtKind},
    parser::{
        Parser,
        token::{Lexer, TokenKind, generate_token_stream},
    },
};
use lox_shared::error::LoxError;

use crate::{Resolver, Value, environment::Environment};

#[derive(Default)]
pub struct Interpreter {
    env: Environment,
    globals: HashMap<Ident, Value>,
    locals: HashMap<NodeId, usize>,
    // native_functions: HashMap<FunSig, Box<dyn NativeFunction>>,
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
        let mut resolver = Resolver::default();

        for stmt in ast.clone() {
            if matches!(stmt.kind(), StmtKind::Item(_) | StmtKind::Block(_)) {
                resolver.resolve_stmt(&stmt)?;
            }
        }
        self.locals = resolver.take_locals();
        for stmt in ast.clone() {
            self.evaluate_stmt(&stmt)?;
        }
        Ok(())
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

    pub fn set_env(&mut self, env: Environment) {
        self.env = env
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

    pub fn find(&mut self, ident: &Ident, id: &NodeId) -> Option<Value> {
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
        Err(LoxError::Runtime(
            format!("could not find value for variable {ident}").into(),
        ))
    }
}
