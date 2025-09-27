mod expr;
mod item;
mod stmt;

use hashbrown::HashMap;

use crate::{
    ast::{ident::Ident, item::FunSig},
    error::InterpreterError,
    native::NativeFunction,
    runtime::{environment::Environment, loxtype::LoxType, resolver::Resolver},
};

#[derive(Default)]
pub struct Interpreter {
    pub resolver: Resolver,
    pub env: Environment,
    globals: HashMap<Ident, LoxType>,
    native_functions: HashMap<FunSig, Box<dyn NativeFunction>>,
}

impl Interpreter {
    pub fn get_native_fun(&self, sig: &FunSig) -> Option<&dyn NativeFunction> {
        self.native_functions.get(sig).map(|fun| fun.as_ref())
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

    pub fn insert_var(&mut self, ident: Ident, val: LoxType) {
        if self.env.global_scope() {
            self.globals.insert(ident, val);
        } else {
            self.env.insert_var(ident, val);
        }
    }

    pub fn find(&self, ident: &Ident) -> Option<LoxType> {
        let mut res = self.env.find(ident);
        if res.is_none() {
            res = self.globals.get(ident).cloned()
        }
        res
    }

    pub fn find_method(&self, ident: &Ident) -> Option<LoxType> {
        let mut res = self.env.find_method(ident);
        if res.is_none() {
            res = self.globals.get(ident).cloned()
        }
        res
    }
}
pub type EvaluateResult = Result<Option<LoxType>, InterpreterError>;
