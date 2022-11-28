use std::{sync::Arc, collections::HashMap};

use crate::{token::Token, function::HezenFunction, class::HezenClass, instance::HezenInstance};

#[derive(Debug, Clone, Default)]
pub struct HezenEnvironment {
    values: HashMap<String, HezenVariable>,
    enclosing: Option<Arc<HezenEnvironment>>,
}

#[derive(Debug, Clone)]
pub struct HezenVariable {
    value: HezenValue,
    is_mutable: bool,
    definition_token: Token,
}

#[derive(Debug, Clone)]
pub enum HezenValue {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Function(HezenFunction),
    Class(HezenClass),
    Instance(HezenInstance),
}
