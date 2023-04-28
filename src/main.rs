mod ast;
mod builtin;
mod environment;
mod error;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

use std::env;

use crate::repl::repl;

fn main() {
    let username = env::var("LOGNAME").unwrap_or("anonymous".to_string());
    println!("Hello {username}! Welcome to YASCL REPL.");
    repl();
}
