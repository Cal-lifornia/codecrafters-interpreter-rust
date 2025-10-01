use lox_shared::{error::LoxError, mod_flat};

use crate::value::Value;

mod_flat!(item expr stmt);

pub type EvalResult = Result<Option<Value>, LoxError>;
