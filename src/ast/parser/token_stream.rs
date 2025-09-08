use std::rc::Rc;

use crate::{
    error::InterpreterError,
    tokens::{Lexer, Token},
};

#[derive(Debug, Clone)]
pub struct TokenStream(Rc<Vec<TokenTree>>);

impl TokenStream {
    fn new(trees: Vec<TokenTree>) -> Self {
        Self(Rc::new(trees))
    }
    pub fn get(&self, idx: usize) -> Option<&TokenTree> {
        self.0.get(idx)
    }
}

#[derive(Debug, Clone)]
pub enum TokenTree {
    Token(Token),
    Delimited(Delimiter, TokenStream),
}

#[derive(Debug, Clone)]
pub enum Delimiter {
    Parenthesis,
    Brace,
}

pub fn create_token_tree(lexer: &mut Lexer) -> Result<TokenTree, InterpreterError> {
    let first = lexer.next_token();
    match first {
        Token::LeftParen => {
            let mut stream = vec![];
            let next = lexer.peek_next();
            loop {
                if next == Token::RightParen {
                    let _ = lexer.next_token();
                    break;
                } else if next == Token::EOF {
                    return Err(InterpreterError::Syntax("missing ')'".to_string()));
                } else {
                    stream.push(create_token_tree(lexer)?);
                }
            }
            Ok(TokenTree::Delimited(
                Delimiter::Parenthesis,
                TokenStream::new(stream),
            ))
        }
        Token::LeftBrace => {
            let mut stream = vec![];
            let next = lexer.peek_next();
            loop {
                if next == Token::RightBrace {
                    let _ = lexer.next_token();
                    break;
                } else if next == Token::EOF {
                    return Err(InterpreterError::Syntax("missing '}'".to_string()));
                } else {
                    stream.push(create_token_tree(lexer)?);
                }
            }
            Ok(TokenTree::Delimited(
                Delimiter::Brace,
                TokenStream::new(stream),
            ))
        }
        _ => Ok(TokenTree::Token(first)),
    }
}

#[derive(Debug, Clone)]
pub struct TokenCursor {
    pub curr: TokenTreeCursor,
    pub stack: Vec<TokenTreeCursor>,
}

#[derive(Debug, Clone)]
pub struct TokenTreeCursor {
    stream: TokenStream,
    index: usize,
}
