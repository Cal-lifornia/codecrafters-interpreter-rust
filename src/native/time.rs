use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    ast::evaluate::{EvaluateResult, EvaluateValue},
    error::InterpreterError,
    native::NativeFunction,
};

pub struct Clock;

impl NativeFunction for Clock {
    fn run(&self) -> EvaluateResult {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => Ok(EvaluateValue::Number(time.as_secs() as f64)),
            Err(err) => Err(InterpreterError::Runtime(format!("Error: {err}"))),
        }
    }
}
