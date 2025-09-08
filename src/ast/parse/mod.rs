pub mod expr;
pub mod token_stream;

use crate::{
    ast::parse::token_stream::{TokenCursor, TokenStream, TokenTreeCursor},
    tokens::Token,
};

pub struct Parser {
    pub current_token: Token,
    pub prev_token: Token,
    cursor: TokenCursor,
}

impl Parser {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            current_token: Token::Dot,
            prev_token: Token::Dot,
            cursor: TokenCursor {
                curr: TokenTreeCursor::new(stream),
                stack: vec![],
            },
        }
    }

    pub fn bump(&mut self) {
        let next = self.cursor.next_token();

        self.prev_token = std::mem::replace(&mut self.current_token, next);
    }
}
