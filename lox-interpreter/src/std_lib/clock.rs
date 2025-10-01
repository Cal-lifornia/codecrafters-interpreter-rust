use std::time::{SystemTime, UNIX_EPOCH};

use crate::{std_lib::NativeFunction, value::Value};

pub struct Clock {}
impl NativeFunction for Clock {
    fn run(&self, _args: &[Value]) -> crate::eval::EvalResult {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => Ok(Some(Value::Number(n.as_secs_f64()))),
            Err(err) => Err(lox_shared::error::LoxError::Runtime(err.to_string())),
        }
    }
}
