use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Group(Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Arithmetic(BinOp, Box<Expr>, Box<Expr>),
    Variable(String),
    Assignment(String, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}",),
            Self::Group(expr) => write!(f, "(group {expr})"),
            Self::Unary(op, expr) => write!(f, "({op} {expr})"),
            Self::Arithmetic(op, left, right) => write!(f, "({op} {left} {right})"),
            Self::Variable(ident) => write!(f, "{ident}"),
            Self::Assignment(ident, expr) => write!(f, "{ident} equals {expr}"),
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
