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

    pub fn run(&self) -> Result<(), InterpreterError> {
        let mut statements = self.lexer.get_statements();
        // println!("statements {statements:#?}");
        for stmt in statements.iter_mut() {
            if !stmt.tokens().is_empty() {
                let first = stmt.next_token();
                // println!("first {first}");
                if let Token::Reserved(reserved) = first {
                    match reserved {
                        ReservedWord::Print => print_stmt(stmt)?,
                        _ => todo!(),
                    }
                }
            }
        }
        Ok(())
    }
}
