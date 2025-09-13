use crate::{
    ast::{ident::Ident, parse::Parser, BinOp, Expr, Group, Literal, LogicOp, UnaryOp},
    error::InterpreterError,
    tokens::{ReservedWord, Token},
};

impl Parser {
    pub fn parse_group(&mut self) -> Result<Group, InterpreterError> {
        assert_eq!(self.current_token, Token::LeftParen);
        self.bump();
        let group = Group(self.parse_expr(0)?);
        let next = self.look_ahead(1);
        if next == Token::RightParen {
            // Bump past right paren
            self.bump();
            Ok(group)
        } else {
            Err(InterpreterError::Syntax("missing ')'".to_string()))
        }
    }

    pub fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, InterpreterError> {
        let current = self.current_token.clone();

        let mut lhs = if let Some(literal) = Literal::from_token(&current) {
            Expr::Literal(literal)
        } else if let Some(op) = UnaryOp::from_token(&current) {
            self.bump();
            Expr::Unary(op, Box::new(self.parse_expr(9)?))
        } else {
            match current {
                Token::LeftParen => Expr::Group(Box::new(self.parse_group()?)),
                Token::Reserved(reserved) => match reserved {
                    ReservedWord::Var => {
                        self.bump();
                        if let Some(ident) = Ident::from_token(&self.current_token) {
                            let next = self.look_ahead(1);
                            if next == Token::Equal {
                                self.bump();
                                self.bump();
                                Expr::InitVar(ident.clone(), Box::new(self.parse_expr(0)?))
                            } else {
                                Expr::InitVar(ident.clone(), Box::new(Expr::Literal(Literal::Nil)))
                            }
                        } else {
                            return Err(InterpreterError::Syntax(format!(
                                "invalid token: {}",
                                self.current_token.clone()
                            )));
                        }
                    }
                    ReservedWord::Print => {
                        self.bump();
                        Expr::Print(Box::new(self.parse_expr(0)?))
                    }
                    ReservedWord::Return => {
                        self.bump();
                        Expr::Return(Box::new(self.parse_expr(0)?))
                    }
                    _ => todo!(),
                },
                Token::Identifier(ident) => {
                    let ident = Ident(ident.to_string());
                    let next = self.look_ahead(1);
                    if next == Token::Equal {
                        self.bump();
                        self.bump();
                        Expr::UpdateVar(ident.clone(), Box::new(self.parse_expr(0)?))
                    } else if next == Token::LeftParen {
                        self.bump();
                        Expr::MethodCall(self.parse_fun_sig(ident)?)
                    } else {
                        Expr::Variable(ident)
                    }
                }
                _ => {
                    return Err(InterpreterError::Syntax(format!(
                        "invalid token: {}",
                        current.clone()
                    )));
                }
            }
        };

        loop {
            let next = self.look_ahead(1);

            if let Some(op) = LogicOp::from_token(&next) {
                self.bump();
                self.bump();
                let rhs = self.parse_expr(0)?;
                lhs = Expr::Conditional(op, Box::new(lhs), Box::new(rhs));
            } else if let Some(op) = BinOp::from_token(&next) {
                let (l_bp, r_bp) = op.infix_binding_power();
                if l_bp < min_bp {
                    break;
                }

                self.bump(); // Bump past op
                self.bump(); // Bump to update current token for parse

                let rhs = self.parse_expr(r_bp)?;
                lhs = Expr::Arithmetic(op, Box::new(lhs), Box::new(rhs));
            } else if matches!(next, Token::EOF | Token::RightParen | Token::Semicolon) {
                break;
            } else {
                return Err(InterpreterError::Syntax(format!("invalid token: {}", next)));
            }
        }
        Ok(lhs)
    }
}

trait InfixBindingPower {
    fn infix_binding_power(&self) -> (u8, u8);
}

impl InfixBindingPower for BinOp {
    fn infix_binding_power(&self) -> (u8, u8) {
        use BinOp::*;
        match self {
            Lt | Le | Gt | Ge | Eq | Ne => (1, 2),
            Add | Sub => (3, 4),
            Mul | Div => (5, 6),
        }
    }

    // fn create_expression(&self, left: Expr, right: Expr) -> Expr {
    //     Expr::Arithmetic(self.clone(), Box::new(left), Box::new(right))
    // }
}

impl InfixBindingPower for LogicOp {
    fn infix_binding_power(&self) -> (u8, u8) {
        (1, 2)
    }
    // fn create_expression(&self, left: Expr, right: Expr) -> Expr {
    //     Expr::Conditional(self.clone(), Box::new(left), Box::new(right))
    // }
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

impl ExprKind for LogicOp {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Reserved(ReservedWord::Or) => Some(Self::Or),
            Token::Reserved(ReservedWord::And) => Some(Self::And),
            _ => None,
        }
    }
}
