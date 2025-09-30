#[derive(Debug, thiserror::Error, Clone)]
pub enum LoxError {
    #[error("{0}")]
    Runtime(String),
    #[error("{0}")]
    Syntax(String),
}

impl LoxError {
    pub fn exit_code(&self) -> i32 {
        match self {
            LoxError::Runtime(_) => 70,
            LoxError::Syntax(_) => 65,
        }
    }
}
