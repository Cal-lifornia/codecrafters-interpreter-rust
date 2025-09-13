use hashbrown::HashMap;

use crate::{
    ast::{evaluate::EvaluateValue, ident::Ident},
    error::InterpreterError,
};

#[derive(Debug, Default, Clone)]
struct ScopeCursor {
    id: usize,
    vars: HashMap<Ident, EvaluateValue>,
}

impl ScopeCursor {
    pub fn new(current_id: usize) -> Self {
        Self {
            id: current_id + 1,
            vars: HashMap::new(),
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn get(&self, ident: &Ident) -> Option<&EvaluateValue> {
        self.vars.get(ident)
    }

    pub fn get_mut(&mut self, ident: &Ident) -> Option<&mut EvaluateValue> {
        self.vars.get_mut(ident)
    }

    pub fn insert_unique(&mut self, ident: Ident, val: EvaluateValue) {
        self.vars.insert(ident, val);
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

    pub fn find_var(&self, ident: &Ident) -> Option<&EvaluateValue> {
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

    pub fn insert_var(&mut self, ident: Ident, val: EvaluateValue) {
        self.local.insert_unique(ident, val);
    }

    pub fn update_var(
        &mut self,
        ident: &Ident,
        val: EvaluateValue,
    ) -> Result<(), InterpreterError> {
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
