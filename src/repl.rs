use std::io::{stdin, stdout, Write};

use crate::{lexer::Lexer, parser::Parser};

pub fn start() {
    loop {
        let source = ask_for_input("|> ");
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        let errors = parser.errors();
        if errors.is_empty() {
            for statement in program.statements {
                println!("{statement:#?}");
            }
        } else {
            for error in errors {
                eprintln!("{}", error.report());
            }
        }
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
