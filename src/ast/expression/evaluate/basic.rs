use std::fmt::Display;

use crate::{
    ast::expression::{BinOp, Expr, Literal, UnaryOp},
    error::InterpreterError,
    program::Program,
};

impl Expr {
    pub fn evaluate(&self, program: &mut Program) -> EvaluateResult {
        // println!("evaluate expr: {self}");
        match self {
            Expr::Literal(literal) => match literal {
                Literal::Number(num) => Ok(EvaluateValue::Number(*num)),
                Literal::String(val) => Ok(EvaluateValue::String(val.to_string())),
                Literal::True => Ok(EvaluateValue::Boolean(true)),
                Literal::False => Ok(EvaluateValue::Boolean(false)),
                Literal::Nil => Ok(EvaluateValue::Nil),
            },
            Expr::Group(expr) => expr.evaluate(program),
            Expr::Unary(unary_op, expr) => match unary_op {
                UnaryOp::Bang => match expr.evaluate(program)? {
                    EvaluateValue::String(_) => Ok(EvaluateValue::Boolean(false)),
                    EvaluateValue::Number(_) => Ok(EvaluateValue::Boolean(false)),
                    EvaluateValue::Boolean(val) => Ok(EvaluateValue::Boolean(!val)),
                    EvaluateValue::Nil => Ok(EvaluateValue::Boolean(true)),
                    _ => Err(InterpreterError::Runtime(
                        "Incorrect expression type".to_string(),
                    )),
                },
                UnaryOp::Minus => match expr.evaluate(program)? {
                    EvaluateValue::Number(num) => Ok(EvaluateValue::Number(-num)),
                    _ => Err(InterpreterError::Runtime(
                        "Operand must be a number".to_string(),
                    )),
                },
            },
            Expr::Arithmetic(op, left, right) => {
                match (left.evaluate(program)?, right.evaluate(program)?) {
                    (EvaluateValue::Number(num_left), EvaluateValue::Number(num_right)) => match op
                    {
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
                }
            }
            Expr::Variable(ident) => match program.variables.get(ident) {
                Some(val) => Ok(val.clone()),
                None => Err(InterpreterError::Runtime(format!(
                    "Undefined variable '{ident}'"
                ))),
            },
            Expr::Assignment(ident, expr) => {
                let value = expr.evaluate(program)?;
                if let Some(val) = program.variables.get_mut(ident) {
                    *val = value.clone();
                } else {
                    let _ = program.variables.insert(ident.to_string(), value.clone());
                }
                Ok(value)
            }
            Expr::Print(expr) => {
                let value = expr.evaluate(program)?;
                println!("{value}");
                Ok(EvaluateValue::Empty)
            }
        }
    }
}

pub type EvaluateResult = Result<EvaluateValue, InterpreterError>;

#[derive(Debug, Clone, PartialEq)]
pub enum EvaluateValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Many(Vec<EvaluateValue>),
    Nil,
    Empty,
}

impl Display for EvaluateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(val) => write!(f, "{val}"),
            Self::Number(val) => write!(f, "{val}"),
            Self::Boolean(val) => write!(f, "{val}"),
            Self::Many(vals) => {
                let mut out = String::new();
                vals.iter()
                    .for_each(|expr| out.push_str(format!("{expr}").as_str()));
                write!(f, "{out}")
            }
            Self::Nil => write!(f, "nil"),
            Self::Empty => write!(f, ""),
        }
    }
}
