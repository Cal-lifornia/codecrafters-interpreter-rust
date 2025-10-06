use lox_shared::error::LoxError;

use crate::{
    ast::{Attribute, BinOp, Expr, ExprKind, Group, Ident, Literal, LogicOp, UnaryOp},
    parser::{
        Parser, syntax_error,
        token::{ReservedWord, Token, TokenKind},
    },
};

impl Parser {
    pub fn parse_group(&mut self) -> Result<Group, LoxError> {
        assert_eq!(self.current_token, TokenKind::LeftParen);
        self.bump();
        let group = Group(self.parse_expr(0)?);
        let next = self.look_ahead(1);
        if next == TokenKind::RightParen {
            // Bump past right paren
            self.bump();
            Ok(group)
        } else {
            Err(LoxError::Syntax("missing ')'".into()))
        }
    }

    pub fn parse_method_call(&mut self, expr: Expr) -> Result<Expr, LoxError> {
        assert_eq!(self.current_token, TokenKind::LeftParen);
        self.bump();

        // #[NOTE] method.spellcaster()() == (method.spellcaster())()

        let mut inputs: Vec<Expr> = vec![];

        loop {
            match self.current_token.kind() {
                TokenKind::Comma => {
                    self.bump();
                    // tracing::debug!("{}", self.current_token);
                    if matches!(self.current_token.kind(), TokenKind::RightParen) {
                        return Err(syntax_error(
                            &self.generate_attr(),
                            "Comma must be followed up with a function param",
                        ));
                    }
                }
                TokenKind::RightParen => {
                    let call_expr = Expr::new(
                        ExprKind::FunctionCall(Box::new(expr.clone()), inputs),
                        self.generate_attr(),
                    );
                    if self.look_ahead(1) == TokenKind::LeftParen {
                        self.bump();
                        return self.parse_method_call(call_expr);
                    } else {
                        return Ok(call_expr);
                    }
                }
                _ => {
                    inputs.push(self.parse_expr(0)?);
                    self.bump();
                }
            }
        }

        // let out: Expr = loop {
        //     if self.current_token == TokenKind::RightParen {
        //         if self.look_ahead(1) == TokenKind::LeftParen {
        //             self.bump();
        //             let sub_expr = self.parse_method_call(expr)?;
        //             return Ok(sub_expr);
        //         } else {
        //             break expr;
        //         }
        //     }
        //     inputs.push(self.parse_expr(0)?);
        //     self.bump();
        //     if self.current_token == TokenKind::Comma {
        //         self.bump();
        //         continue;
        //     } else {
        //         let next = self.look_ahead(1);
        //         if next == TokenKind::LeftParen {
        //             self.bump();
        //             if let Some(last) = inputs.pop()
        //                 && matches!(last.kind(), ExprKind::Variable(_))
        //             {
        //                 inputs.push(self.parse_method_call(last)?);
        //             } else {
        //                 return Err(LoxError::Syntax("Invalid expr".into()));
        //             }
        //         }
        //     }
        // };
        // assert_eq!(self.current_token, TokenKind::RightParen);
        // Ok(Expr::new(
        //     ExprKind::FunctionCall(Box::new(out.clone()), inputs),
        //     out.attr().clone(),
        // ))
    }

    pub fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, LoxError> {
        let current: Token = self.current_token.clone();
        let span = current.span().clone();
        let attr = Attribute::new(self.new_node_id(), span);

        let mut lhs = if let Some(literal) = Literal::from_token(&current) {
            Expr::new(ExprKind::Literal(literal), attr)
        } else if let Some(op) = UnaryOp::from_token(&current) {
            self.bump();
            Expr::new(
                ExprKind::Unary(
                    op,
                    Box::new(self.parse_expr(BindingOp::unary_binding_power())?),
                ),
                attr,
            )
        } else {
            match current.kind() {
                TokenKind::LeftParen => {
                    Expr::new(ExprKind::Group(Box::new(self.parse_group()?)), attr)
                }
                TokenKind::Reserved(reserved) => match reserved {
                    ReservedWord::Var => {
                        self.bump();
                        if let Some(ident) = Ident::from_token(&self.current_token) {
                            let next = self.look_ahead(1);
                            if next == TokenKind::Equal {
                                self.bump();
                                self.bump();
                                Expr::new(
                                    ExprKind::InitVar(ident.clone(), Box::new(self.parse_expr(0)?)),
                                    attr,
                                )
                            } else {
                                Expr::new(
                                    ExprKind::InitVar(
                                        ident.clone(),
                                        Box::new(Expr::new(
                                            ExprKind::Literal(Literal::Nil),
                                            attr.clone(),
                                        )),
                                    ),
                                    attr,
                                )
                            }
                        } else {
                            return Err(LoxError::Syntax(format!(
                                "invalid token reserved: {}",
                                self.current_token.clone()
                            )));
                        }
                    }
                    ReservedWord::Print => {
                        self.bump();
                        Expr::new(ExprKind::Print(Box::new(self.parse_expr(0)?)), attr)
                    }
                    ReservedWord::Return => {
                        let next = self.look_ahead(1);
                        if next == TokenKind::Semicolon {
                            Expr::new(ExprKind::Return(None), attr)
                        } else {
                            self.bump();
                            Expr::new(ExprKind::Return(Some(Box::new(self.parse_expr(0)?))), attr)
                        }
                    }
                    ReservedWord::This => Expr::new(
                        ExprKind::Variable(Ident(std::borrow::Cow::Borrowed("this"))),
                        attr,
                    ),
                    _ => todo!(),
                },
                TokenKind::Identifier(ident) => {
                    let ident = Ident(ident.clone());
                    let next = self.look_ahead(1);
                    match next.kind() {
                        TokenKind::Equal => {
                            self.bump();
                            self.bump();
                            Expr::new(
                                ExprKind::UpdateVar(ident.clone(), Box::new(self.parse_expr(0)?)),
                                attr,
                            )
                        }
                        TokenKind::LeftParen => {
                            self.bump();
                            self.parse_method_call(Expr::new(ExprKind::Variable(ident), attr))?
                        }
                        _ => Expr::new(ExprKind::Variable(ident), attr),
                    }
                }
                _ => {
                    return Err(syntax_error(
                        &self.generate_attr(),
                        format!("invalid token: {}", current.clone()),
                    ));
                }
            }
        };

        loop {
            let next = self.look_ahead(1);
            if let Some(op) = BindingOp::from_token(&next) {
                let (l_bp, r_bp) = op.infix_binding_power();
                if l_bp < min_bp {
                    break;
                }

                self.bump(); // Bump past op
                self.bump(); // Bump to update current token for parse

                lhs = op.create_expression(self, lhs, r_bp)?;
            } else if matches!(
                next.kind(),
                TokenKind::EOF | TokenKind::RightParen | TokenKind::Semicolon | TokenKind::Comma
            ) {
                break;
            } else {
                return Err(LoxError::Runtime(format!(
                    "{}: invalid token: {}",
                    lhs.attr().as_display(),
                    next,
                )));
            }
        }
        Ok(lhs)
    }
}

pub enum BindingOp {
    Bin(BinOp),
    Logic(LogicOp),
    Dot,
}

impl BindingOp {
    fn infix_binding_power(&self) -> (u8, u8) {
        use BinOp::*;
        match self {
            BindingOp::Logic(LogicOp::Or | LogicOp::And) => (1, 2),
            BindingOp::Bin(Lt | Le | Gt | Ge | Eq | Ne) => (3, 4),
            BindingOp::Bin(Add | Sub) => (5, 6),
            BindingOp::Bin(Mul | Div) => (7, 8),
            BindingOp::Dot => (10, 9),
        }
    }
    fn create_expression(
        &self,
        parser: &mut Parser,
        left: Expr,
        min_bp: u8,
    ) -> Result<Expr, LoxError> {
        let attr = Attribute::new(parser.new_node_id(), left.attr().span().clone());
        match self {
            BindingOp::Bin(op) => Ok(Expr::new(
                ExprKind::Arithmetic(
                    op.clone(),
                    Box::new(left),
                    Box::new(parser.parse_expr(min_bp)?),
                ),
                attr,
            )),
            BindingOp::Logic(op) => Ok(Expr::new(
                ExprKind::Conditional(
                    op.clone(),
                    Box::new(left),
                    Box::new(parser.parse_expr(min_bp)?),
                ),
                attr,
            )),
            BindingOp::Dot => parser.parse_class_call(left),
        }
    }
    fn unary_binding_power() -> u8 {
        11
    }
}

impl ExprOp for BindingOp {
    fn from_token(token: &Token) -> Option<Self> {
        if token == &TokenKind::Dot {
            return Some(Self::Dot);
        }
        let mut res = BinOp::from_token(token).map(Self::Bin);
        if res.is_none() {
            res = LogicOp::from_token(token).map(Self::Logic);
        }
        res
    }
}

trait ExprOp: Sized {
    fn from_token(token: &Token) -> Option<Self>;
}
impl ExprOp for BinOp {
    fn from_token(token: &Token) -> Option<Self> {
        match token.kind() {
            TokenKind::Plus => Some(Self::Add),
            TokenKind::Minus => Some(Self::Sub),
            TokenKind::Star => Some(Self::Mul),
            TokenKind::Slash => Some(Self::Div),
            TokenKind::Less => Some(Self::Lt),
            TokenKind::LessEqual => Some(Self::Le),
            TokenKind::Greater => Some(Self::Gt),
            TokenKind::GreaterEqual => Some(Self::Ge),
            TokenKind::EqualEqual => Some(Self::Eq),
            TokenKind::BangEqual => Some(Self::Ne),
            _ => None,
        }
    }
}
impl ExprOp for UnaryOp {
    fn from_token(token: &Token) -> Option<Self> {
        match token.kind() {
            TokenKind::Bang => Some(Self::Bang),
            TokenKind::Minus => Some(Self::Minus),
            _ => None,
        }
    }
}

impl ExprOp for Literal {
    fn from_token(value: &Token) -> Option<Self> {
        match value.kind() {
            TokenKind::Reserved(reserved) => match reserved {
                ReservedWord::True => Some(Self::True),
                ReservedWord::False => Some(Self::False),
                ReservedWord::Nil => Some(Self::Nil),
                _ => None,
            },
            TokenKind::StringLiteral(value) => Some(Self::String(value.to_string())),
            TokenKind::Number(_, value) => Some(Self::Number(*value)),
            _ => None,
        }
    }
}

impl ExprOp for LogicOp {
    fn from_token(token: &Token) -> Option<Self> {
        match token.kind() {
            TokenKind::Reserved(ReservedWord::Or) => Some(Self::Or),
            TokenKind::Reserved(ReservedWord::And) => Some(Self::And),
            _ => None,
        }
    }
}
