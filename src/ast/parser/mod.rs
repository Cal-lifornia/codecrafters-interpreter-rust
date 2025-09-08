mod token_stream;

use crate::{ast::parser::token_stream::TokenCursor, tokens::Token};

pub struct Parser {
    current_token: Token,
    next_token: Token,
    prev_token: Token,
    cursor: TokenCursor,
}
