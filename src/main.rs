use std::env;

use yascl::repl::run;

fn main() {
    let username = env::var("LOGNAME").unwrap_or("anonymous".to_string());
    println!("Hello {username}! Welcome to YASCL REPL.");
    run();
}
