mod expr;
mod item;
mod stmt;

use crate::{
    error::InterpreterError,
    runtime::{compiler::Compiler, loxtype::LoxType, resolver::Resolver},
};

#[derive(Default)]
pub struct Interpreter {
    pub resolver: Resolver,
    pub compiler: Compiler,
}

pub type EvaluateResult = Result<Option<LoxType>, InterpreterError>;
