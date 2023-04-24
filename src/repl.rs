use std::io::{stdin, stdout, Write};

use crate::{error::Error, lexer::Lexer, parser::Parser};

pub fn repl() {
    loop {
        match run() {
            Ok(()) => {}
            Err(error) => error.report(),
        }
    }
}

fn run() -> Result<(), Error> {
    let source = ask_for_input("|> ");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    for statement in program.statements {
        println!("{statement:#?}");
    }
    Ok(())
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
