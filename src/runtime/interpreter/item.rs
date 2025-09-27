use crate::{
    ast::item::{Function, Item},
    error::InterpreterError,
    runtime::{
        interpreter::{EvaluateResult, Interpreter},
        loxtype::LoxType,
    },
};

impl Interpreter {
    pub fn evaluate_item(&mut self, item: &Item) -> Result<(), InterpreterError> {
        match item {
            Item::Fun(function) => {
                let mut fun_clone = function.clone();
                fun_clone.closure = self.capture_env();
                self.insert_var(function.sig.ident.clone(), LoxType::Method(fun_clone));
                Ok(())
            }
        }
    }

    pub fn evaluate_function(&mut self, fun: &Function, args: Vec<LoxType>) -> EvaluateResult {
        let current_env = self.capture_env();
        self.enter_closure(fun.closure.clone());
        args.iter()
            .zip(fun.sig.inputs.iter())
            .for_each(|(arg, input)| {
                self.env.insert_var(input.clone(), arg.clone());
            });
        let res = self
            .evaluate_block(&fun.body)
            .map(|val| val.map(|lox| lox.into_inner().clone()));
        self.exit_closure(current_env);
        res
    }
}
