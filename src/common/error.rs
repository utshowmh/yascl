#[derive(Debug)]
pub(crate) enum Error {
    Lexer(String),
    Parser(String),
    Runtime(String),
}

impl Error {
    pub(crate) fn report(&self) {
        match self {
            Error::Lexer(message) => {
                eprintln!("LexerError: {message}.");
            }
            Error::Parser(message) => {
                eprintln!("ParserError: {message}.");
            }
            Error::Runtime(message) => {
                eprintln!("RuntimeError: {message}.")
            }
        }
    }
}
