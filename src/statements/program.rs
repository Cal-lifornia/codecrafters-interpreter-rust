use crate::{
    error::InterpreterError,
    statements::print::print_stmt,
    tokens::{Lexer, ReservedWord, Token},
};

pub struct Program {
    pub lexer: Lexer,
}

pub type ProgramResult = Result<(), InterpreterError>;

impl Program {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn run(&mut self) -> Result<(), InterpreterError> {
        if let Token::Semicolon = self.lexer.pop_last() {
            let first = self.lexer.next_token();
            if let Token::Reserved(reserved) = first {
                match reserved {
                    ReservedWord::Print => print_stmt(self)?,
                    _ => todo!(),
                }
            }
            Ok(())
        } else {
            Err(InterpreterError::Runtime("missing semicolon".to_string()))
        }
    }
}
