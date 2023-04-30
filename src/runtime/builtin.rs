use crate::common::{error::Error, object::Object};

use super::environment::Environment;

fn exit(objects: Vec<Object>) -> Result<Object, Error> {
    let exit_code = match objects.get(0) {
        Some(Object::Integer(exit_code)) => *exit_code as i32,
        _ => 0,
    };
    std::process::exit(exit_code)
}

fn len(objects: Vec<Object>) -> Result<Object, Error> {
    let object = match objects.get(0) {
        Some(Object::Array(array)) => Object::Integer(array.len() as i64),
        Some(Object::String(string)) => Object::Integer(string.len() as i64),
        _ => Object::Null,
    };
    Ok(object)
}

fn first(objects: Vec<Object>) -> Result<Object, Error> {
    let object = match objects.get(0) {
        Some(Object::Array(array)) => array.get(0).unwrap_or(&Object::Null),
        _ => &Object::Null,
    };
    Ok(object.to_owned())
}

fn rest(objects: Vec<Object>) -> Result<Object, Error> {
    let object = match objects.get(0) {
        Some(Object::Array(array)) => match (array.get(1), array.get(array.len() - 1)) {
            (Some(_), Some(_)) => Object::Array(array[1..array.len()].to_vec()),
            (_, _) => Object::Null,
        },
        _ => Object::Null,
    };
    Ok(object)
}

fn append(objects: Vec<Object>) -> Result<Object, Error> {
    let object = match objects.get(0) {
        Some(Object::Array(array)) => {
            if let Some(_) = objects.get(1) {
                let mut array = array.to_owned();
                for object in &objects[1..objects.len()] {
                    array.push(object.to_owned());
                }
                array
            } else {
                array.to_owned()
            }
        }
        _ => vec![],
    };
    Ok(Object::Array(object))
}

pub(crate) fn get_builtin() -> Environment {
    let mut environment = Environment::new();
    environment.set("exit".to_string(), Object::Builtin(exit));
    environment.set("len".to_string(), Object::Builtin(len));
    environment.set("first".to_string(), Object::Builtin(first));
    environment.set("rest".to_string(), Object::Builtin(rest));
    environment.set("append".to_string(), Object::Builtin(append));
    environment
}
