use crate::{
    ast::{
        expression::{BinOp, Expr, Literal, UnaryOp},
        item::FunSig,
        LogicOp,
    },
    error::InterpreterError,
    runtime::{loxtype::LoxType, program::Runtime},
};

impl Expr {
    pub fn evaluate(&self, runtime: &mut Runtime) -> Result<Option<LoxType>, InterpreterError> {
        match self {
            Expr::Literal(literal) => match literal {
                Literal::Number(num) => Ok(Some(LoxType::Number(*num))),
                Literal::String(val) => Ok(Some(LoxType::String(val.to_string()))),
                Literal::True => Ok(Some(LoxType::Boolean(true))),
                Literal::False => Ok(Some(LoxType::Boolean(false))),
                Literal::Nil => Ok(Some(LoxType::Nil)),
            },
            Expr::Group(group) => group.0.evaluate(runtime),
            Expr::Unary(unary_op, expr) => match unary_op {
                UnaryOp::Bang => match expr.evaluate(runtime)? {
                    Some(lox) => Ok(Some(!lox)),
                    None => Err(InterpreterError::Runtime(
                        "Incorrect expression type".to_string(),
                    )),
                },
                UnaryOp::Minus => match expr.evaluate(runtime)? {
                    Some(lox) => (-lox).map(Some),
                    _ => Err(InterpreterError::Runtime(
                        "Operand must be a number".to_string(),
                    )),
                },
            },
            Expr::Arithmetic(op, left, right) => {
                if let (Some(left_val), Some(right_val)) =
                    (left.evaluate(runtime)?, right.evaluate(runtime)?)
                {
                    match op {
                        BinOp::Add => (left_val + right_val).map(Some),
                        BinOp::Sub => (left_val - right_val).map(Some),
                        BinOp::Mul => (left_val * right_val).map(Some),
                        BinOp::Div => (left_val / right_val).map(Some),
                        BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                            if let (LoxType::Number(left_num), LoxType::Number(right_num)) =
                                (left_val.into_inner(), right_val.into_inner())
                            {
                                match op {
                                    BinOp::Lt => Ok(Some(LoxType::Boolean(left_num < right_num))),
                                    BinOp::Le => Ok(Some(LoxType::Boolean(left_num <= right_num))),
                                    BinOp::Gt => Ok(Some(LoxType::Boolean(left_num > right_num))),
                                    BinOp::Ge => Ok(Some(LoxType::Boolean(left_num >= right_num))),
                                    _ => unreachable!(),
                                }
                            } else {
                                Err(InterpreterError::Runtime(
                                    "Operand should be a number".to_string(),
                                ))
                            }
                        }
                        BinOp::Eq => Ok(Some(LoxType::Boolean(left_val == right_val))),
                        BinOp::Ne => Ok(Some(LoxType::Boolean(left_val != right_val))),
                    }
                } else {
                    Err(InterpreterError::Runtime(
                        "Can't evaluate empty expr".to_string(),
                    ))
                }
            }
            Expr::Conditional(op, left, right) => match op {
                LogicOp::Or => {
                    let Some(left_val) = left.evaluate(runtime)? else {
                        return Err(InterpreterError::Runtime(
                            "Expr must not be empty".to_string(),
                        ));
                    };
                    if left_val.is_truthy() {
                        return Ok(Some(left_val));
                    }
                    let Some(right_val) = right.evaluate(runtime)? else {
                        return Err(InterpreterError::Runtime(
                            "Expr must not be empty".to_string(),
                        ));
                    };
                    if right_val.is_truthy() {
                        return Ok(Some(right_val));
                    }
                    Ok(Some(LoxType::Boolean(false)))
                }

                LogicOp::And => {
                    let Some(left_val) = left.evaluate(runtime)? else {
                        return Err(InterpreterError::Runtime(
                            "Expr must not be empty".to_string(),
                        ));
                    };
                    if !left_val.is_truthy() {
                        return Ok(Some(LoxType::Boolean(false)));
                    }
                    let Some(right_val) = right.evaluate(runtime)? else {
                        return Err(InterpreterError::Runtime(
                            "Expr must not be empty".to_string(),
                        ));
                    };
                    if right_val.is_truthy() {
                        return Ok(Some(right_val));
                    }
                    Ok(Some(LoxType::Boolean(false)))
                }
            },
            Expr::Variable(ident) => match runtime.scope.find_var(ident) {
                Some(val) => Ok(Some(val.clone())),
                None => Err(InterpreterError::Runtime(format!(
                    "Undefined variable '{ident}'"
                ))),
            },
            Expr::InitVar(ident, expr) => {
                if let Some(value) = expr.evaluate(runtime)? {
                    runtime.scope.insert_var(ident.clone(), value.clone());
                    Ok(Some(value))
                } else {
                    Err(InterpreterError::Syntax(format!(
                        "Can't assign empty expression to {ident}"
                    )))
                }
            }
            Expr::UpdateVar(ident, expr) => {
                if let Some(value) = expr.evaluate(runtime)? {
                    runtime.scope.update_var(ident, value.clone())?;
                    Ok(Some(value))
                } else {
                    Err(InterpreterError::Syntax(format!(
                        "Can't update {ident} to empty expression"
                    )))
                }
            }
            Expr::Print(expr) => {
                match expr.evaluate(runtime)? {
                    Some(val) => println!("{val}"),
                    None => println!(),
                }
                Ok(None)
            }
            Expr::MethodCall(ident, args) => {
                let sig = FunSig::method_call(ident.clone(), args.len());
                let mut vals = vec![];
                if let Some(fun) = runtime.get_function(&sig) {
                    for arg in args.iter() {
                        if let Some(val) = arg.evaluate(runtime)? {
                            vals.push(val);
                        } else {
                            return Err(InterpreterError::Syntax(
                                "Can't use empty statement as parameter".to_string(),
                            ));
                        }
                    }
                    fun.run(runtime, vals)
                } else if let Some(fun) = runtime.get_native_fun(&sig) {
                    fun.run().map(Some)
                } else {
                    Err(InterpreterError::Runtime(format!(
                        "cannot find method with name {ident}"
                    )))
                }
            }
            Expr::Return(opt) => match opt {
                Some(expr) => Ok(expr
                    .evaluate(runtime)?
                    .map(|val| LoxType::Return(Box::new(val)))),
                None => Ok(Some(LoxType::Return(Box::new(LoxType::Nil)))),
            },
        }
    }
}
