use std::io::{stdin, stdout, Write};

use crate::{lexer::Lexer, token::Token};

pub fn start() {
    loop {
        let source = ask_for_input("|> ");
        let mut lexer = Lexer::new(source);
        loop {
            let token = lexer.next_token();
            if let Token::Eof = token {
                break;
            } else {
                println!("{token:?}");
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
