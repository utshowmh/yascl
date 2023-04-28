use crate::{environment::Environment, error::Error, object::Object};

fn write(objects: Vec<Object>) -> Result<Object, Error> {
    for object in objects {
        print!("{object}");
    }
    println!("");
    Ok(Object::Null)
}

pub fn get_builtin() -> Environment {
    let mut environment = Environment::new();
    environment.set("write".to_string(), Object::Builtin(write));
    environment
}
