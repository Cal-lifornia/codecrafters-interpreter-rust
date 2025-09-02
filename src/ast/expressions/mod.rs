use std::fmt::Display;

mod literal;
pub use literal::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Group(Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Arithmetic(BinOp, Box<Expr>, Box<Expr>),
}

// enum State {
//     Literal,
//     Group,
//     Arithmetic(SymbolToken),
//     Unary(SymbolToken),
// }

impl Expr {
    // pub fn parse_tokens(input: &[Token]) -> std::io::Result<Self> {
    //     let mut temp_tokens: Vec<Token> = vec![];
    //     let mut current_state = State::Literal;
    //     let mut exprs: Vec<Expression> = vec![];

    //     for (idx, val) in input.iter().enumerate() {
    //         current_state = match current_state {
    //             State::Literal => match val {
    //                 Token::Symbol(SymbolToken::LeftParen) => State::Group,
    //                 Token::Symbol(SymbolToken::Minus) => State::Unary(SymbolToken::Minus),

    //                 Token::Symbol(SymbolToken::Bang) => State::Unary(SymbolToken::Bang),

    //                 _ => {
    //                     if let Some(literal) = Literal::from_token(val) {
    //                         exprs.push(Self::Literal(literal));
    //                         match input[idx + 1] {
    //                             Token::Symbol(symbol) => {
    //                                 if matches!(symbol, SymbolToken::Slash | SymbolToken::Star) {
    //                                     State::Arithmetic(symbol)
    //                                 } else {
    //                                     break;
    //                                 }
    //                             }
    //                             _ => {
    //                                 break;
    //                             }
    //                         }
    //                     } else {
    //                         temp_tokens.push(val.clone());
    //                         continue;
    //                     }
    //                 }
    //             },
    //             State::Group => match val {
    //                 Token::Symbol(SymbolToken::RightParen) => {
    //                     let group = Self::Group(Box::new(Group::new(exprs.remove(0))));
    //                     exprs.push(group);
    //                     if matches!(
    //                         input[idx + 1],
    //                         Token::Symbol(SymbolToken::Star | SymbolToken::Slash)
    //                     ) {
    //                         State::Arithmetic
    //                     } else {
    //                         break;
    //                     }
    //                 }
    //                 _ => {
    //                     let expr = Expression::parse_tokens(&input[idx..])?;
    //                     exprs.push(expr);
    //                     State::Group
    //                 }
    //             },
    //             State::Arithmetic(symbol) => match val {
    //                 Token::Symbol(SymbolToken::Star) => {}
    //             },
    //         }
    //     }

    //     Ok(exprs[0].clone())
    // }
    //
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}",),
            Self::Group(expr) => write!(f, "({expr})"),
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
