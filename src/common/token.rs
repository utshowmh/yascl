use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    Eof,

    Identifier(String),
    Integer(String),
    Float(String),
    String(String),

    Assign,

    Spread,

    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lesser,
    LesserOrEqual,
    Greater,
    GreaterOrEqual,

    Equal,
    NotEqual,

    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,

    Comma,
    Colon,
    Dot,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Let,
    Mut,
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

            Token::Spread => write!(f, ".."),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),

            Token::Lesser => write!(f, "<"),
            Token::LesserOrEqual => write!(f, "<="),
            Token::Greater => write!(f, ">"),
            Token::GreaterOrEqual => write!(f, ">="),

            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),

            Token::Ampersand => write!(f, "&"),
            Token::AmpersandAmpersand => write!(f, "&&"),
            Token::Pipe => write!(f, "|"),
            Token::PipePipe => write!(f, "||"),

            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Dot => write!(f, "."),

            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),

            Token::Let => write!(f, "let"),
            Token::Mut => write!(f, "mut"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
        }
    }
}

pub(crate) fn lookup_identifier(identifier: &str) -> Token {
    match identifier {
        "let" => Token::Let,
        "mut" => Token::Mut,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Identifier(identifier.to_owned()),
    }
}
