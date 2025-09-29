use std::{cell::RefCell, rc::Rc};

use hashbrown::HashMap;
use lox_shared::error::LoxError;

use crate::Value;

type ScopeCell = Rc<RefCell<Scope>>;

#[derive(Debug, Default, Clone)]
struct Scope {
    values: HashMap<usize, Value>,
}

impl Scope {
    fn get(&self, id: &usize) -> Option<&Value> {
        self.values.get(id)
    }

    fn set(&mut self, id: usize, val: Value) {
        self.values.insert(id, val);
    }

    fn get_mut(&mut self, id: &usize) -> Option<&mut Value> {
        self.values.get_mut(id)
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<ScopeCell>,
    depth: usize,
}

impl Environment {
    pub fn find(&self, id: &usize) -> Option<Value> {
        self.scopes.last().unwrap().borrow().get(id).cloned()
    }

    pub fn insert(&mut self, id: usize, val: Value) {
        self.scopes.last().unwrap().borrow_mut().set(id, val);
    }

    pub fn update(&mut self, id: &usize, val: Value) -> Result<(), LoxError> {
        if let Some(var) = self.scopes.last().unwrap().borrow_mut().get_mut(id) {
            *var = val;
            Ok(())
        } else {
            Err(LoxError::Runtime(
                format!("Couldn't find variable: {val}").into(),
            ))
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            scopes: vec![Rc::new(RefCell::new(Scope::default()))],
            depth: 0,
        }
    }
}
