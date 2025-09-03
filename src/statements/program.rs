use crate::{
    error::InterpreterError,
    expression::parse_tokens,
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
        for stmt in statements.iter_mut() {
            if !stmt.tokens().is_empty() {
                run_statement(stmt)?;
            }
        }
        Ok(())
    }
}

fn run_statement(lexer: &mut Lexer) -> Result<(), InterpreterError> {
    let first = lexer.peek_next();
    if let Token::Reserved(ReservedWord::Print) = first {
        print_stmt(lexer)
    } else {
        parse_tokens(lexer, 0)?.evaluate()?;
        Ok(())
    }
}
