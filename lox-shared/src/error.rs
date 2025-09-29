use crate::SStr;

#[derive(Debug, thiserror::Error, Clone)]
pub enum LoxError {
    #[error("{0}")]
    Runtime(SStr),
    #[error("{0}")]
    Syntax(SStr),
}

impl LoxError {
    pub fn exit_code(&self) -> i32 {
        match self {
            LoxError::Runtime(_) => 70,
            LoxError::Syntax(_) => 65,
        }
    }
}
