use std::fmt::Display;

use crate::tokens::{SymbolToken, Token};

mod literal;
pub use literal::*;
mod group;
pub use group::*;
mod unary;
pub use unary::*;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Group(Box<Group>),
    Unary(Box<Unary>),
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
                    Token::Symbol(SymbolToken::Minus) => {
                        let expr = Expression::parse_tokens(&input[idx + 1..])?;
                        return Ok(Self::Unary(Box::new(Unary::Minus(expr))));
                    }
                    Token::Symbol(SymbolToken::Bang) => {
                        let expr = Expression::parse_tokens(&input[idx + 1..])?;
                        return Ok(Self::Unary(Box::new(Unary::Bang(expr))));
                    }
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

        Err(std::io::Error::other("unreachable"))
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}",),
            Self::Group(group) => write!(f, "{group}"),
            Self::Unary(unary) => write!(f, "{unary}"),
        }
    }
}
