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
    super_class: Option<Box<Class>>,
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Class {
    pub fn new(class_item: &ClassItem, super_class: Option<Box<Class>>) -> Self {
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
            super_class,
        }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn get_method(&self, ident: &Ident) -> Option<&Method> {
        let res = self.methods.get(ident);
        if res.is_none() {
            if let Some(class) = &self.super_class {
                class.get_method(ident)
            } else {
                None
            }
        } else {
            res
        }
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
        let mut current_class = &mut inst.0.borrow_mut().super_class;
        while let Some(super_class) = current_class {
            for (_, method) in super_class.methods.iter_mut() {
                method.set_closure(closure.clone());
            }
            current_class = &mut super_class.super_class;
        }
        inst.clone()
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
