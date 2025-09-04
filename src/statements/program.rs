use hashbrown::HashMap;

use crate::{
    context::Context,
    error::InterpreterError,
    expression::{evaluate::EvaluateValue, parse_tokens},
    statements::basic::print_stmt,
    tokens::{Lexer, ReservedWord, Token},
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
        let statements = self.lexer.get_statements();
        for stmt in statements {
            if !stmt.tokens().is_empty() {
                let mut ctx = Context::new(stmt);
                self.run_statement(&mut ctx)?;
            }
        }
        Ok(())
    }

    pub fn lexer(&self) -> Lexer {
        self.lexer.clone()
    }

    fn run_statement(&mut self, ctx: &mut Context) -> Result<(), InterpreterError> {
        let first = ctx.statement().peek_next();
        if let Token::Reserved(reserved) = first {
            match reserved {
                ReservedWord::Print => {
                    ctx.statement().next_token();
                    print_stmt(self, ctx)
                }
                ReservedWord::Var => {
                    parse_tokens(ctx, 0)?.evaluate(self)?;
                    Ok(())
                }
                _ => {
                    parse_tokens(ctx, 0)?;
                    Ok(())
                }
            }
        } else {
            parse_tokens(ctx, 0)?.evaluate(self)?;
            Ok(())
        }
    }
}
