use std::collections::HashMap;

use crate::{
    error::InterpreterError,
    expression::{parse_tokens, Expr},
    statements::basic::print_stmt,
    tokens::{Lexer, ReservedWord, Token},
};

pub struct Program {
    pub variables: HashMap<String, Expr>,
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

    pub fn lexer(&self) -> &Lexer {
        &self.lexer
    }

    pub fn lexer_mut(&mut self) -> &mut Lexer {
        &mut self.lexer
    }

    pub fn run(&mut self) -> Result<(), InterpreterError> {
        let mut statements = self.lexer.get_statements();
        for stmt in statements.iter_mut() {
            if !stmt.tokens().is_empty() {
                self.run_statement(stmt)?;
            }
        }
        Ok(())
    }
    fn run_statement(&mut self, lexer: &mut Lexer) -> Result<(), InterpreterError> {
        let first = lexer.peek_next();
        if let Token::Reserved(ReservedWord::Print) = first {
            print_stmt(lexer)
        } else {
            parse_tokens(lexer, 0)?.evaluate()?;
            Ok(())
        }
    }
}
