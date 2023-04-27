use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

#[derive(Debug)]
pub struct Environment {
    bindings: HashMap<String, Object>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    pub fn extend(parent: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            bindings: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, identifier: &str) -> Option<Object> {
        match self.bindings.get(identifier) {
            Some(object) => Some(object.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|environment| environment.borrow().get(identifier)),
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.bindings.insert(name, value);
    }
}
