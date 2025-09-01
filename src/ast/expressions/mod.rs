use std::{fmt::Display, io::Error};

use crate::tokens::{SymbolToken, Token};

mod literal;
pub use literal::*;
mod group;
pub use group::*;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Group(Box<Group>),
}

enum State {
    Literal,
    Group,
}

impl Expression {
    pub fn parse_tokens(input: &[Token]) -> std::io::Result<Self> {
        let mut temp_tokens: Vec<Token> = vec![];
        let mut current_state = State::Literal;
        let mut exprs: Vec<Expression> = vec![];

        for (idx, val) in input.iter().enumerate() {
            current_state = match current_state {
                State::Literal => match val {
                    Token::Symbol(SymbolToken::LeftParen) => State::Group,
                    _ => {
                        if let Some(literal) = Literal::from_token(val) {
                            return Ok(Self::Literal(literal));
                        } else {
                            temp_tokens.push(val.clone());
                            State::Literal
                        }
                    }
                },
                State::Group => match val {
                    Token::Symbol(SymbolToken::RightParen) => {
                        return Ok(Self::Group(Box::new(Group::new(exprs[0].clone()))));
                    }
                    _ => {
                        let expr = Expression::parse_tokens(&input[idx..])?;
                        exprs.push(expr);
                        State::Group
                    }
                },
            }
        }

        Ok(exprs[0].clone())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}",),
            Self::Group(group) => write!(f, "{group}"),
        }
    }
}
