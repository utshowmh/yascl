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

    pub(crate) fn get(&self, name: &str) -> Option<Object> {
        match self.bindings.get(name) {
            Some(object) => Some(object.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|environment| environment.borrow().get(name)),
        }
    }

    pub(crate) fn set(&mut self, name: String, value: Object) {
        self.bindings.insert(name, value);
    }

    pub(crate) fn mutate(&mut self, name: &str, value: Object) -> Option<Object> {
        match self.bindings.get(name) {
            Some(_) => {
                self.set(name.to_owned(), value.to_owned());
                Some(value)
            }
            None => self
                .parent
                .as_deref()
                .and_then(|environment| environment.borrow_mut().mutate(name, value)),
        }
    }
}
