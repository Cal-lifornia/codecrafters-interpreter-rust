use std::{
    fmt::Display,
    ops::{Add, Not},
};

use crate::error::InterpreterError;

#[derive(Debug, Clone, PartialEq)]
pub enum LoxType {
    String(String),
    Number(f64),
    Boolean(bool),
    Variable(Box<LoxType>),
    Nil,
}

impl LoxType {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::String(_) => true,
            Self::Number(_) => true,
            Self::Boolean(val) => *val,
            Self::Nil => false,
            Self::Variable(var) => var.is_truthy(),
        }
    }
}

impl Display for LoxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(val) => write!(f, "{val}"),
            Self::Number(val) => write!(f, "{val}"),
            Self::Boolean(val) => write!(f, "{val}"),
            Self::Variable(val) => write!(f, "{val}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl Not for LoxType {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            LoxType::String(_) => LoxType::Boolean(false),
            LoxType::Number(_) => LoxType::Boolean(false),
            LoxType::Boolean(val) => LoxType::Boolean(!val),
            LoxType::Variable(lox_type) => !(*lox_type),
            LoxType::Nil => LoxType::Boolean(true),
        }
    }
}

impl Add for LoxType {
    type Output = Result<LoxType, InterpreterError>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (LoxType::Number(num_left), LoxType::Number(num_right)) => {
                Ok(LoxType::Number(num_left + num_right))
            }
            (LoxType::String(string_left), LoxType::String(string_right)) => {
                Ok(LoxType::String(format!("{string_left}{string_right}")))
            }

            _ => Err(InterpreterError::Runtime(
                "operand must be a number".to_string(),
            )),
        }
    }
}
