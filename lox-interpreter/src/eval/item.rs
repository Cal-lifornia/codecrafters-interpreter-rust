use lox_ast::ast::{Expr, Item, ItemKind};
use lox_shared::error::LoxError;

use crate::{
    Interpreter,
    eval::EvalResult,
    runtime_err,
    value::{Class, Method, Value},
};

impl Interpreter {
    pub fn evaluate_item(&mut self, item: &Item) -> Result<(), LoxError> {
        match item.kind() {
            ItemKind::Fun(function) => {
                let closure = self.capture_env();
                self.insert(
                    function.sig.ident.clone(),
                    Value::Function(Method::new(function.clone(), closure)),
                );
                Ok(())
            }
            ItemKind::Class(class_item) => {
                let super_class = if let Some(ident) = &class_item.super_class {
                    if let Some(Value::Class(class)) = self.find(ident, item.attr().id()) {
                        Some(Box::new(class.clone()))
                    } else {
                        return Err(runtime_err(
                            item.attr(),
                            format!("Could not find class {ident}"),
                        ));
                    }
                } else {
                    None
                };
                let class = Class::new(class_item, super_class);
                self.insert(class_item.ident.clone(), Value::Class(class));
                Ok(())
            }
        }
    }

    pub fn run_function(&mut self, method: &Method, args: &[Expr]) -> EvalResult {
        tracing::debug!("running function: {method}");

        if method.fun().param_len() != args.len() {
            return Err(LoxError::Runtime(format!(
                "Fun {} expects {} args, got {}",
                method.fun().sig.ident,
                method.fun().param_len(),
                args.len()
            )));
        }

        let vals = self.eval_function_params(args.to_vec())?;
        let current_env = self.capture_env();
        self.enter_closure(method.closure().clone());
        vals.iter()
            .zip(method.fun().sig.inputs.iter())
            .for_each(|(arg, input)| {
                self.insert(input.clone(), arg.clone());
            });
        let res = self
            .evaluate_block(&method.fun().body, false)
            .map(|val| val.map(|lox| lox.into_inner().clone()));
        self.exit_closure(current_env);
        res
    }
    pub fn eval_function_params(&mut self, args: Vec<Expr>) -> Result<Vec<Value>, LoxError> {
        let mut vals = vec![];
        for arg in args.iter() {
            if let Some(val) = self.evaluate_expr(arg)? {
                vals.push(val);
            } else {
                return Err(LoxError::Syntax(
                    "Can't use empty statement as parameter".into(),
                ));
            }
        }
        Ok(vals)
    }
}
