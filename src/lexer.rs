use std::{iter::Peekable, str::Chars};

use crate::token::{lookup_identifier, Token};

pub struct Lexer {
    source: String,
    position: usize,
    character: char,
    characters: Peekable<Chars<'static>>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let chars = unsafe { std::mem::transmute(source.chars().peekable()) };
        let mut lexer = Lexer {
            source,
            position: 0,
            character: '\u{0}',
            characters: chars,
        };
        lexer.read_next_character();
        lexer
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut end_of_file = false;
        while !end_of_file {
            let token = self.next_token();
            if let Token::Eof = token {
                end_of_file = true;
            }
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token: Token;
        match self.character {
            '\u{0}' => token = Token::Eof,
            '=' => {
                if self.peek_char() == '=' {
                    self.read_next_character();
                    token = Token::Equal
                } else {
                    token = Token::Assign
                }
            }
            '+' => token = Token::Plus,
            '-' => token = Token::Minus,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_next_character();
                    token = Token::NotEqual
                } else {
                    token = Token::Bang
                }
            }
            '*' => token = Token::Asterisk,
            '/' => token = Token::Slash,
            '<' => token = Token::Lesser,
            '>' => token = Token::Greater,
            ',' => token = Token::Comma,
            ':' => token = Token::Colon,
            ';' => token = Token::Semicolon,
            '(' => token = Token::LeftParen,
            ')' => token = Token::RightParen,
            '{' => token = Token::LeftBrace,
            '}' => token = Token::RightBrace,
            '[' => token = Token::LeftBracket,
            ']' => token = Token::RightBracket,
            '"' => token = Token::String(self.read_string()),
            character => {
                if is_letter(character) {
                    let identifier = self.read_identifier();
                    return lookup_identifier(identifier);
                } else if is_digit(character) {
                    let integer = self.read_number();
                    if self.character == '.' && is_digit(self.peek_char()) {
                        self.read_next_character();
                        let fraction = self.read_number();
                        return Token::Float(format!("{integer}.{fraction}"));
                    } else {
                        return Token::Integer(integer);
                    }
                } else {
                    token = Token::Illegal
                }
            }
        }

        self.read_next_character();
        token
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.character) {
            self.read_next_character()
        }
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        // The first character is checked to be a letter.
        while is_letter(self.character) || is_digit(self.character) {
            self.read_next_character();
        }
        &self.source[position..self.position]
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.character) {
            self.read_next_character();
        }
        self.source[position..self.position].to_string()
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        // FIXME: check for string termination and report an error in the case of unterminated ones.
        loop {
            self.read_next_character();
            if self.character == '\u{0}' || self.character == '"' {
                break;
            }
        }
        self.source[position..self.position].to_string()
    }

    fn read_next_character(&mut self) {
        self.position += if self.character == '\u{0}' {
            0
        } else {
            self.character.len_utf8()
        };
        self.character = self.characters.next().unwrap_or('\u{0}');
    }

    fn peek_char(&mut self) -> char {
        self.characters.peek().cloned().unwrap_or('\u{0}')
    }
}

fn is_letter(character: char) -> bool {
    character == '_' || ('a'..='z').contains(&character) || ('A'..='Z').contains(&character)
}

fn is_digit(character: char) -> bool {
    ('0'..='9').contains(&character)
}

fn is_whitespace(character: char) -> bool {
    character == ' ' || character == '\t' || character == '\r' || character == '\n'
}
