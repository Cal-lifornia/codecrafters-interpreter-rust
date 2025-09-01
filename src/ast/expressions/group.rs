use std::fmt::Display;

use crate::ast::Expression;

#[derive(Debug, Clone)]
pub struct Group {
    expr: Expression,
}

impl Group {
    pub fn new(expr: Expression) -> Self {
        Self { expr }
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(group {})", self.expr)
    }
}
