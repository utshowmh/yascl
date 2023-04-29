use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Eof,

    Identifier(String),
    Integer(String),
    Float(String),
    String(String),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lesser,
    Greater,

    Equal,
    NotEqual,

    Comma,
    Colon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Let,
    Function,
    If,
    Else,
    Return,
    True,
    False,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Eof => write!(f, "EndOfFile"),

            Token::Identifier(identifier) => write!(f, "{identifier}"),
            Token::Integer(int) => write!(f, "{int}"),
            Token::Float(float) => write!(f, "{float}"),
            Token::String(s) => write!(f, "\"{s}\""),

            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),

            Token::Lesser => write!(f, "<"),
            Token::Greater => write!(f, ">"),

            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),

            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),

            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),

            Token::Function => write!(f, "fun"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
        }
    }
}

pub fn lookup_identifier(identifier: &str) -> Token {
    match identifier {
        "let" => Token::Let,
        "fun" => Token::Function,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Identifier(identifier.to_owned()),
    }
}
