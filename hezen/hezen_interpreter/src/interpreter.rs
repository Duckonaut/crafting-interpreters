use std::collections::HashMap;

use hezen_core::error::{HezenErrorList, HezenError};

use crate::{ast::{Expr, Stmt}, environment::HezenEnvironment};

#[derive(Debug)]
pub struct Interpreter {
    pub globals: HezenEnvironment,
    environment: HezenEnvironment,
    locals: HashMap<Expr, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            globals: HezenEnvironment::default(),
            environment: HezenEnvironment::default(),
            locals: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), HezenError> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), HezenError> {
        unimplemented!()
    }
}
