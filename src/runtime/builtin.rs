use crate::common::{error::Error, object::Object};

use super::environment::Environment;

fn exit(objects: Vec<Object>) -> Result<Object, Error> {
    let exit_code = match objects.get(0) {
        Some(Object::Integer(exit_code)) => *exit_code as i32,
        _ => 0,
    };
    std::process::exit(exit_code)
}

pub(crate) fn get_builtin() -> Environment {
    let mut environment = Environment::new();
    environment.set("exit".to_string(), Object::Builtin(exit));
    environment
}
