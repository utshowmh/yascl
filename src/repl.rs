use std::{
    cell::RefCell,
    io::{stdin, stdout, Write},
    rc::Rc,
};

use crate::{
    ast::Program, builtin::get_builtin, environment::Environment, evaluator::evaluate,
    lexer::Lexer, parser::Parser,
};

pub fn repl() {
    let mut environment = Rc::new(RefCell::new(get_builtin()));

    loop {
        environment = Rc::new(RefCell::new(Environment::extend(environment)));
        let source = ask_for_input("yascl => ");
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().unwrap_or_else(|err| {
            err.report();
            Vec::new()
        });
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap_or_else(|err| {
            err.report();
            Program {
                statements: Vec::new(),
            }
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
    stdin()
        .read_line(&mut source)
        .expect("Failed to read from stdin.");
    source
}
