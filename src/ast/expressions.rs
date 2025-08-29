use std::fmt::Display;

use crate::tokens::{ReservedWord, Token};

pub enum Expression {
    Literal(Literal),
}

pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(val) => write!(f, "{val:?}"),
            Self::String(val) => write!(f, "{val}"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl Literal {
    pub fn from_token(value: &Token) -> Option<Self> {
        match value {
            Token::Reserved(reserved) => match reserved {
                ReservedWord::True => Some(Self::True),
                ReservedWord::False => Some(Self::False),
                ReservedWord::Nil => Some(Self::Nil),
                _ => None,
            },
            Token::StringLiteral(value) => Some(Self::String(value.to_string())),
            Token::Number(_, value) => Some(Self::Number(*value)),
            _ => None,
        }
    }
}

impl Expression {
    pub fn parse_tokens(tokens: &[Token]) -> Vec<Self> {
        let mut results = vec![];

        for token in tokens {
            if let Some(literal) = Literal::from_token(token) {
                results.push(Self::Literal(literal));
            }
        }
        results
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}",),
        }
    }
}
