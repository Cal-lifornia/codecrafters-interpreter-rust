use std::{cell::RefCell, rc::Rc};

use hashbrown::HashMap;
use lox_ast::ast::Ident;

use crate::Value;

type ScopeCell = Rc<RefCell<Scope>>;

#[derive(Debug, Default, Clone)]
struct Scope {
    values: HashMap<Ident, Value>,
}

impl Scope {
    fn get(&self, ident: &Ident) -> Option<&Value> {
        self.values.get(ident)
    }

    fn set(&mut self, ident: Ident, val: Value) {
        self.values.insert(ident, val);
    }

    fn get_mut(&mut self, ident: &Ident) -> Option<&mut Value> {
        self.values.get_mut(ident)
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    stack: Vec<ScopeCell>,
    depth: usize,
}

impl Environment {
    pub fn global_scope(&self) -> bool {
        self.depth == 0
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(Rc::new(RefCell::new(Scope::default())));
        self.depth += 1;
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop().unwrap();
        self.depth -= 1;
    }

    pub fn find(&self, ident: &Ident, dist: usize) -> Option<Value> {
        if let Some(scope) = self.stack.get(dist) {
            scope.borrow().get(ident).cloned()
        } else {
            panic!("attempted to access out of bounds index scope")
        }
    }

    pub fn insert(&mut self, ident: Ident, val: Value) {
        self.stack.last().unwrap().borrow_mut().set(ident, val);
    }

    pub fn update(&mut self, ident: &Ident, val: Value, dist: usize) -> Option<()> {
        if let Some(scope) = self.stack.get(dist) {
            if let Some(var) = scope.borrow_mut().get_mut(ident) {
                *var = val;
                Some(())
            } else {
                None
            }
        } else {
            panic!("attempted to access out of bounds index scope")
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            stack: vec![Rc::new(RefCell::new(Scope::default()))],
            depth: 0,
        }
    }
}
