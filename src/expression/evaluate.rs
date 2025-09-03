use std::fmt::Display;

use crate::{
    error::InterpreterError,
    expression::{BinOp, Expr, Literal, UnaryOp},
};

impl Expr {
    pub fn evaluate(&self) -> EvaluateResult {
        // println!("evaluate expr: {self}");
        match self {
            Expr::Literal(literal) => match literal {
                Literal::Number(num) => Ok(EvaluateValue::Number(*num)),
                Literal::String(val) => Ok(EvaluateValue::String(val.to_string())),
                Literal::True => Ok(EvaluateValue::Boolean(true)),
                Literal::False => Ok(EvaluateValue::Boolean(false)),
                Literal::Nil => Ok(EvaluateValue::Nil),
            },
            Expr::Group(expr) => expr.evaluate(),
            Expr::Unary(unary_op, expr) => match unary_op {
                UnaryOp::Bang => match expr.evaluate()? {
                    EvaluateValue::String(_) => Ok(EvaluateValue::Boolean(false)),
                    EvaluateValue::Number(_) => Ok(EvaluateValue::Boolean(false)),
                    EvaluateValue::Boolean(val) => Ok(EvaluateValue::Boolean(!val)),
                    EvaluateValue::Nil => Ok(EvaluateValue::Boolean(true)),
                },
                UnaryOp::Minus => match expr.evaluate()? {
                    EvaluateValue::Number(num) => Ok(EvaluateValue::Number(-num)),
                    _ => Err(InterpreterError::Runtime(
                        "Operand must be a number".to_string(),
                    )),
                },
            },
            Expr::Arithmetic(op, left, right) => match (left.evaluate()?, right.evaluate()?) {
                (EvaluateValue::Number(num_left), EvaluateValue::Number(num_right)) => match op {
                    BinOp::Add => Ok(EvaluateValue::Number(num_left + num_right)),
                    BinOp::Sub => Ok(EvaluateValue::Number(num_left - num_right)),
                    BinOp::Mul => Ok(EvaluateValue::Number(num_left * num_right)),
                    BinOp::Div => Ok(EvaluateValue::Number(num_left / num_right)),
                    BinOp::Lt => Ok(EvaluateValue::Boolean(num_left < num_right)),
                    BinOp::Le => Ok(EvaluateValue::Boolean(num_left <= num_right)),
                    BinOp::Gt => Ok(EvaluateValue::Boolean(num_left > num_right)),
                    BinOp::Ge => Ok(EvaluateValue::Boolean(num_left >= num_right)),
                    BinOp::Eq => Ok(EvaluateValue::Boolean(num_left == num_right)),
                    BinOp::Ne => Ok(EvaluateValue::Boolean(num_left != num_right)),
                },
                (EvaluateValue::String(string_left), EvaluateValue::String(string_right)) => {
                    match op {
                        BinOp::Add => Ok(EvaluateValue::String(format!(
                            "{string_left}{string_right}"
                        ))),
                        BinOp::Eq => Ok(EvaluateValue::Boolean(string_left == string_right)),
                        BinOp::Ne => Ok(EvaluateValue::Boolean(string_left != string_right)),
                        _ => Err(InterpreterError::Runtime(
                            "Operand must be a number".to_string(),
                        )),
                    }
                }
                (left_val, right_val) => match op {
                    BinOp::Eq => Ok(EvaluateValue::Boolean(left_val == right_val)),
                    BinOp::Ne => Ok(EvaluateValue::Boolean(left_val != right_val)),
                    _ => Err(InterpreterError::Runtime(
                        "Operand must be a number".to_string(),
                    )),
                },
            },
        }
    }
}

pub type EvaluateResult = Result<EvaluateValue, InterpreterError>;

#[derive(Debug, Clone, PartialEq)]
pub enum EvaluateValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Display for EvaluateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(val) => write!(f, "{val}"),
            Self::Number(val) => write!(f, "{val}"),
            Self::Boolean(val) => write!(f, "{val}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}
