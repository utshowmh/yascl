use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::runtime::environment::Environment;

use super::{ast::Expression, error::Error};

#[derive(Debug, Clone)]
pub(crate) enum Object {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Range(i64, i64),
    Array(Vec<Object>),
    Hash(HashMap<String, Object>),
    Return(Box<Object>),
    Function(Vec<String>, Expression, Rc<RefCell<Environment>>),
    Builtin(BuiltinFunction),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Boolean(value) => write!(f, "{value}"),
            Object::Integer(value) => write!(f, "{value}"),
            Object::Float(value) => write!(f, "{value}"),
            Object::String(value) => write!(f, "{value}"),
            Object::Range(from, to) => write!(f, "{from}..{to}"),
            Object::Array(values) => {
                let values = values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{values}]")
            }
            Object::Hash(pairs) => {
                let pairs = pairs
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<String>>();
                write!(f, "{{{}}}", pairs.join(", "))
            }
            Object::Return(value) => write!(f, "{}", *value),
            Object::Function(params, body, _) => {
                write!(f, "<function({}) {}>", params.join(", "), body)
            }
            Object::Builtin(_) => write!(f, "<builtin function>"),
        }
    }
}

impl Object {
    pub(crate) fn equal(&self, other: &Object) -> bool {
        match (self, other) {
            (Object::Null, Object::Null) => true,
            (Object::Boolean(x), Object::Boolean(y)) => x == y,
            (Object::Integer(x), Object::Integer(y)) => x == y,
            (Object::Float(x), Object::Float(y)) => x == y,
            (Object::String(x), Object::String(y)) => x == y,
            _ => false,
        }
    }

    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Object::Null => false,
            Object::Boolean(bool) => *bool,
            _ => true,
        }
    }
}

pub(crate) type BuiltinFunction = fn(Vec<Object>) -> Result<Object, Error>;
