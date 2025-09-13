use crate::ast::evaluate::EvaluateResult;

pub mod time;

pub trait NativeFunction {
    fn run(&self) -> EvaluateResult;
}
