use crate::{
    ast::{evaluate::EvaluateValue, ident::Ident, stmt::Block},
    error::InterpreterError,
    runtime::program::Runtime,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Fun(Function),
}

impl Item {
    pub fn run(&self, runtime: &mut Runtime) -> Result<(), InterpreterError> {
        match self {
            Item::Fun(function) => {
                runtime.insert_unique_function(function.sig.clone(), function.clone());
                runtime
                    .scope
                    .insert_function_var(function.sig.ident.clone());
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub sig: FunSig,
    pub body: Block,
}

impl Function {
    pub fn run(
        &self,
        runtime: &mut Runtime,
        args: Vec<Ident>,
    ) -> Result<EvaluateValue, InterpreterError> {
        let mut method = self.clone();
        method.sig.inputs = args;
        method.body.run(runtime)
    }
}

#[derive(Debug, Clone)]
pub struct FunSig {
    pub ident: Ident,
    pub inputs: Vec<Ident>,
}

impl std::hash::Hash for FunSig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
        self.inputs.len().hash(state);
    }
}

impl Eq for FunSig {}

impl PartialEq for FunSig {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident && self.inputs.len() == other.inputs.len()
    }
}
