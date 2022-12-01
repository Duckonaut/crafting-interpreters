use std::{cell::RefCell, rc::Rc};

use hezen_core::error::HezenError;

use crate::{
    ast::Stmt,
    environment::{HezenEnvironment, HezenValue},
    instance::HezenInstance,
    interpreter::{HezenControl, HezenInterruption, Interpreter},
    token::Token,
};

pub trait HezenCallable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[HezenValue],
    ) -> Result<HezenValue, HezenError>;
    fn arity(&self) -> usize;
    fn name(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Debug, Clone)]
pub struct HezenFunction {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Stmt,
    closure: Rc<HezenEnvironment>,
    initializer: bool,
}

impl HezenFunction {
    pub fn new(
        name: Token,
        parameters: Vec<Token>,
        body: Stmt,
        closure: Rc<HezenEnvironment>,
        initializer: bool,
    ) -> Self {
        Self {
            name,
            parameters,
            body,
            closure,
            initializer,
        }
    }

    pub fn bind(&self, instance: Rc<HezenInstance>) -> Self {
        let mut environment = HezenEnvironment::new(Some(Rc::clone(&self.closure)));
        environment.define(
            Token::new(
                crate::token::TokenType::Builtin,
                "self".into(),
                self.name.position.clone(),
            ),
            HezenValue::Instance(instance),
            false,
        );

        Self {
            name: self.name.clone(),
            parameters: self.parameters.clone(),
            body: self.body.clone(),
            closure: Rc::new(environment),
            initializer: self.initializer,
        }
    }
}

impl HezenCallable for HezenFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[HezenValue],
    ) -> Result<HezenValue, HezenError> {
        let mut environment = Rc::new(HezenEnvironment::new(Some(self.closure.clone())));

        for (parameter, argument) in self.parameters.iter().zip(arguments) {
            Rc::get_mut(&mut environment).unwrap().define(
                parameter.clone(),
                argument.clone(),
                false,
            );
        }

        let result = interpreter.execute_block(
            match &self.body {
                Stmt::Block(block) => block.iter().collect(),
                _ => unreachable!(),
            },
            environment.clone(),
        );

        if let Err(HezenInterruption::Control(HezenControl::Return(value))) = result {
            if self.initializer {
                return Ok(self.closure.get(&Token::new(
                    crate::token::TokenType::Self_,
                    "self".into(),
                    self.name.position.clone(),
                ))?);
            }

            return Ok(value);
        }

        if self.initializer {
            return Ok(self.closure.get(&Token::new(
                crate::token::TokenType::Self_,
                "self".into(),
                self.name.position.clone(),
            ))?);
        }

        result.map_err(|i| match i {
            HezenInterruption::Control(_) => panic!("break or continue uncaught at function boundary. Investigate resolver, should not be possible"),
            HezenInterruption::Error(why) => why,
        })
    }

    fn arity(&self) -> usize {
        self.parameters.len()
    }

    fn name(&self) -> String {
        self.name.lexeme.clone()
    }
}

impl PartialEq for HezenFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.parameters == other.parameters
            && self.initializer == other.initializer
    }
}

#[derive(Clone)]
pub struct HezenNativeFunction {
    pub name: Token,
    pub arity: usize,
    pub function: fn(&[HezenValue]) -> Result<HezenValue, HezenError>,
}

impl HezenNativeFunction {
    pub fn new(
        name: Token,
        arity: usize,
        function: fn(&[HezenValue]) -> Result<HezenValue, HezenError>,
    ) -> Self {
        Self {
            name,
            arity,
            function,
        }
    }
}

impl std::fmt::Debug for HezenNativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HezenNativeFunction")
            .field("name", &self.name)
            .field("arity", &self.arity)
            .finish()
    }
}

impl HezenCallable for HezenNativeFunction {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &[HezenValue],
    ) -> Result<HezenValue, HezenError> {
        if arguments.len() != self.arity {
            return Err(HezenError::runtime(
                self.name.position.file.clone(),
                self.name.position.line,
                self.name.position.column,
                format!(
                    "Expected {} arguments but got {}.",
                    self.arity,
                    arguments.len()
                ),
            ));
        }

        (self.function)(arguments)
    }

    fn arity(&self) -> usize {
        self.arity
    }

    fn name(&self) -> String {
        self.name.lexeme.clone()
    }
}
