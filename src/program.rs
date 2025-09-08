use hashbrown::HashMap;

use crate::{ast::evaluate::EvaluateValue, error::InterpreterError};

#[derive(Debug, Default, Clone)]
pub struct Program {
    pub variables: HashMap<String, EvaluateValue>,
}

pub type ProgramResult = Result<(), InterpreterError>;
