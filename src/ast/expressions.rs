use std::fmt::Display;

use crate::tokens::{ReservedWord, Token, TokenDisplay};

pub enum Expression {
    Literal(Token),
}

impl Expression {
    pub fn parse_tokens(tokens: &[Token]) -> Vec<Self> {
        use Token::*;

        let mut results = vec![];

        for token in tokens {
            if let Reserved(reserved) = token {
                if matches!(
                    reserved,
                    ReservedWord::True | ReservedWord::False | ReservedWord::Nil
                ) {
                    results.push(Self::Literal(token.clone()));
                }
            }
        }
        results
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(token) => write!(f, "{}", token.lexeme()),
        }
    }
}
