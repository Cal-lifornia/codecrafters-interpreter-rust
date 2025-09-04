use crate::{
    context::Context,
    error::InterpreterError,
    expression::{BinOp, Expr, Literal, UnaryOp},
    tokens::{ReservedWord, Token},
};

pub fn parse_tokens(ctx: &mut Context, min_bp: u8) -> Result<Expr, InterpreterError> {
    let first = ctx.statement().next_token();
    let mut lhs = match first {
        Token::LeftParen => {
            let lhs = Expr::Group(Box::new(parse_tokens(ctx, 0)?));
            if ctx.statement().next_token() != Token::RightParen {
                return Err(InterpreterError::Syntax("Error: Missing ')'".to_string()));
            }

            lhs
        }
        Token::Reserved(ReservedWord::Var) => {
            if let Token::Identifier(ident) = ctx.statement().next_token() {
                if ctx.statement().peek_next() == Token::Equal {
                    assert_eq!(ctx.statement().next_token(), Token::Equal);
                    Expr::Assignment(ident, Box::new(parse_tokens(ctx, 0)?))
                } else {
                    Expr::Assignment(ident, Box::new(Expr::Literal(Literal::Nil)))
                }
            } else {
                return Err(InterpreterError::Syntax(format!("invalid token: {first}")));
            }
        }
        Token::Identifier(ident) => {
            if ctx.statement().peek_next() == Token::Equal {
                assert_eq!(ctx.statement().next_token(), Token::Equal);
                Expr::Assignment(ident, Box::new(parse_tokens(ctx, 0)?))
            } else {
                Expr::Variable(ident)
            }
        }
        _ => {
            if let Some(literal) = Literal::from_token(&first) {
                Expr::Literal(literal)
            } else if first == Token::LeftParen {
                let lhs = Expr::Group(Box::new(parse_tokens(ctx, 0)?));
                if ctx.statement().next_token() != Token::RightParen {
                    return Err(InterpreterError::Syntax("Error: Missing ')'".to_string()));
                }

                lhs
            } else if let Some(op) = UnaryOp::from_token(&first) {
                Expr::Unary(op, Box::new(parse_tokens(ctx, 7)?))
            } else {
                return Err(InterpreterError::Syntax(format!("invalid token: {first}")));
            }
        }
    };

    loop {
        let next = ctx.statement().peek_next();
        let op = match next {
            Token::EOF => break,
            Token::RightParen => {
                break;
            }
            _ => {
                if let Some(op) = BinOp::from_token(&next) {
                    op
                } else {
                    return Err(InterpreterError::Syntax(format!("invalid token: {next}")));
                }
            }
        };

        let (l_bp, r_bp) = op.infix_binding_power();
        if l_bp < min_bp {
            break;
        }

        ctx.statement().next_token();
        let rhs = parse_tokens(ctx, r_bp)?;
        lhs = Expr::Arithmetic(op, Box::new(lhs), Box::new(rhs));
    }
    Ok(lhs)
}

trait BindingPower {
    fn infix_binding_power(&self) -> (u8, u8);
}

impl BindingPower for BinOp {
    fn infix_binding_power(&self) -> (u8, u8) {
        use BinOp::*;
        match self {
            Add | Sub => (3, 4),
            Mul | Div => (5, 6),
            Lt | Le | Gt | Ge | Eq | Ne => (1, 2),
        }
    }
}

trait ExprKind: Sized {
    fn from_token(token: &Token) -> Option<Self>;
}
impl ExprKind for BinOp {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Plus => Some(Self::Add),
            Token::Minus => Some(Self::Sub),
            Token::Star => Some(Self::Mul),
            Token::Slash => Some(Self::Div),
            Token::Less => Some(Self::Lt),
            Token::LessEqual => Some(Self::Le),
            Token::Greater => Some(Self::Gt),
            Token::GreaterEqual => Some(Self::Ge),
            Token::EqualEqual => Some(Self::Eq),
            Token::BangEqual => Some(Self::Ne),
            _ => None,
        }
    }
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
impl ExprKind for Literal {
    fn from_token(value: &Token) -> Option<Self> {
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
