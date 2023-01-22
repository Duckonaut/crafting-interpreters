use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use hezen_core::error::HezenError;

use crate::{
    ast::Literal,
    class::HezenClass,
    function::{HezenFunction, HezenNativeFunction},
    instance::HezenInstanceHandle,
    token::Token,
};

#[derive(Debug, Default, Clone)]
pub struct HezenEnvironmentHandle {
    env: Rc<RefCell<HezenEnvironment>>,
}

#[derive(Debug, Default)]
struct HezenEnvironment {
    values: HashMap<String, HezenVariable>,
    enclosing: Option<HezenEnvironmentHandle>,
}

impl HezenEnvironmentHandle {
    pub fn new(enclosing: Option<HezenEnvironmentHandle>) -> Self {
        Self {
            env: Rc::new(RefCell::new(HezenEnvironment {
                values: HashMap::default(),
                enclosing,
            })),
        }
    }

    pub fn define(&mut self, name: Token, value: HezenValue, is_mutable: bool) {
        self.env.borrow_mut().values.insert(
            name.lexeme.clone(),
            HezenVariable::new(name, value, is_mutable),
        );
    }

    pub fn assign(&mut self, name: &Token, value: HezenValue) -> Result<(), HezenError> {
        if let Some(var) = self.env.borrow_mut().values.get_mut(&name.lexeme) {
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

        if let Some(enclosing) = &mut self.env.borrow_mut().enclosing {
            return enclosing.assign(name, value);
        }

        Err(HezenError::runtime(
            name.position.file.clone(),
            name.position.line,
            name.position.column,
            format!("Undefined variable IN ENVIRONMENT ASSIGN '{}'", name.lexeme),
        ))
    }

    pub fn get(&self, name: &Token) -> Result<HezenValue, HezenError> {
        if let Some(var) = self.env.borrow().values.get(&name.lexeme) {
            return Ok(var.value.clone());
        }

        if let Some(enclosing) = &self.env.borrow().enclosing {
            return enclosing.get(name);
        }

        Err(HezenError::runtime(
            name.position.file.clone(),
            name.position.line,
            name.position.column,
            format!("Undefined variable IN ENVIRONMENT GET '{}'", name.lexeme),
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
        self.ancestor(distance).assign(name, value)
    }

    fn ancestor(&self, distance: usize) -> HezenEnvironmentHandle {
        if distance == 0 {
            return self.clone();
        }
        let mut environment = self.env.borrow().enclosing.clone().unwrap();

        for _ in 0..(distance - 1) {
            environment = environment.clone().env.borrow().enclosing.clone().unwrap();
        }

        environment
    }

    pub fn defined(&self, name: &Token) -> bool {
        self.env.borrow().values.contains_key(&name.lexeme)
    }

    pub fn defined_at(&self, distance: usize, name: &Token) -> bool {
        self.ancestor(distance).defined(name)
    }

    pub fn mutable(&self, name: &Token) -> bool {
        (self.defined(name)
            && self
                .env
                .borrow()
                .values
                .get(&name.lexeme)
                .unwrap()
                .is_mutable)
            || (self.env.borrow().enclosing.is_some()
                && self.env.borrow().enclosing.clone().unwrap().mutable(name))
    }

    pub fn mutable_at(&self, distance: usize, name: &Token) -> bool {
        self.ancestor(distance).defined(name) && self.ancestor(distance).mutable(name)
    }

    pub fn enclosing(&self) -> Option<HezenEnvironmentHandle> {
        self.env.borrow().enclosing.clone()
    }

    pub(crate) fn all_values(&self) -> HashMap<String, (HezenVariable, usize)> {
        if let Some(enclosing) = &self.env.borrow().enclosing {
            let mut values = enclosing.all_values().iter_mut().map(|(k, v)| {
                v.1 += 1;
                (k.clone(), v.clone())
            }).collect::<HashMap<String, (HezenVariable, usize)>>();
            for (key, value) in self.env.borrow().values.iter() {
                values.insert(key.clone(), (value.clone(), 0));
            }
            values
        } else {
            self.env
                .borrow()
                .values
                .iter()
                .map(|(key, value)| (key.clone(), (value.clone(), 0)))
                .collect()
        }
    }
}

#[derive(Debug, Clone)]
pub struct HezenVariable {
    value: HezenValue,
    is_mutable: bool,
    pub definition_token: Token,
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
    Instance(HezenInstanceHandle),
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
            HezenValue::Instance(i) => format!("instance of {}", i.type_name()),
            HezenValue::NativeFunction(_) => "native function".to_string(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            HezenValue::Nil => false,
            HezenValue::Bool(b) => *b,
            HezenValue::Number(n) => *n != 0.0,
            HezenValue::String(s) => !s.is_empty(),
            HezenValue::Function(_) => true,
            HezenValue::Class(_) => true,
            HezenValue::Instance(_) => true,
            HezenValue::NativeFunction(_) => true,
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
            HezenValue::Instance(hi) => write!(f, "<instance {}>", hi.type_name()),
            HezenValue::NativeFunction(nf) => write!(f, "<native function {}>", nf.name.lexeme),
        }
    }
}

impl From<&Literal> for HezenValue {
    fn from(literal: &Literal) -> Self {
        match literal {
            Literal::Nil => Self::Nil,
            Literal::Bool(b) => Self::Bool(*b),
            Literal::Number(n) => Self::Number(*n),
            Literal::String(s) => Self::String(s.clone()),
        }
    }
}
