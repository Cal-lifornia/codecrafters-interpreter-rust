use hashbrown::HashMap;

use crate::{ast::ident::Ident, runtime::environment::Environment};

#[derive(Default)]
pub struct Compiler {
    pub locals: HashMap<Ident, usize>,
    pub env: Environment,
}
