use std::fmt::Display;

use crate::ast::{Attribute, NodeId, ident::Ident};

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    attr: Attribute,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Expr {
    pub fn new(kind: ExprKind, attr: Attribute) -> Expr {
        Self { kind, attr }
    }

    pub fn kind(&self) -> &ExprKind {
        &self.kind
    }

    pub fn attr(&self) -> &Attribute {
        &self.attr
    }
    pub fn id(&self) -> &NodeId {
        self.attr.id()
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl PartialEq<ExprKind> for Expr {
    fn eq(&self, other: &ExprKind) -> bool {
        self.kind() == other
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Literal(Literal),
    Group(Box<Group>),
    Unary(UnaryOp, Box<Expr>),
    Arithmetic(BinOp, Box<Expr>, Box<Expr>),
    Conditional(LogicOp, Box<Expr>, Box<Expr>),
    Variable(Ident),
    InitVar(Ident, Box<Expr>),
    UpdateVar(Ident, Box<Expr>),
    Print(Box<Expr>),
    FunctionCall(Box<Expr>, Vec<Expr>),
    Get(Box<Expr>, Box<Expr>),
    Set(Box<Expr>, Ident, Box<Expr>),
    Return(Option<Box<Expr>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group(pub Expr);

impl Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}",),
            Self::Group(group) => write!(f, "(group {})", group.0),
            Self::Unary(op, expr) => write!(f, "({op} {expr})"),
            Self::Arithmetic(op, left, right) => write!(f, "({op} {left} {right})"),
            Self::Conditional(op, left, right) => write!(f, "({op} {left} {right})"),
            Self::Variable(ident) => write!(f, "{ident}"),
            Self::InitVar(ident, expr) => write!(f, "{ident} equals {expr}"),
            Self::UpdateVar(ident, expr) => write!(f, "updating {ident} to {expr}"),
            Self::Print(expr) => write!(f, "printing {expr}"),
            Self::FunctionCall(ident, args) => {
                write!(f, "calling function '{ident}' with args: {args:?}")
            }
            Self::Get(inst, prop) => write!(f, "(get class: {inst}; property: {prop})"),
            Self::Set(inst, prop, expr) => {
                write!(f, "(set class: {inst}; property: {prop} val: {expr})")
            }
            Self::Return(opt) => match opt {
                Some(expr) => write!(f, "(return {expr})"),
                None => write!(f, "blank return"),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicOp {
    Or,
    And,
}

impl Display for LogicOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicOp::Or => write!(f, "or"),
            LogicOp::And => write!(f, "and"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BinOp::*;
        let symbol = match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Lt => "<",
            Le => "<=",
            Gt => ">",
            Ge => ">=",
            Eq => "==",
            Ne => "!=",
        };
        write!(f, "{symbol}")
    }
}

#[derive(Debug, Clone, PartialEq)]
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
