use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{class::HezenClass, environment::HezenValue};

#[derive(Debug, Clone)]
struct HezenInstance {
    pub class: Rc<HezenClass>,
    pub fields: HashMap<String, HezenValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HezenInstanceHandle {
    instance: Rc<RefCell<HezenInstance>>,
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

impl HezenInstanceHandle {
    pub fn new(class: Rc<HezenClass>) -> Self {
        Self {
            instance: Rc::new(RefCell::new(HezenInstance {
                class,
                fields: HashMap::default(),
            })),
        }
    }

    pub fn get(&self, name: &str) -> Option<HezenValue> {
        if let Some(v) = self.instance.borrow().fields.get(name) {
            Some(v.clone())
        } else {
            self.instance
                .borrow()
                .class
                .find_method(name)
                .map(|m| HezenValue::Function(Rc::new(m.bind(self.clone()))))
        }
    }

    pub fn set(&self, name: String, value: HezenValue) {
        self.instance.borrow_mut().fields.insert(name, value);
    }

    pub fn type_name(&self) -> String {
        self.instance.borrow().class.name.clone()
    }
}
