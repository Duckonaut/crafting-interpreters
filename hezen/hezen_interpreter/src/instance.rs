use std::{sync::Arc, collections::HashMap};

use crate::{class::HezenClass, environment::HezenVariable};

#[derive(Debug, Clone)]
pub struct HezenInstance {
    pub class: Arc<HezenClass>,
    pub fields: HashMap<String, HezenVariable>,
}
