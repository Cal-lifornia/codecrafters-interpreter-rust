use lox_ast::ast::{Function, Item, ItemKind};
use lox_shared::error::LoxError;

use crate::{
    Interpreter,
    environment::Environment,
    eval::EvalResult,
    value::{Class, Value},
};

impl Interpreter {
    pub fn evaluate_item(&mut self, item: &Item) -> Result<(), LoxError> {
        match item.kind() {
            ItemKind::Fun(function) => {
                let closure = self.capture_env();
                self.insert(
                    function.sig.ident.clone(),
                    Value::Method(function.clone(), closure),
                );
                Ok(())
            }
            ItemKind::Class(class_item) => {
                let class = Class::new(self, class_item);
                self.insert(class_item.ident.clone(), Value::Class(class));
                Ok(())
            }
        }
    }

    pub fn run_function(
        &mut self,
        fun: &Function,
        closure: Environment,
        args: Vec<Value>,
    ) -> EvalResult {
        let current_env = self.capture_env();
        self.enter_closure(closure);
        args.iter()
            .zip(fun.sig.inputs.iter())
            .for_each(|(arg, input)| {
                self.insert(input.clone(), arg.clone());
            });
        let res = self
            .evaluate_block(&fun.body, false)
            .map(|val| val.map(|lox| lox.into_inner().clone()));
        self.exit_closure(current_env);
        res
    }
}
