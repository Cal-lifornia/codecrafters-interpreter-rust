use lox_shared::mod_flat;

use crate::{eval::EvalResult, value::Value};

mod_flat!(clock);

pub trait NativeFunction {
    fn run(&self, args: &[Value]) -> EvalResult;
}
