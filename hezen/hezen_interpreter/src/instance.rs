use std::{collections::HashMap, rc::Rc};

use crate::{class::HezenClass, environment::HezenVariable};

#[derive(Debug, Clone)]
pub struct HezenInstance {
    pub class: Rc<HezenClass>,
    pub fields: HashMap<String, HezenVariable>,
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
}
