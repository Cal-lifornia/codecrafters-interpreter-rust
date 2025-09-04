use crate::{error::InterpreterError, tokens::Token};

#[derive(Debug, Clone)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(file: &str) -> Result<Self, InterpreterError> {
        let file_contents = std::fs::read_to_string(file)
            .map_err(|err| InterpreterError::Syntax(err.to_string()))?;

        let (mut tokens, errs) = Token::tokenize(&file_contents);
        if !errs.is_empty() {
            return Err(errs[0].clone());
        }
        tokens.reverse();

        Ok(Self { tokens })
    }

    pub fn next_token(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::EOF)
    }
    pub fn peek_next(&mut self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::EOF)
    }
    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
    pub fn last_token(&self) -> &Token {
        self.tokens.first().unwrap_or(&Token::EOF)
    }
    pub fn pop_last(&mut self) -> Token {
        if !self.tokens.is_empty() {
            self.tokens.remove(0)
        } else {
            Token::EOF
        }
    }

    pub fn get_statements(&self) -> Vec<Self> {
        self.tokens
            .rsplit(|token| token == &Token::Semicolon)
            .map(|tokens| Self {
                tokens: tokens.to_vec(),
            })
            .collect::<Vec<Self>>()
    }
}
