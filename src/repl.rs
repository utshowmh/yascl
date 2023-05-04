use std::{
    cell::RefCell,
    env::var,
    io::{stdin, stdout, Write},
    rc::Rc,
};

use crate::{
    common::ast::Program,
    frontend::{lexer::Lexer, parser::Parser},
    runtime::{builtin::get_builtin, environment::Environment, evaluator::evaluate},
};

pub fn run() {
    let username = var("LOGNAME").unwrap_or("anonymous".to_string());
    println!("Hello, {username}! Welcome to YASCL REPL.");
    let mut environment = Rc::new(RefCell::new(get_builtin()));

    loop {
        environment = Rc::new(RefCell::new(Environment::extend(environment)));
        let source = ask_for_input("=> ");
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
            Ok(object) => println!("{object}"),
            Err(error) => error.report(),
        };
    }
}

fn ask_for_input(prompt: &str) -> String {
    let mut source = String::new();
    print!("{prompt}");
    stdout().flush().expect("Failed to flush stdout.");
    for line in stdin().lines() {
        let line = line.expect("Failed to read from stdin.");
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        source.push_str(line);
        print!(".. ");
        stdout().flush().expect("Failed to flush stdout.");
    }
    source
}
