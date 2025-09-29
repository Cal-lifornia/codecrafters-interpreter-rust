use crate::{
    ast::{
        expr::{BinOp, ExprKind, Literal, LogicOp, UnaryOp},
        item::FunSig,
    },
    error::InterpreterError,
    runtime::{interpreter::Interpreter, loxtype::LoxType},
};

impl Interpreter {
    pub fn evaluate_expr(&mut self, expr: &ExprKind) -> Result<Option<LoxType>, InterpreterError> {
        match expr {
            ExprKind::Literal(literal) => match literal {
                Literal::Number(num) => Ok(Some(LoxType::Number(*num))),
                Literal::String(val) => Ok(Some(LoxType::String(val.to_string()))),
                Literal::True => Ok(Some(LoxType::Boolean(true))),
                Literal::False => Ok(Some(LoxType::Boolean(false))),
                Literal::Nil => Ok(Some(LoxType::Nil)),
            },
            ExprKind::Group(group) => self.evaluate_expr(&group.0),
            ExprKind::Unary(unary_op, expr) => match unary_op {
                UnaryOp::Bang => match self.evaluate_expr(expr)? {
                    Some(lox) => Ok(Some(!lox)),
                    None => Err(InterpreterError::Runtime(
                        "Incorrect expression type".to_string(),
                    )),
                },
                UnaryOp::Minus => match self.evaluate_expr(expr)? {
                    Some(lox) => (-lox).map(Some),
                    _ => Err(InterpreterError::Runtime(
                        "Operand must be a number".to_string(),
                    )),
                },
            },
            ExprKind::Arithmetic(op, left, right) => {
                if let (Some(left_val), Some(right_val)) =
                    (self.evaluate_expr(left)?, self.evaluate_expr(right)?)
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
            ExprKind::Conditional(op, left, right) => match op {
                LogicOp::Or => {
                    let Some(left_val) = self.evaluate_expr(left)? else {
                        return Err(InterpreterError::Runtime(
                            "Expr must not be empty".to_string(),
                        ));
                    };
                    if left_val.is_truthy() {
                        return Ok(Some(left_val));
                    }
                    let Some(right_val) = self.evaluate_expr(right)? else {
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
                    let Some(left_val) = self.evaluate_expr(left)? else {
                        return Err(InterpreterError::Runtime(
                            "Expr must not be empty".to_string(),
                        ));
                    };
                    if !left_val.is_truthy() {
                        return Ok(Some(LoxType::Boolean(false)));
                    }
                    let Some(right_val) = self.evaluate_expr(right)? else {
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
            ExprKind::Variable(ident) => match self.find(ident) {
                Some(val) => Ok(Some(val)),
                None => Err(InterpreterError::Runtime(format!(
                    "Undefined variable '{ident}'"
                ))),
            },
            ExprKind::InitVar(ident, expr) => {
                if let Some(value) = self.evaluate_expr(expr)? {
                    self.insert_var(ident.clone(), value.clone());
                    Ok(Some(value))
                } else {
                    Err(InterpreterError::Syntax(format!(
                        "Can't assign empty expression to {ident}"
                    )))
                }
            }
            ExprKind::UpdateVar(ident, expr) => {
                if let Some(value) = self.evaluate_expr(expr)? {
                    self.env.update(ident, value.clone())?;
                    Ok(Some(value))
                } else {
                    Err(InterpreterError::Syntax(format!(
                        "Can't update {ident} to empty expression"
                    )))
                }
            }
            ExprKind::Print(expr) => {
                match self.evaluate_expr(expr)? {
                    Some(val) => println!("{val}"),
                    None => println!(),
                }
                Ok(None)
            }
            ExprKind::MethodCall(ident, args) => {
                let sig = FunSig::method_call(ident.clone(), args.len());
                let mut vals = vec![];
                let res = self.find(ident);

                if let LoxType::Method(fun) = res.unwrap_or(LoxType::Nil).into_inner() {
                    if fun.param_len() != args.len() {
                        return Err(InterpreterError::Runtime(format!(
                            "Fun {} expects {} args, got {}",
                            fun.sig.ident,
                            fun.param_len(),
                            args.len()
                        )));
                    }
                    for arg in args.iter() {
                        if let Some(val) = self.evaluate_expr(arg)? {
                            vals.push(val);
                        } else {
                            return Err(InterpreterError::Syntax(
                                "Can't use empty statement as parameter".to_string(),
                            ));
                        }
                    }
                    self.evaluate_function(fun, vals)
                } else if let Some(fun) = self.get_native_fun(&sig) {
                    fun.run().map(Some)
                } else {
                    Err(InterpreterError::Runtime(format!(
                        "cannot find method with name {ident}"
                    )))
                }
            }
            ExprKind::Return(opt) => match opt {
                Some(expr) => Ok(self
                    .evaluate_expr(expr)?
                    .map(|val| LoxType::Return(Box::new(val)))),
                None => Ok(Some(LoxType::Return(Box::new(LoxType::Nil)))),
            },
        }
    }
}
