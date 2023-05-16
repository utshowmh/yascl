use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::common::{
    ast::{BlockStatement, Expression, Program, Statement},
    error::Error,
    object::Object,
    token::Token,
};

use super::environment::Environment;

pub(crate) fn evaluate(
    program: &Program,
    environment: Rc<RefCell<Environment>>,
) -> Result<Object, Error> {
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
        Statement::Return(expression) => {
            let value = evaluate_expression(expression, Rc::clone(&environment))?;
            Ok(Object::Return(Box::new(value)))
        }
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
                    let last_index = array.len() as i64 - 1;
                    if index >= 0 && index <= last_index {
                        Ok(array[index as usize].to_owned())
                    } else {
                        Err(Error::Runtime(format!("Index '{index}' not valid")))
                    }
                }
                (Object::Array(array), Object::Range(from, to)) => {
                    let last_index = array.len() as i64;
                    if from >= 0 && from <= last_index && to >= 0 && to <= last_index {
                        Ok(Object::Array(array[from as usize..to as usize].to_vec()))
                    } else {
                        Err(Error::Runtime(format!(
                            "Index '{}' not valid",
                            Object::Range(from, to)
                        )))
                    }
                }
                (Object::Hash(pairs), Object::String(key)) => {
                    if let Some(object) = pairs.get(&key) {
                        Ok(object.to_owned())
                    } else {
                        Err(Error::Runtime(format!("Key '{key}' not valid",)))
                    }
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
                (Object::Integer(from), Token::Spread, Object::Integer(to)) => {
                    Ok(Object::Range(from, to))
                }
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
                (Object::Integer(left), Token::Lesser, Object::Integer(right)) => {
                    Ok(Object::Boolean(left < right))
                }
                (Object::Float(left), Token::Lesser, Object::Float(right)) => {
                    Ok(Object::Boolean(left < right))
                }
                (Object::Integer(left), Token::LesserOrEqual, Object::Integer(right)) => {
                    Ok(Object::Boolean(left <= right))
                }
                (Object::Float(left), Token::LesserOrEqual, Object::Float(right)) => {
                    Ok(Object::Boolean(left <= right))
                }
                (Object::Integer(left), Token::Greater, Object::Integer(right)) => {
                    Ok(Object::Boolean(left > right))
                }
                (Object::Float(left), Token::Greater, Object::Float(right)) => {
                    Ok(Object::Boolean(left > right))
                }
                (Object::Integer(left), Token::GreaterOrEqual, Object::Integer(right)) => {
                    Ok(Object::Boolean(left >= right))
                }
                (Object::Float(left), Token::GreaterOrEqual, Object::Float(right)) => {
                    Ok(Object::Boolean(left >= right))
                }
                (Object::Integer(left), Token::Ampersand, Object::Integer(right)) => {
                    Ok(Object::Integer(left & right))
                }
                (Object::Integer(left), Token::Pipe, Object::Integer(right)) => {
                    Ok(Object::Integer(left | right))
                }
                (left, Token::Equal, right) => Ok(Object::Boolean(left.equal(&right))),
                (left, Token::NotEqual, right) => Ok(Object::Boolean(!left.equal(&right))),
                (left, Token::AmpersandAmpersand, right) => {
                    Ok(Object::Boolean(left.is_truthy() && right.is_truthy()))
                }
                (left, Token::PipePipe, right) => {
                    Ok(Object::Boolean(left.is_truthy() || right.is_truthy()))
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
            let mut _arguments = vec![];
            for argument in arguments {
                let argument = evaluate_expression(argument, Rc::clone(&environment))?;
                _arguments.push(argument);
            }
            match callee {
                Object::Function(parameters, body, environment) => {
                    if parameters.len() != _arguments.len() {
                        Err(Error::Runtime(format!(
                            "Expected {} argument(s), got {}",
                            parameters.len(),
                            _arguments.len()
                        )))
                    } else {
                        let local_environment =
                            Rc::new(RefCell::new(Environment::extend(environment)));
                        for i in 0..parameters.len() {
                            local_environment
                                .borrow_mut()
                                .set(parameters[i].to_owned(), _arguments[i].to_owned());
                        }
                        evaluate_block_statement(&body, local_environment)
                    }
                }
                Object::Builtin(func) => func(_arguments),
                object => Err(Error::Runtime(format!("Object '{object}' is not callable"))),
            }
        }
    }
}
