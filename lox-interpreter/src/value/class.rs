use std::{cell::RefCell, fmt::Display, rc::Rc};

use hashbrown::HashMap;
use lox_ast::ast::{ClassItem, Function, Ident};

use crate::{Interpreter, environment::Environment, value::Value};

#[derive(Debug, Clone)]
pub struct Class {
    ident: Ident,
    methods: HashMap<Ident, (Function, Environment)>,
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
            methods.insert(fun.sig.ident.clone(), (fun.clone(), env.clone()));
        }
        Self {
            ident: class_item.ident.clone(),
            methods,
        }
    }

    pub fn get_method(&mut self, ident: &Ident) -> Option<&(Function, Environment)> {
        self.methods.get(ident)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ident)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassInstance {
    class: Class,
    pub properties: HashMap<Ident, Value>,
}

impl ClassInstance {
    // pub fn new(
    //     interpreter: &mut Interpreter,
    //     class: &ClassItem,
    // ) -> Result<Rc<RefCell<Self>>, LoxError> {
    //     let mut properties = HashMap::new();
    //     let env = interpreter.capture_env();
    //     let mut resolver = Resolver::new(interpreter);
    //     for fun in &class.methods {
    //         properties.insert(
    //             fun.sig.ident.clone(),
    //             Value::Method(fun.clone(), env.clone()),
    //         );
    //         resolver.resolve_function(fun)?;
    //     }
    //     Ok(Rc::new(RefCell::new(Self {
    //         ident: class.ident.clone(),
    //         properties,
    //     })))
    // }
    pub fn new(class: Class) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            class: class.clone(),
            properties: HashMap::new(),
        }))
    }

    pub fn get_property(&self, prop: &Ident) -> Option<Value> {
        let res = self.properties.get(prop);
        if res.is_none() {
            self.class
                .methods
                .get(prop)
                .map(|(fun, env)| Value::Method(fun.clone(), env.clone()))
        } else {
            res.cloned()
        }
    }
}

impl Display for ClassInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.class.ident)
    }
}
