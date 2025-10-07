use std::{cell::RefCell, fmt::Display, rc::Rc};

use hashbrown::HashMap;
use lox_ast::ast::{ClassItem, Ident};
use lox_shared::error::LoxError;

use crate::{
    environment::Environment,
    value::{Method, Value},
};

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
    pub fn new(class_item: &ClassItem) -> Self {
        let mut methods = HashMap::new();
        for method in &class_item.methods {
            methods.insert(
                method.sig.ident.clone(),
                Method::new(method.clone(), Environment::default()),
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
            Value::Function(method) => {
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
    pub fn new(mut closure: Environment, class: Class) -> Self {
        let inst = Self(Rc::new(RefCell::new(class)));
        closure.enter_scope();
        closure.insert(Ident("this".into()), Value::ClassInst(inst.clone()));
        for (_, method) in inst.0.borrow_mut().methods.iter_mut() {
            method.set_closure(closure.clone());
        }
        inst
    }

    pub fn class(&self) -> Rc<RefCell<Class>> {
        self.0.clone()
    }
}

impl Display for ClassInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.0.borrow().ident)
    }
}
