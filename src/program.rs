use hashbrown::HashMap;

use crate::{
    ast::{
        evaluate::EvaluateValue,
        parse::{token_stream::generate_token_stream, Parser},
        stmt::Stmt,
    },
    error::InterpreterError,
    tokens::{Lexer, Token},
};

#[derive(Debug, Clone)]
pub struct Program {
    pub variables: HashMap<String, EvaluateValue>,
    ast: Vec<Stmt>,
}

impl Program {
    pub fn new(filename: &str) -> Result<Self, InterpreterError> {
        let mut ast = vec![];
        let mut lexer = Lexer::new(filename)?;
        let stream = generate_token_stream(&mut lexer)?;
        let mut parser = Parser::new(stream);
        loop {
            if parser.current_token == Token::EOF {
                break;
            } else {
                ast.push(parser.parse_stmt()?);
            }
        }
        Ok(Program {
            variables: HashMap::default(),
            ast,
        })
    }

    pub fn empty() -> Self {
        Self {
            variables: HashMap::new(),
            ast: vec![],
        }
    }

    pub fn run(&mut self) -> Result<(), InterpreterError> {
        for stmt in self.ast.clone() {
            stmt.run(self)?;
        }
        Ok(())
    }
}
