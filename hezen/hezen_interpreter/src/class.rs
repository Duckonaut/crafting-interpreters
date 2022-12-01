use std::{collections::HashMap, rc::Rc};

use hezen_core::error::HezenError;

use crate::{
    environment::HezenValue,
    function::{HezenCallable, HezenFunction},
    instance::HezenInstance,
    interpreter::Interpreter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassType {
    None,
    Class,
    Subclass,
}

#[derive(Debug, Clone)]
pub struct HezenClass {
    pub name: String,
    pub superclass: Option<Rc<HezenClass>>,
    pub methods: HashMap<String, HezenFunction>,
}

impl HezenClass {
    pub fn new(
        name: String,
        superclass: Option<Rc<HezenClass>>,
        methods: HashMap<String, HezenFunction>,
    ) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }

    pub fn find_method(&self, name: &str) -> Option<HezenFunction> {
        match self.methods.get(name.into()) {
            Some(m) => Some(m.clone()),
            None => match &self.superclass {
                Some(sc) => sc.find_method(name),
                None => None,
            },
        }
    }
}

impl PartialEq for HezenClass {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.superclass == other.superclass
            && self.methods == other.methods
    }
}

impl HezenCallable for Rc<HezenClass>
{
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[HezenValue],
    ) -> Result<HezenValue, HezenError> {
        if let Some(init) = self.methods.get("init") {
            init.call(interpreter, arguments)
        } else {
            Ok(HezenValue::Instance(Rc::new(HezenInstance::new(self.clone()))))
        }
    }

    fn arity(&self) -> usize {
        if let Some(init) = self.methods.get("init") {
            init.arity()
        } else {
            0
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
