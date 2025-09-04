use crate::expression::Expr;

#[derive(Debug, Clone)]
pub struct Variable {
    ident: String,
    value: Expr,
}

impl Variable {
    pub fn new(ident: String, value: Expr) -> Self {
        Self { ident, value }
    }
    pub fn new_nil(ident: String) -> Self {
        Self {
            ident,
            value: Expr::Literal(crate::expression::Literal::Nil),
        }
    }

    pub fn set_value(&mut self, value: Expr) {
        self.value = value;
    }
    pub fn ident(&self) -> &str {
        &self.ident
    }
    pub fn value(&self) -> &Expr {
        &self.value
    }
}
