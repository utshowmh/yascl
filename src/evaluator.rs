use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{BlockStatement, Expression, Program, Statement},
    environment::Environment,
    error::Error,
    object::Object,
    token::Token,
};

pub fn evaluate(program: &Program, environment: Rc<RefCell<Environment>>) -> Result<Object, Error> {
    let mut value = Object::Null;
    for statement in &program.statements {
        value = evaluate_statement(statement, Rc::clone(&environment))?;
        if let Object::Return(value) = value {
            return Ok(*value);
        }
    }
    Ok(value)
}

fn evaluate_block_statement(
    statement: &BlockStatement,
    environment: Rc<RefCell<Environment>>,
) -> Result<Object, Error> {
    let mut value = Object::Null;
    for statement in &statement.statements {
        value = evaluate_statement(statement, Rc::clone(&environment))?;
        if let Object::Return(value) = value {
            return Ok(*value);
        }
    }
    Ok(value)
}

fn evaluate_statement(
    statement: &Statement,
    environment: Rc<RefCell<Environment>>,
) -> Result<Object, Error> {
    match statement {
        Statement::Let(name, expression) => {
            let value = evaluate_expression(expression, Rc::clone(&environment))?;
            environment
                .borrow_mut()
                .set(name.to_owned(), value.to_owned());
            Ok(value)
        }
        Statement::Return(Some(expression)) => {
            let value = evaluate_expression(expression, Rc::clone(&environment))?;
            Ok(Object::Return(Box::new(value)))
        }
        Statement::Return(None) => Ok(Object::Return(Box::new(Object::Null))),
        Statement::Expression(expression) => {
            evaluate_expression(expression, Rc::clone(&environment))
        }
    }
}

fn evaluate_expression(
    expression: &Expression,
    environment: Rc<RefCell<Environment>>,
) -> Result<Object, Error> {
    match expression {
        Expression::Identifier(identifier) => {
            environment
                .borrow()
                .get(identifier)
                .ok_or(Error::Runtime(format!(
                    "Name '{identifier}' is not defined"
                )))
        }
        Expression::Integer(value) => Ok(Object::Integer(*value)),
        Expression::Float(value) => Ok(Object::Float(*value)),
        Expression::String(value) => Ok(Object::String(value.to_owned())),
        Expression::Boolean(value) => Ok(Object::Boolean(*value)),
        Expression::Array(expressions) => {
            let mut array = vec![];
            for expression in expressions {
                array.push(evaluate_expression(expression, Rc::clone(&environment))?);
            }
            Ok(Object::Array(array))
        }
        Expression::Hash(pairs) => {
            let mut hash = HashMap::new();
            for (name, value) in pairs {
                let name = evaluate_expression(name, Rc::clone(&environment))?;
                match name {
                    Object::String(name) => {
                        let value = evaluate_expression(value, Rc::clone(&environment))?;
                        hash.insert(name, value);
                    }
                    _ => {
                        return Err(Error::Runtime(format!("Can not use '{name}' as a key")));
                    }
                }
            }
            Ok(Object::Hash(hash))
        }
        Expression::Index(object, index) => {
            let object = evaluate_expression(object, Rc::clone(&environment))?;
            let index = evaluate_expression(index, Rc::clone(&environment))?;
            match (object, index) {
                (Object::Array(array), Object::Integer(index)) => {
                    Ok(array[index as usize].to_owned()) // FIXME: Handle index out of bound.
                }
                (Object::Hash(pairs), Object::String(key)) => {
                    Ok(pairs.get(&key).unwrap().to_owned()) // FIXME: Why unwrap?
                }
                (object, index) => Err(Error::Runtime(format!(
                    "Object '{object}' is not indexable with '{index}'"
                ))),
            }
        }
        Expression::Prefix(operator, right) => {
            let right = evaluate_expression(right, Rc::clone(&environment))?;
            match (operator, right) {
                (Token::Minus, Object::Integer(value)) => Ok(Object::Integer(-value)),
                (Token::Minus, Object::Float(value)) => Ok(Object::Float(-value)),
                (Token::Bang, Object::Boolean(value)) => Ok(Object::Boolean(!value)),
                (operator, right) => Err(Error::Runtime(format!(
                    "Operator '{operator}' is not defined for '{right}'"
                ))),
            }
        }
        Expression::Infix(left, operator, right) => {
            let left = evaluate_expression(left, Rc::clone(&environment))?;
            let right = evaluate_expression(right, Rc::clone(&environment))?;
            match (left, operator, right) {
                (Object::Integer(left), Token::Plus, Object::Integer(right)) => {
                    Ok(Object::Integer(left + right))
                }
                (Object::Float(left), Token::Plus, Object::Float(right)) => {
                    Ok(Object::Float(left + right))
                }
                (Object::String(left), Token::Plus, Object::String(right)) => {
                    Ok(Object::String(left + &right))
                }
                (Object::Integer(left), Token::Minus, Object::Integer(right)) => {
                    Ok(Object::Integer(left - right))
                }
                (Object::Float(left), Token::Minus, Object::Float(right)) => {
                    Ok(Object::Float(left - right))
                }
                (Object::Integer(left), Token::Asterisk, Object::Integer(right)) => {
                    Ok(Object::Integer(left * right))
                }
                (Object::Float(left), Token::Asterisk, Object::Float(right)) => {
                    Ok(Object::Float(left * right))
                }
                (Object::Integer(left), Token::Slash, Object::Integer(right)) => {
                    Ok(Object::Integer(left / right))
                }
                (Object::Float(left), Token::Slash, Object::Float(right)) => {
                    Ok(Object::Float(left / right))
                }
                (Object::Integer(left), Token::Greater, Object::Integer(right)) => {
                    Ok(Object::Boolean(left > right))
                }
                (Object::Float(left), Token::Greater, Object::Float(right)) => {
                    Ok(Object::Boolean(left > right))
                }
                (Object::Integer(left), Token::Lesser, Object::Integer(right)) => {
                    Ok(Object::Boolean(left < right))
                }
                (Object::Float(left), Token::Lesser, Object::Float(right)) => {
                    Ok(Object::Boolean(left < right))
                }
                (left, operator, right) => Err(Error::Runtime(format!(
                    "Operator '{operator}' is not defined for '{left}' and '{right}'"
                ))),
            }
        }
        Expression::If(condition, consequence, alternative) => {
            let condition = evaluate_expression(condition, Rc::clone(&environment))?;
            if condition.is_truthy() {
                evaluate_block_statement(consequence, Rc::clone(&environment))
            } else {
                if let Some(alternative) = alternative {
                    evaluate_block_statement(alternative, Rc::clone(&environment))
                } else {
                    Ok(Object::Null)
                }
            }
        }
        Expression::Function(parameters, body) => Ok(Object::Function(
            parameters.to_owned(),
            body.to_owned(),
            Rc::clone(&environment),
        )),
        Expression::Call(callee, arguments) => {
            let callee = evaluate_expression(callee, Rc::clone(&environment))?;
            let mut args = vec![];
            for argument in arguments {
                let argument = evaluate_expression(argument, Rc::clone(&environment))?;
                args.push(argument);
            }
            match callee {
                Object::Function(parameters, body, environment) => {
                    if parameters.len() != args.len() {
                        Err(Error::Runtime(format!(
                            "Expected {} arguments, got {}",
                            parameters.len(),
                            args.len()
                        )))
                    } else {
                        let local_environment =
                            Rc::new(RefCell::new(Environment::extend(environment)));
                        for i in 0..parameters.len() {
                            local_environment
                                .borrow_mut()
                                .set(parameters[i].to_owned(), args[i].to_owned());
                        }
                        evaluate_block_statement(&body, local_environment)
                    }
                }
                Object::Builtin(func) => func(args),
                object => Err(Error::Runtime(format!("Object '{object}' is not callable"))),
            }
        }
    }
}