use std::fmt::Display;

use crate::ast::Expression;

#[derive(Debug, Clone)]
pub enum Unary {
    Bang(Expression),
    Minus(Expression),
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unary::Bang(expr) => write!(f, "(! {expr})"),
            Unary::Minus(expr) => write!(f, "(- {expr})"),
        }
    }
}
