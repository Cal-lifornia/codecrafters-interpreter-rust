use std::fmt::Display;

use crate::expression::{BinOp, Expr, Literal, UnaryOp};

impl Expr {
    pub fn evaluate(&self) -> EvaluateResult {
        match self {
            Expr::Literal(literal) => match literal {
                Literal::Number(num) => EvaluateResult::Number(*num),
                Literal::String(val) => EvaluateResult::String(val.to_string()),
                Literal::True => EvaluateResult::Boolean(true),
                Literal::False => EvaluateResult::Boolean(false),
                Literal::Nil => EvaluateResult::Nil,
            },
            Expr::Group(expr) => expr.evaluate(),
            Expr::Unary(unary_op, expr) => match unary_op {
                UnaryOp::Bang => match expr.evaluate() {
                    EvaluateResult::String(_) => EvaluateResult::Boolean(false),
                    EvaluateResult::Number(_) => EvaluateResult::Boolean(false),
                    EvaluateResult::Boolean(val) => EvaluateResult::Boolean(!val),
                    EvaluateResult::Nil => EvaluateResult::Boolean(true),
                },
                UnaryOp::Minus => match expr.evaluate() {
                    EvaluateResult::String(_) => EvaluateResult::Boolean(false),
                    EvaluateResult::Number(num) => EvaluateResult::Number(-num),
                    EvaluateResult::Boolean(_) => EvaluateResult::Boolean(false),
                    EvaluateResult::Nil => EvaluateResult::Nil,
                },
            },
            Expr::Arithmetic(op, left, right) => match (left.evaluate(), right.evaluate()) {
                (EvaluateResult::Number(num_left), EvaluateResult::Number(num_right)) => match op {
                    BinOp::Add => EvaluateResult::Number(num_left + num_right),
                    BinOp::Sub => EvaluateResult::Number(num_left - num_right),
                    BinOp::Mul => EvaluateResult::Number(num_left * num_right),
                    BinOp::Div => EvaluateResult::Number(num_left / num_right),
                    _ => todo!(),
                },
                _ => todo!(),
            },
        }
    }
}

pub enum EvaluateResult {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
impl Display for EvaluateResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(val) => write!(f, "{val}"),
            Self::Number(val) => write!(f, "{val}"),
            Self::Boolean(val) => write!(f, "{val}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}
