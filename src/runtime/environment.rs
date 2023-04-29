use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::common::object::Object;

#[derive(Debug)]
pub(crate) struct Environment {
    bindings: HashMap<String, Object>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub(crate) fn new() -> Environment {
        Environment {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    pub(crate) fn extend(parent: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            bindings: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub(crate) fn get(&self, identifier: &str) -> Option<Object> {
        match self.bindings.get(identifier) {
            Some(object) => Some(object.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|environment| environment.borrow().get(identifier)),
        }
    }

    pub(crate) fn set(&mut self, name: String, value: Object) {
        self.bindings.insert(name, value);
    }
}
