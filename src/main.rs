use std::env::args;

use yascl::{repl, script};

const USAGE: &str = "
Program: YASCL.

help            : print help (this page).
yascl           : run repl.
yascl [path]    : run script from the path.
";

fn main() {
    let args = args().collect::<Vec<_>>();
    match args.len() {
        1 => repl::run(),
        2 => match args[1].as_str() {
            "help" => println!("{USAGE}"),
            path => script::run(path),
        },
        _ => eprintln!("{USAGE}"),
    }
}
