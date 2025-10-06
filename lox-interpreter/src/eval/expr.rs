use lox_ast::ast::{BinOp, Expr, ExprKind, Literal, LogicOp, UnaryOp};
use lox_shared::error::LoxError;

use crate::{
    Interpreter,
    eval::EvalResult,
    runtime_err,
    value::{ClassInstance, Value},
};

impl Interpreter {
    pub fn evaluate_expr(&mut self, expr: &Expr) -> EvalResult {
        match expr.kind() {
            ExprKind::Literal(literal) => match literal {
                Literal::Number(num) => Ok(Some(Value::Number(*num))),
                Literal::String(val) => Ok(Some(Value::String(val.clone().into()))),
                Literal::True => Ok(Some(Value::Boolean(true))),
                Literal::False => Ok(Some(Value::Boolean(false))),
                Literal::Nil => Ok(Some(Value::Nil)),
            },
            ExprKind::Group(group) => self.evaluate_expr(&group.0),
            ExprKind::Unary(unary_op, expr) => match unary_op {
                UnaryOp::Bang => match self.evaluate_expr(expr)? {
                    Some(lox) => Ok(Some(!lox)),
                    None => Err(LoxError::Runtime("Incorrect expression type".into())),
                },
                UnaryOp::Minus => match self.evaluate_expr(expr)? {
                    Some(lox) => (-lox).map(Some),
                    _ => Err(LoxError::Runtime("Operand must be a number".into())),
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
                            if let (Value::Number(left_num), Value::Number(right_num)) =
                                (left_val.into_inner(), right_val.into_inner())
                            {
                                match op {
                                    BinOp::Lt => Ok(Some(Value::Boolean(left_num < right_num))),
                                    BinOp::Le => Ok(Some(Value::Boolean(left_num <= right_num))),
                                    BinOp::Gt => Ok(Some(Value::Boolean(left_num > right_num))),
                                    BinOp::Ge => Ok(Some(Value::Boolean(left_num >= right_num))),
                                    _ => unreachable!(),
                                }
                            } else {
                                Err(LoxError::Runtime("Operand should be a number".into()))
                            }
                        }
                        BinOp::Eq => Ok(Some(Value::Boolean(left_val == right_val))),
                        BinOp::Ne => Ok(Some(Value::Boolean(left_val != right_val))),
                    }
                } else {
                    Err(runtime_err(
                        expr.attr(),
                        format!("can't evaluate empty expression {expr}"),
                    ))
                }
            }
            ExprKind::Conditional(op, left, right) => match op {
                LogicOp::Or => {
                    let Some(left_val) = self.evaluate_expr(left)? else {
                        return Err(LoxError::Runtime("Expr must not be empty".into()));
                    };
                    if left_val.is_truthy() {
                        return Ok(Some(left_val));
                    }
                    let Some(right_val) = self.evaluate_expr(right)? else {
                        return Err(LoxError::Runtime("Expr must not be empty".into()));
                    };
                    if right_val.is_truthy() {
                        return Ok(Some(right_val));
                    }
                    Ok(Some(Value::Boolean(false)))
                }

                LogicOp::And => {
                    let Some(left_val) = self.evaluate_expr(left)? else {
                        return Err(LoxError::Runtime("Expr must not be empty".into()));
                    };
                    if !left_val.is_truthy() {
                        return Ok(Some(Value::Boolean(false)));
                    }
                    let Some(right_val) = self.evaluate_expr(right)? else {
                        return Err(LoxError::Runtime("Expr must not be empty".into()));
                    };
                    if right_val.is_truthy() {
                        return Ok(Some(right_val));
                    }
                    Ok(Some(Value::Boolean(false)))
                }
            },
            ExprKind::Variable(ident) => match self.find(ident, expr.attr().id()) {
                Some(val) => Ok(Some(val)),
                None => {
                    tracing::debug!("{}", self.debug_display());

                    Err(LoxError::Runtime(format!(
                        "{}; Undefined variable '{ident}'",
                        expr.attr().as_display()
                    )))
                }
            },
            ExprKind::InitVar(ident, expr) => {
                if let Some(value) = self.evaluate_expr(expr)? {
                    self.insert(ident.clone(), value.clone());
                    Ok(Some(value))
                } else {
                    Err(LoxError::Syntax(format!(
                        "Can't assign empty expression to {ident}"
                    )))
                }
            }
            ExprKind::UpdateVar(ident, subexpr) => {
                if let Some(value) = self.evaluate_expr(subexpr)? {
                    if let Err(err) = self.update(ident, expr.attr().id(), value.clone()) {
                        #[cfg(debug_assertions)]
                        eprintln!("'current stack'\n{}", self.debug_display());

                        return Err(LoxError::Syntax(format!(
                            "{}; {err}",
                            expr.attr().as_display()
                        )));
                    }
                    Ok(Some(value))
                } else {
                    Err(LoxError::Syntax(format!(
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
            ExprKind::FunctionCall(expr, args) => {
                if let ExprKind::Variable(ident) = expr.kind()
                    && self.native_functions.contains_key(ident)
                {
                    let vals = self.eval_function_params(args.to_vec())?;
                    self.native_functions.get(ident).unwrap().run(&vals)
                } else {
                    match self.evaluate_expr(expr)? {
                        Some(Value::Function(method)) => self.run_function(&method, args),
                        Some(Value::Class(class)) => Ok(Some(Value::ClassInst(
                            ClassInstance::new(self.capture_env(), class),
                        ))),

                        out => {
                            tracing::debug!("other function call {out:#?}");

                            #[cfg(debug_assertions)]
                            eprintln!("'current stack'\n{}", self.debug_display());

                            Err(LoxError::Runtime(format!(
                                "{}; Cannot find method {expr}",
                                expr.attr().as_display(),
                            )))
                        }
                    }
                }
            }
            ExprKind::Get(left, right) => {
                tracing::debug!("running get; left: {left}; right: {right}");

                if let Some(Value::ClassInst(inst)) = self.evaluate_expr(left)? {
                    match right.kind() {
                        ExprKind::Variable(prop) => {
                            let res = inst.class().borrow().properties.get(prop).cloned();
                            if res.is_none() {
                                let res = inst
                                    .class()
                                    .borrow()
                                    .get_method(prop)
                                    .map(|method| Value::Function(method.clone()));
                                if res.is_none() {
                                    Err(runtime_err(right.attr(), "found no class property"))
                                } else {
                                    Ok(res)
                                }
                            } else {
                                Ok(res)
                            }
                        }
                        ExprKind::FunctionCall(expr, args) => {
                            if let Some(Value::Function(method)) = self.evaluate_expr(expr)? {
                                self.run_function(&method, args)
                            } else {
                                Err(runtime_err(
                                    right.attr(),
                                    format!("Expected property or method call: got {right}"),
                                ))
                            }
                        }
                        _ => Err(runtime_err(
                            right.attr(),
                            format!("Expected property or method call: got {right}"),
                        )),
                    }
                } else {
                    Err(LoxError::Runtime(format!(
                        "{}; (get) only classes have fields",
                        left.attr().as_display()
                    )))
                }
            }
            ExprKind::Set(expr, prop, sub_expr) => {
                if let Some(val) = self.evaluate_expr(sub_expr)? {
                    let ret = self.evaluate_expr(expr)?;
                    if let Some(Value::ClassInst(inst)) = ret {
                        inst.class().borrow_mut().set(prop, val)?;
                        Ok(None)
                    } else {
                        Err(LoxError::Runtime(format!(
                            "{}; (set) only classes have fields; got {ret:#?}",
                            expr.attr().as_display()
                        )))
                    }
                } else {
                    Err(runtime_err(
                        expr.attr(),
                        "Cannot assign empty expression to class propery",
                    ))
                }
            }
            ExprKind::Return(opt) => match opt {
                Some(expr) => Ok(self
                    .evaluate_expr(expr)?
                    .map(|val| Value::Return(Box::new(val)))),
                None => Ok(Some(Value::Return(Box::new(Value::Nil)))),
            },
        }
    }
}
