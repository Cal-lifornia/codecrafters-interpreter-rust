use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

use lox_ast::ast::Function;
use lox_shared::{SStr, error::LoxError};

use crate::{
    environment::Environment,
    value::{Class, ClassInstance},
};

#[derive(Debug, Clone)]
pub enum Value {
    String(SStr),
    Number(f64),
    Boolean(bool),
    Nil,
    Return(Box<Value>),
    Function(Method),
    Class(Class),
    ClassInst(ClassInstance),
    ClassInit(ClassInitialiser),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::String(_) => true,
            Self::Number(_) => true,
            Self::Boolean(val) => *val,
            Self::Nil => false,
            // Self::Variable(val) => val.is_truthy(),
            Self::Return(val) => val.is_truthy(),
            Self::Function(_) => false,
            Self::Class(_) => false,
            Self::ClassInst(_) => true,
            Self::ClassInit(_) => false,
        }
    }

    pub fn into_inner(&self) -> &Self {
        match self {
            // Self::Variable(val) => val,
            Self::Return(val) => val,
            _ => self,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(val) => write!(f, "{val}"),
            Self::Number(val) => write!(f, "{val}"),
            Self::Boolean(val) => write!(f, "{val}"),
            Self::Nil => write!(f, "nil"),
            // Self::Variable(val) => write!(f, "{val}"),
            Self::Return(val) => write!(f, "{val}"),
            Self::Function(method) => write!(f, "{method}"),
            Self::Class(class) => write!(f, "{}", class),
            Self::ClassInst(class_inst) => write!(f, "{}", class_inst),
            Self::ClassInit((method, _)) => write!(f, "{method}"),
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Value::String(_) => Value::Boolean(false),
            Value::Number(_) => Value::Boolean(false),
            Value::Boolean(val) => Value::Boolean(!val),
            Value::Nil => Value::Boolean(true),
            // Value::Variable(lox_type) => !(*lox_type),
            Value::Return(lox_type) => !(*lox_type),
            Value::Function(_) => Value::Boolean(false),
            Value::Class(_) => Value::Boolean(false),
            Value::ClassInst(_) => Value::Boolean(false),
            Value::ClassInit(_) => Value::Boolean(false),
        }
    }
}

impl Add for Value {
    type Output = Result<Value, LoxError>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self.into_inner(), rhs.into_inner()) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
            (Value::String(left), Value::String(right)) => {
                Ok(Value::String(format!("{left}{right}").into()))
            }

            _ => Err(LoxError::Runtime("operand must be a number".into())),
        }
    }
}

impl Sub for Value {
    type Output = Result<Value, LoxError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if let (Value::Number(left), Value::Number(right)) = (self.into_inner(), rhs.into_inner()) {
            Ok(Value::Number(left - right))
        } else {
            Err(LoxError::Runtime("operand must be a number".into()))
        }
    }
}

impl Mul for Value {
    type Output = Result<Value, LoxError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if let (Value::Number(left), Value::Number(right)) = (self.into_inner(), rhs.into_inner()) {
            Ok(Value::Number(left * right))
        } else {
            Err(LoxError::Runtime("operand must be a number".into()))
        }
    }
}

impl Div for Value {
    type Output = Result<Value, LoxError>;

    fn div(self, rhs: Self) -> Self::Output {
        if let (Value::Number(left), Value::Number(right)) = (self.into_inner(), rhs.into_inner()) {
            Ok(Value::Number(left / right))
        } else {
            Err(LoxError::Runtime("operand must be a number".into()))
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self.into_inner(), other.into_inner()) {
            (Self::String(left), Self::String(right)) => left == right,
            (Self::Number(left), Self::Number(right)) => left == right,
            (Self::Boolean(left), Self::Boolean(right)) => left == right,
            (Self::Nil, Self::Nil) => true,
            _ => false,
        }
    }
}

impl Neg for Value {
    type Output = Result<Value, LoxError>;

    fn neg(self) -> Self::Output {
        if let Self::Number(num) = self.into_inner() {
            Ok(Value::Number(-num))
        } else {
            Err(LoxError::Runtime("Operand must be a number".into()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Method {
    fun: Function,
    closure: Environment,
}

impl Method {
    pub fn new(fun: Function, closure: Environment) -> Self {
        Self { fun, closure }
    }
    pub fn fun(&self) -> &Function {
        &self.fun
    }

    pub fn closure(&self) -> &Environment {
        &self.closure
    }

    pub fn set_closure(&mut self, closure: Environment) {
        self.closure = closure;
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.fun.sig.ident)
    }
}

pub type ClassInitialiser = (Method, ClassInstance);
