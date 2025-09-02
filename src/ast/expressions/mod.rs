use std::{
    fmt::Display,
    io::{Error, ErrorKind},
};

mod literal;
pub use literal::*;

use crate::tokens::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Group(Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Arithmetic(BinOp, Box<Expr>, Box<Expr>),
}

trait ExprKind: Sized {
    fn from_token(token: &Token) -> Option<Self>;
}

trait BindingPower {
    fn infix_binding_power(&self) -> (u8, u8);
}
pub fn parse_tokens(lexer: &mut Lexer, min_bp: u8) -> std::io::Result<Expr> {
    let first = lexer.next_token();
    let mut lhs = if let Some(literal) = Literal::from_token(&first) {
        Expr::Literal(literal)
    } else if first == Token::LeftParen {
        Expr::Group(Box::new(parse_tokens(lexer, 0)?))
    } else if let Some(op) = UnaryOp::from_token(&first) {
        Expr::Unary(op, Box::new(parse_tokens(lexer, 5)?))
    } else {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("invalid token: {first}"),
        ));
    };

    loop {
        let next = lexer.peek_next();
        let op = match next {
            Token::EOF => break,
            Token::RightParen => {
                lexer.next_token();
                break;
            }
            _ => {
                if let Some(op) = BinOp::from_token(&next) {
                    op
                } else {
                    return Err(std::io::Error::new(
                        ErrorKind::InvalidInput,
                        "invalid token",
                    ));
                }
            }
        };

        let (l_pb, r_pb) = op.infix_binding_power();
        if l_pb < min_bp {
            break;
        }

        lexer.next_token();
        let rhs = parse_tokens(lexer, r_pb)?;
        lhs = Expr::Arithmetic(op, Box::new(lhs), Box::new(rhs));
    }
    Ok(lhs)
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}",),
            Self::Group(expr) => write!(f, "(group {expr})"),
            Self::Unary(op, expr) => write!(f, "({op} {expr})"),
            Self::Arithmetic(op, left, right) => write!(f, "({op} {left} {right})"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Bang,
    Minus,
}

impl ExprKind for UnaryOp {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Bang => Some(Self::Bang),
            Token::Minus => Some(Self::Minus),
            _ => None,
        }
    }
}

impl BindingPower for UnaryOp {
    fn infix_binding_power(&self) -> (u8, u8) {
        (0, 5)
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Bang => write!(f, "!"),
            UnaryOp::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BindingPower for BinOp {
    fn infix_binding_power(&self) -> (u8, u8) {
        use BinOp::*;
        match self {
            Add | Sub => (1, 2),
            Mul | Div => (3, 4),
        }
    }
}

impl ExprKind for BinOp {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Plus => Some(Self::Add),
            Token::Minus => Some(Self::Sub),
            Token::Star => Some(Self::Mul),
            Token::Slash => Some(Self::Div),
            _ => None,
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
        };
        write!(f, "{symbol}")
    }
}
