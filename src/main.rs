mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

use std::env;

use crate::repl::start;

fn main() {
    let username = env::var("LOGNAME").unwrap_or("anonymous".to_string());
    println!("Hello {username}! Welcome to ASCL REPL.");
    start();
}
