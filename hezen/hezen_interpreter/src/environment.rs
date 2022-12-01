use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use hezen_core::error::HezenError;

use crate::{
    class::HezenClass,
    function::{HezenFunction, HezenNativeFunction},
    instance::HezenInstance,
    token::Token,
};

#[derive(Debug, Default)]
pub struct HezenEnvironment {
    values: HashMap<String, HezenVariable>,
    enclosing: Option<Rc<HezenEnvironment>>,
}

impl HezenEnvironment {
    pub fn new(enclosing: Option<Rc<HezenEnvironment>>) -> Self {
        Self {
            values: HashMap::default(),
            enclosing,
        }
    }

    pub fn define(
        &mut self,
        name: Token,
        value: HezenValue,
        is_mutable: bool,
    ) -> &HezenVariable {
        self.values.insert(
            name.lexeme.clone(),
            HezenVariable::new(name.clone(), value, is_mutable),
        );

        self.values.get(&name.lexeme).unwrap()
    }

    pub fn assign(&mut self, name: &Token, value: HezenValue) -> Result<(), HezenError> {
        if let Some(var) = self.values.get_mut(&name.lexeme) {
            if var.is_mutable {
                var.value = value;
                return Ok(());
            } else {
                return Err(HezenError::runtime(
                    name.position.file.clone(),
                    name.position.line,
                    name.position.column,
                    format!("Cannot assign to immutable variable '{}'", name.lexeme),
                ));
            }
        }

        if let Some(enclosing) = &mut self.enclosing {
            return Rc::get_mut(enclosing).unwrap().assign(name, value);
        }

        Err(HezenError::runtime(
            name.position.file.clone(),
            name.position.line,
            name.position.column,
            format!("Undefined variable '{}'", name.lexeme),
        ))
    }

    pub fn get(&self, name: &Token) -> Result<HezenValue, HezenError> {
        if let Some(var) = self.values.get(&name.lexeme) {
            return Ok(var.value.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }

        Err(HezenError::runtime(
            name.position.file.clone(),
            name.position.line,
            name.position.column,
            format!("Undefined variable '{}'", name.lexeme),
        ))
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> Result<HezenValue, HezenError> {
        self.ancestor(distance).get(name)
    }

    pub fn assign_at(
        &mut self,
        distance: usize,
        name: &Token,
        value: HezenValue,
    ) -> Result<(), HezenError> {
        Rc::get_mut(&mut self.ancestor(distance)).unwrap().assign(name, value)
    }

    fn ancestor(&self, distance: usize) -> Rc<HezenEnvironment> {
        let mut environment = self.enclosing.clone().unwrap();

        for _ in 0..(distance - 1) {
            environment = environment.enclosing.clone().unwrap();
        }

        environment
    }

    pub fn defined(&self, name: &Token) -> bool {
        self.values.contains_key(&name.lexeme)
    }

    pub fn defined_at(&self, distance: usize, name: &Token) -> bool {
        self.ancestor(distance).defined(name)
    }

    pub fn mutable(&self, name: &Token) -> bool {
        (self.defined(name) && self.values.get(&name.lexeme).unwrap().is_mutable)
            || (self.enclosing.is_some() && self.enclosing.clone().unwrap().mutable(name))
    }

    pub fn mutable_at(&self, distance: usize, name: &Token) -> bool {
        self.ancestor(distance).defined(name)
            && self.ancestor(distance).mutable(name)
    }
}

#[derive(Debug, Clone)]
pub struct HezenVariable {
    value: HezenValue,
    is_mutable: bool,
    definition_token: Token,
}

impl HezenVariable {
    pub fn new(name: Token, value: HezenValue, is_mutable: bool) -> Self {
        Self {
            value,
            is_mutable,
            definition_token: name,
        }
    }

    pub fn get(&self) -> HezenValue {
        self.value.clone()
    }

    pub fn set(&mut self, value: HezenValue) {
        self.value = value;
    }
}

impl PartialEq for HezenVariable {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Debug, Clone)]
pub enum HezenValue {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Function(Rc<HezenFunction>),
    NativeFunction(Rc<HezenNativeFunction>),
    Class(Rc<HezenClass>),
    Instance(Rc<HezenInstance>),
}

impl PartialEq for HezenValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Function(a), Self::Function(b)) => a == b,
            (Self::Class(a), Self::Class(b)) => a == b,
            (Self::Instance(a), Self::Instance(b)) => a == b,
            _ => false,
        }
    }
}

impl HezenValue {
    pub fn type_name(&self) -> String {
        match self {
            HezenValue::Nil => "nil".to_string(),
            HezenValue::Bool(_) => "bool".to_string(),
            HezenValue::Number(_) => "number".to_string(),
            HezenValue::String(_) => "string".to_string(),
            HezenValue::Function(_) => "function".to_string(),
            HezenValue::Class(c) => format!("class {}", c.name),
            HezenValue::Instance(i) => format!("{}", i.class.name),
            HezenValue::NativeFunction(_) => format!("native function"),
        }
    }
}

impl Display for HezenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HezenValue::Nil => write!(f, "nil"),
            HezenValue::Bool(b) => write!(f, "{}", b),
            HezenValue::Number(n) => write!(f, "{}", n),
            HezenValue::String(s) => write!(f, "{}", s),
            HezenValue::Function(hf) => write!(f, "<function {}>", hf.name.lexeme),
            HezenValue::Class(hc) => write!(f, "<class {}>", hc.name),
            HezenValue::Instance(hi) => write!(f, "<instance {}>", hi.class.name),
            HezenValue::NativeFunction(nf) => write!(f, "<native function {}>", nf.name.lexeme),
        }
    }
}
