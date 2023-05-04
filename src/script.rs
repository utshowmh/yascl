use std::{cell::RefCell, fs::read_to_string, rc::Rc};

use crate::{
    common::ast::Program,
    frontend::{lexer::Lexer, parser::Parser},
    runtime::{builtin::get_builtin, evaluator::evaluate},
};

pub fn run(path: &str) {
    let environment = Rc::new(RefCell::new(get_builtin()));
    let source = read_to_string(path).expect(&format!("Could not read from '{path}'"));
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex().unwrap_or_else(|err| {
        err.report();
        Vec::default()
    });
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap_or_else(|err| {
        err.report();
        Program::default()
    });
    match evaluate(&program, Rc::clone(&environment)) {
        Ok(_) => {}
        Err(error) => error.report(),
    };
}
