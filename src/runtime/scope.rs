use hashbrown::HashMap;

use crate::{ast::evaluate::EvaluateValue, error::InterpreterError};

#[derive(Debug, Default, Clone)]
struct ScopeCursor {
    id: usize,
    vals: HashMap<String, EvaluateValue>,
}

impl ScopeCursor {
    pub fn new(current_id: usize) -> Self {
        Self {
            id: current_id + 1,
            vals: HashMap::new(),
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn get(&self, ident: &str) -> Option<&EvaluateValue> {
        self.vals.get(ident)
    }

    pub fn get_mut(&mut self, ident: &str) -> Option<&mut EvaluateValue> {
        self.vals.get_mut(ident)
    }

    pub fn insert_unique(&mut self, ident: String, val: EvaluateValue) {
        self.vals.insert(ident, val);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Scope {
    local: ScopeCursor,
    stack: Vec<ScopeCursor>,
}

impl Scope {
    pub fn add_local(&mut self) {
        let id = self.local.id();
        self.stack.push(std::mem::take(&mut self.local));
        self.local = ScopeCursor::new(id);
    }

    pub fn drop_local(&mut self) {
        if !self.stack.is_empty() {
            let _ = std::mem::replace(&mut self.local, self.stack.pop().unwrap());
        }
    }

    pub fn find(&self, ident: &str) -> Option<&EvaluateValue> {
        let mut result = self.local.get(ident);
        for scope in self.stack.iter().rev() {
            if result.is_some() {
                break;
            } else {
                result = scope.get(ident);
            }
        }
        result
    }

    pub fn insert(&mut self, ident: String, val: EvaluateValue) {
        self.local.insert_unique(ident, val);
    }

    pub fn update(&mut self, ident: &str, val: EvaluateValue) -> Result<(), InterpreterError> {
        if let Some(res) = self.local.get_mut(ident) {
            *res = val;
            return Ok(());
        } else {
            for scope in self.stack.iter_mut().rev() {
                if let Some(res) = scope.get_mut(ident) {
                    *res = val.clone();
                    return Ok(());
                }
            }
        }
        Err(InterpreterError::Runtime(format!(
            "variable {ident} not found"
        )))
    }
}
