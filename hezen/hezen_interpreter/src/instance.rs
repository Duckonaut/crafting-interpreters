use std::{collections::HashMap, rc::Rc};

use crate::{class::HezenClass, environment::{HezenVariable, HezenValue}};

#[derive(Debug, Clone)]
pub struct HezenInstance {
    pub class: Rc<HezenClass>,
    pub fields: HashMap<String, HezenValue>,
}

impl PartialEq for HezenInstance {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class
            && self
                .fields
                .iter()
                .all(|(k, v)| other.fields.get(k) == Some(v))
    }
}

impl HezenInstance {
    pub fn new(class: Rc<HezenClass>) -> Self {
        Self {
            class,
            fields: HashMap::default(),
        }
    }

    pub fn get(&self, name: &str) -> Option<HezenValue> {
        self.fields.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: HezenValue) {
        self.fields.insert(name, value);
    }
}
