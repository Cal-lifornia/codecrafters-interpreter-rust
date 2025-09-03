#[derive(Debug, thiserror::Error)]
pub enum InterpreterError {
    #[error("{0}")]
    Runtime(String),
    #[error("{0}")]
    Syntax(String),
}

impl InterpreterError {
    pub fn new_syntax_err(err: &dyn std::fmt::Display) -> Self {
        Self::Syntax(format!("{err}"))
    }
    pub fn new_runtime_err(err: &dyn std::fmt::Display) -> Self {
        Self::Runtime(format!("{err}"))
    }
    pub fn exit_code(&self) -> i32 {
        match self {
            InterpreterError::Runtime(_) => 70,
            InterpreterError::Syntax(_) => 65,
        }
    }
}
