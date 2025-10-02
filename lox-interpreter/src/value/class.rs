use std::{cell::RefCell, fmt::Display, rc::Rc};

use hashbrown::HashMap;
use lox_ast::ast::{ClassItem, Expr, Function, Ident};
use lox_shared::error::LoxError;

use crate::{Interpreter, environment::Environment, eval::EvalResult, value::Value};

#[derive(Debug, Clone)]
pub struct Class {
    ident: Ident,
    methods: HashMap<Ident, Method>,
    pub properties: HashMap<Ident, Value>,
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Class {
    pub fn new(interpreter: &mut Interpreter, class_item: &ClassItem) -> Self {
        let mut methods = HashMap::new();
        let env = interpreter.capture_env();
        for fun in &class_item.methods {
            methods.insert(
                fun.sig.ident.clone(),
                Method::new(fun.clone(), env.clone(), None),
            );
        }
        Self {
            ident: class_item.ident.clone(),
            methods,
            properties: HashMap::new(),
        }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn get_method(&self, ident: &Ident) -> Option<&Method> {
        self.methods.get(ident)
    }

    pub fn set(&mut self, ident: &Ident, val: Value) -> Result<(), LoxError> {
        match val {
            Value::ClassMethod(method) => {
                if let Some(val) = self.methods.get_mut(ident) {
                    *val = method;
                    Ok(())
                } else {
                    Err(LoxError::Runtime(format!(
                        "Could not find method named {ident} on class {}",
                        self.ident
                    )))
                }
            }
            _ => {
                self.properties.insert(ident.clone(), val);
                Ok(())
            }
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassInstance(Rc<RefCell<Class>>);

impl ClassInstance {
    pub fn new(class: Class) -> Self {
        Self(Rc::new(RefCell::new(class)))
    }
    pub fn class(&self) -> Rc<RefCell<Class>> {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Method {
    fun: Function,
    closure: Environment,
    pub this: Option<ClassInstance>,
}

impl Display for ClassInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.0.borrow().ident)
    }
}

impl Method {
    pub fn new(fun: Function, closure: Environment, this: Option<ClassInstance>) -> Self {
        Self { fun, closure, this }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.fun.sig.ident)
    }
}

impl Interpreter {
    pub fn run_method(
        &mut self,
        inst: &ClassInstance,
        method: &Method,
        args: &[Expr],
    ) -> EvalResult {
        let prev = std::mem::replace(&mut self.this, method.this.clone());
        if self.this.is_none() {
            self.this = Some(inst.clone());
        }

        if method.fun.param_len() != args.len() {
            return Err(LoxError::Runtime(format!(
                "Fun {} expects {} args, got {}",
                method.fun.sig.ident,
                method.fun.param_len(),
                args.len()
            )));
        }

        let vals = self.eval_function_params(args.to_vec())?;
        let res = self.run_function(&method.fun, method.closure.clone(), vals);
        self.this = prev;
        res
    }
}
