use crate::{
    ast::{ident::Ident, stmt::Block},
    error::InterpreterError,
    runtime::{environment::Environment, loxtype::LoxType},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Fun(Function),
}

impl Item {
    pub fn run(&self, env: &mut Environment) -> Result<(), InterpreterError> {
        match self {
            Item::Fun(function) => {
                env.insert_var(
                    function.sig.ident.clone(),
                    LoxType::Method(function.clone()),
                );
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
        env: &mut Environment,
        args: Vec<LoxType>,
    ) -> Result<Option<LoxType>, InterpreterError> {
        let closure = &env.capture_context();
        env.enter_closure(closure);
        args.iter()
            .zip(self.sig.inputs.iter())
            .for_each(|(arg, input)| {
                env.insert_var(input.clone(), arg.clone());
            });
        let res = self
            .body
            .run(env)
            .map(|val| val.map(|lox| lox.into_inner().clone()));
        env.exit_closure(closure);
        res
    }
}

#[derive(Debug, Clone)]
pub struct FunSig {
    pub ident: Ident,
    pub inputs: Vec<Ident>,
}

impl FunSig {
    pub fn method_call(ident: Ident, len: usize) -> Self {
        let mut inputs = vec![];
        for _ in 0..len {
            inputs.push(Ident("".to_string()));
        }
        Self { ident, inputs }
    }
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
