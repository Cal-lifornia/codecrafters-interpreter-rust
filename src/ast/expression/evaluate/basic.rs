use std::fmt::Display;

use crate::{
    ast::{
        expression::{BinOp, Expr, Literal, UnaryOp},
        LogicOp,
    },
    error::InterpreterError,
    runtime::program::Runtime,
};

impl Expr {
    pub fn evaluate(&self, runtime: &mut Runtime) -> EvaluateResult {
        match self {
            Expr::Literal(literal) => match literal {
                Literal::Number(num) => Ok(EvaluateValue::Number(*num)),
                Literal::String(val) => Ok(EvaluateValue::String(val.to_string())),
                Literal::True => Ok(EvaluateValue::Boolean(true)),
                Literal::False => Ok(EvaluateValue::Boolean(false)),
                Literal::Nil => Ok(EvaluateValue::Nil),
            },
            Expr::Group(group) => group.0.evaluate(runtime),
            Expr::Unary(unary_op, expr) => match unary_op {
                UnaryOp::Bang => match expr.evaluate(runtime)? {
                    EvaluateValue::String(_) => Ok(EvaluateValue::Boolean(false)),
                    EvaluateValue::Number(_) => Ok(EvaluateValue::Boolean(false)),
                    EvaluateValue::Boolean(val) => Ok(EvaluateValue::Boolean(!val)),
                    EvaluateValue::Nil => Ok(EvaluateValue::Boolean(true)),
                    _ => Err(InterpreterError::Runtime(
                        "Incorrect expression type".to_string(),
                    )),
                },
                UnaryOp::Minus => match expr.evaluate(runtime)? {
                    EvaluateValue::Number(num) => Ok(EvaluateValue::Number(-num)),
                    _ => Err(InterpreterError::Runtime(
                        "Operand must be a number".to_string(),
                    )),
                },
            },
            Expr::Arithmetic(op, left, right) => {
                match (left.evaluate(runtime)?, right.evaluate(runtime)?) {
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
            Expr::Conditional(op, left, right) => match op {
                LogicOp::Or => {
                    let left_val = left.evaluate(runtime)?;
                    if left_val.is_truthy() {
                        return Ok(left_val);
                    }
                    let right_val = right.evaluate(runtime)?;
                    if right_val.is_truthy() {
                        return Ok(right_val);
                    }
                    Ok(EvaluateValue::Boolean(false))
                }

                LogicOp::And => {
                    let left_val = left.evaluate(runtime)?;
                    if !left_val.is_truthy() {
                        return Ok(EvaluateValue::Boolean(false));
                    }
                    let right_val = right.evaluate(runtime)?;
                    if right_val.is_truthy() {
                        return Ok(right_val);
                    }
                    Ok(EvaluateValue::Boolean(false))
                }
            },
            Expr::Variable(ident) => match runtime.scope.find_var(ident) {
                Some(val) => Ok(val.clone()),
                None => Err(InterpreterError::Runtime(format!(
                    "Undefined variable '{ident}'"
                ))),
            },
            Expr::InitVar(ident, expr) => {
                let value = expr.evaluate(runtime)?;
                runtime.scope.insert_var(ident.clone(), value.clone());
                Ok(value)
            }
            Expr::UpdateVar(ident, expr) => {
                let value = expr.evaluate(runtime)?;
                // println!("updating {ident} to {value}");
                runtime.scope.update_var(ident, value.clone())?;
                Ok(value)
            }
            Expr::Print(expr) => {
                let value = expr.evaluate(runtime)?;
                println!("{value}");
                Ok(EvaluateValue::Empty)
            }
            Expr::MethodCall(fun_sig) => {
                if let Some(fun) = runtime.get_function(fun_sig) {
                    fun.run(runtime, fun_sig.inputs.clone())
                } else if let Some(fun) = runtime.get_native_fun(fun_sig) {
                    fun.run()
                } else {
                    Err(InterpreterError::Runtime(format!(
                        "cannot find method with name {}",
                        fun_sig.ident
                    )))
                }
            }
            Expr::Return(expr) => expr.evaluate(runtime),
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
    Empty,
}

impl EvaluateValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            EvaluateValue::String(_) => true,
            EvaluateValue::Number(_) => true,
            EvaluateValue::Boolean(val) => *val,
            EvaluateValue::Nil => false,
            EvaluateValue::Empty => false,
        }
    }
}

impl Display for EvaluateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(val) => write!(f, "{val}"),
            Self::Number(val) => write!(f, "{val}"),
            Self::Boolean(val) => write!(f, "{val}"),
            Self::Nil => write!(f, "nil"),
            Self::Empty => write!(f, ""),
        }
    }
}
