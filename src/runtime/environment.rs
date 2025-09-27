use std::{cell::RefCell, rc::Rc};

use hashbrown::HashMap;

use crate::{
    ast::{ident::Ident, item::FunSig},
    error::InterpreterError,
    native::{time::Clock, NativeFunction},
    runtime::loxtype::LoxType,
};

pub type Ctx = Rc<RefCell<Context>>;

#[derive(Debug, Clone)]
pub struct Context {
    values: HashMap<Ident, LoxType>,
}

impl Context {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Context {
            values: HashMap::new(),
        }))
    }

    pub fn set(&mut self, ident: Ident, val: LoxType) {
        self.values.insert(ident, val);
    }

    pub fn get(&self, ident: &Ident) -> Option<&LoxType> {
        self.values.get(ident)
    }

    pub fn get_mut(&mut self, ident: &Ident) -> Option<&mut LoxType> {
        self.values.get_mut(ident)
    }
}
#[derive(Debug, Clone)]
pub struct Environment {
    ctx: Vec<Ctx>,
    pub locals: HashMap<Ident, usize>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
            ctx: vec![Context::new()],
        }
    }

    pub fn global_scope(&self) -> bool {
        self.ctx.len() == 1
    }

    pub fn set_locals(&mut self, locals: HashMap<Ident, usize>) {
        self.locals = locals
    }

    pub fn enter_scope(&mut self) {
        self.ctx.push(Context::new());
    }

    pub fn exit_scope(&mut self) {
        self.ctx.pop().unwrap();
    }

    pub fn insert_var(&mut self, ident: Ident, val: LoxType) {
        self.ctx.last_mut().unwrap().borrow_mut().set(ident, val);
    }

    pub fn find(&self, ident: &Ident) -> Option<LoxType> {
        if let Some(dist) = self.locals.get(ident) {
            self.find_at(ident, *dist)
        } else {
            // for ctx in self.ctx.iter().rev() {
            //     let result = ctx.borrow().get(ident).cloned();
            //     if result.is_some() {
            //         return result;
            //     }
            // }
            None
        }
    }

    pub fn find_method(&self, ident: &Ident) -> Option<LoxType> {
        for ctx in self.ctx.iter().rev() {
            let result = ctx.borrow().get(ident).cloned();
            if result.is_some() {
                return result;
            }
        }
        None
    }

    fn find_at(&self, ident: &Ident, dist: usize) -> Option<LoxType> {
        if let Some(ctx) = self.ctx.get(dist) {
            ctx.borrow().get(ident).cloned()
        } else {
            None
        }
    }

    pub fn update(&mut self, ident: &Ident, val: LoxType) -> Result<(), InterpreterError> {
        if let Some(dist) = self.locals.get(ident) {
            self.update_at(ident, val, *dist)
        } else {
            for ctx in self.ctx.iter().rev() {
                if let Some(res) = ctx.borrow_mut().get_mut(ident) {
                    *res = val;
                    return Ok(());
                }
            }

            Err(InterpreterError::Runtime(format!(
                "variable {ident} not found"
            )))
        }
    }

    fn update_at(
        &mut self,
        ident: &Ident,
        val: LoxType,
        dist: usize,
    ) -> Result<(), InterpreterError> {
        if let Some(ctx) = self.ctx.get(dist) {
            if let Some(res) = ctx.borrow_mut().get_mut(ident) {
                *res = val;
                return Ok(());
            }
        }
        Err(InterpreterError::Runtime(format!(
            "variable {ident} not found"
        )))
    }
    pub fn get_local(&self, ident: &Ident) -> Option<LoxType> {
        if let Some(dist) = self.locals.get(ident) {
            self.find_at(ident, *dist)
        } else {
            self.find(ident)
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
