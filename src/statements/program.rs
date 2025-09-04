use hashbrown::HashMap;

use crate::{
    context::Context,
    error::InterpreterError,
    expression::{evaluate::EvaluateValue, parse_tokens},
    tokens::Lexer,
};

pub struct Program {
    pub variables: HashMap<String, EvaluateValue>,
    lexer: Lexer,
}

pub type ProgramResult = Result<(), InterpreterError>;

impl Program {
    pub fn new(filename: &str) -> Result<Self, InterpreterError> {
        let lexer = Lexer::new(filename)?;
        Ok(Self {
            lexer,
            variables: HashMap::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), InterpreterError> {
        let expr = parse_tokens(&mut Context::new(self.lexer()), 0)?;
        expr.evaluate(self)?;
        Ok(())
    }

    pub fn lexer(&self) -> Lexer {
        self.lexer.clone()
    }
}
