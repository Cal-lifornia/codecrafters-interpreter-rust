use std::fmt::Display;

use hashbrown::HashMap;
use lox_ast::ast::{Class, Ident};

use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct ClassInstance {
    ident: Ident,
    pub properties: HashMap<Ident, Value>,
}

impl ClassInstance {
    pub fn new(class: Class) -> Self {
        Self {
            ident: class.ident,
            properties: HashMap::new(),
        }
    }
}

impl Display for ClassInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.ident)
    }
}
