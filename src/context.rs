use crate::tokens::Lexer;

pub struct Context {
    statement: Lexer,
}

impl Context {
    pub fn new(lexer: Lexer) -> Self {
        Self { statement: lexer }
    }
    pub fn statement(&mut self) -> &mut Lexer {
        &mut self.statement
    }
}
