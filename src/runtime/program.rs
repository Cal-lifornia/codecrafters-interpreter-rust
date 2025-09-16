use hashbrown::HashMap;

use crate::{
    ast::{
        ident::Ident,
        item::{FunSig, Function},
        parse::{token_stream::generate_token_stream, Parser},
    },
    error::InterpreterError,
    native::{time::Clock, NativeFunction},
    runtime::scope::Scope,
    tokens::{Lexer, Token},
};

pub struct Runtime {
    pub scope: Scope,
    pub functions: HashMap<FunSig, Function>,
    pub native_functions: HashMap<FunSig, Box<dyn NativeFunction>>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        let mut native_fun: HashMap<FunSig, Box<dyn NativeFunction>> = HashMap::new();
        native_fun.insert(
            FunSig {
                ident: Ident("clock".into()),
                inputs: vec![],
            },
            Box::new(Clock),
        );
        Runtime {
            scope: Scope::default(),
            functions: HashMap::new(),
            native_functions: native_fun,
        }
    }
    pub fn run(&mut self, filename: &str) -> Result<(), InterpreterError> {
        let mut ast = vec![];
        let mut lexer = Lexer::new(filename)?;
        let stream = generate_token_stream(&mut lexer)?;
        let mut parser = Parser::new(stream);
        loop {
            if parser.current_token == Token::EOF {
                break;
            } else {
                ast.push(parser.parse_stmt()?);
            }
        }
        for stmt in ast.clone() {
            stmt.run(self)?;
        }
        Ok(())
    }

    pub fn get_native_fun(&self, sig: &FunSig) -> Option<&dyn NativeFunction> {
        self.native_functions.get(sig).map(|fun| fun.as_ref())
    }
}
