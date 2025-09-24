use crate::{
    ast::{ident::Ident, stmt::Block},
    runtime::environment::Ctx,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Fun(Function),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub sig: FunSig,
    pub body: Block,
    pub closure: Ctx,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.sig == other.sig && self.body == other.body
    }
}

impl Function {
    pub fn param_len(&self) -> usize {
        self.sig.inputs.len()
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
