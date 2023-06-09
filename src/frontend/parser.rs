use crate::common::{
    ast::{Expression, Program, Statement},
    error::Error,
    token::Token,
};

pub(crate) struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub(crate) fn parse_program(&mut self) -> Result<Program, Error> {
        let mut statements = vec![];
        while !self.end_of_tokens() {
            statements.push(self.parse_statement()?);
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.current_token() {
            Token::Let => self.parse_let_statement(),
            Token::Mut => self.parse_mut_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, Error> {
        self.expect_token(Token::Let)?;
        if let Token::Identifier(identifier) = self.current_token().to_owned() {
            self.advance_position();
            self.expect_token(Token::Assign)?;
            let value = self.parse_expression()?;
            Ok(Statement::Let(identifier, value))
        } else {
            Err(Error::Parser(format!(
                "Unexpected token '{}', expected IDENTIFIER",
                self.current_token(),
            )))
        }
    }

    fn parse_mut_statement(&mut self) -> Result<Statement, Error> {
        self.expect_token(Token::Mut)?;
        if let Token::Identifier(identifier) = self.current_token().to_owned() {
            self.advance_position();
            self.expect_token(Token::Assign)?;
            let value = self.parse_expression()?;
            Ok(Statement::Mut(identifier, value))
        } else {
            Err(Error::Parser(format!(
                "Unexpected token '{}', expected IDENTIFIER",
                self.current_token(),
            )))
        }
    }

    fn parse_return_statement(&mut self) -> Result<Statement, Error> {
        self.expect_token(Token::Return)?;
        let value = self.parse_expression()?;
        Ok(Statement::Return(value))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let value = self.parse_expression()?;
        Ok(Statement::Expression(value))
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_infix_expression()
    }

    fn parse_infix_expression(&mut self) -> Result<Expression, Error> {
        self.parse_logical_or_expression()
    }

    fn parse_logical_or_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_logical_and_expression()?;
        while Token::eq(self.current_token(), &Token::PipePipe) {
            let operator = self.next_token();
            let right = self.parse_logical_and_expression()?;
            left = Expression::Infix(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn parse_logical_and_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_bitwise_or_expression()?;
        while Token::eq(self.current_token(), &Token::AmpersandAmpersand) {
            let operator = self.next_token();
            let right = self.parse_bitwise_or_expression()?;
            left = Expression::Infix(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn parse_bitwise_or_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_bitwise_and_expression()?;
        while Token::eq(self.current_token(), &Token::Pipe) {
            let operator = self.next_token();
            let right = self.parse_bitwise_and_expression()?;
            left = Expression::Infix(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn parse_bitwise_and_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_equality_expression()?;
        while Token::eq(self.current_token(), &Token::Ampersand) {
            let operator = self.next_token();
            let right = self.parse_equality_expression()?;
            left = Expression::Infix(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn parse_equality_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_comparison_expression()?;
        while self.current_token_in(&[Token::NotEqual, Token::Equal]) {
            let operator = self.next_token();
            let right = self.parse_comparison_expression()?;
            left = Expression::Infix(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn parse_comparison_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_spread_expression()?;
        while self.current_token_in(&[
            Token::Lesser,
            Token::LesserOrEqual,
            Token::Greater,
            Token::GreaterOrEqual,
        ]) {
            let operator = self.next_token();
            let right = self.parse_spread_expression()?;
            left = Expression::Infix(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn parse_spread_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_term_expression()?;
        while Token::eq(self.current_token(), &Token::Spread) {
            let operator = self.next_token();
            let right = self.parse_term_expression()?;
            left = Expression::Infix(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn parse_term_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_factor_expression()?;
        while self.current_token_in(&[Token::Plus, Token::Minus]) {
            let operator = self.next_token();
            let right = self.parse_factor_expression()?;
            left = Expression::Infix(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn parse_factor_expression(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_prefix_expression()?;
        while self.current_token_in(&[Token::Asterisk, Token::Slash]) {
            let operator = self.next_token();
            let right = self.parse_prefix_expression()?;
            left = Expression::Infix(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, Error> {
        if self.current_token_in(&[Token::Bang, Token::Minus]) {
            let operator = self.next_token();
            let right = self.parse_prefix_expression()?;
            Ok(Expression::Prefix(operator, Box::new(right)))
        } else {
            self.parse_call_expression()
        }
    }

    fn parse_call_expression(&mut self) -> Result<Expression, Error> {
        let expression = self.parse_index_expression()?;
        match self.current_token() {
            Token::LeftParen => {
                self.advance_position();
                let mut arguments = vec![];
                if Token::ne(self.current_token(), &Token::RightParen) {
                    loop {
                        arguments.push(self.parse_expression()?);
                        if Token::eq(self.current_token(), &Token::Comma) {
                            self.advance_position();
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                self.expect_token(Token::RightParen)?;
                Ok(Expression::Call(Box::new(expression), arguments))
            }
            _ => Ok(expression),
        }
    }

    fn parse_index_expression(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_literal_expression()?;
        while Token::eq(&Token::LeftBracket, self.current_token()) {
            self.advance_position();
            let index = self.parse_expression()?;
            self.expect_token(Token::RightBracket)?;
            expression = Expression::Index(Box::new(expression), Box::new(index));
        }
        Ok(expression)
    }

    fn parse_literal_expression(&mut self) -> Result<Expression, Error> {
        match self.current_token().to_owned() {
            Token::PipePipe => {
                self.advance_position();
                let body = self.parse_expression()?;
                Ok(Expression::Function(vec![], Box::new(body)))
            }
            Token::Pipe => {
                self.advance_position();
                let mut parameters = vec![];
                if Token::ne(self.current_token(), &Token::Pipe) {
                    loop {
                        if let Token::Identifier(identifier) = self.current_token().to_owned() {
                            self.advance_position();
                            parameters.push(identifier)
                        } else {
                            return Err(Error::Parser(format!(
                                "Unexpected token '{}', expected IDENTIFIER",
                                self.current_token(),
                            )));
                        }
                        if Token::eq(self.current_token(), &Token::Comma) {
                            self.advance_position();
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                self.expect_token(Token::Pipe)?;
                let body = self.parse_expression()?;
                Ok(Expression::Function(parameters, Box::new(body)))
            }
            Token::If => {
                self.advance_position();
                let condition = self.parse_expression()?;
                let consequence = self.parse_expression()?;
                let mut alternative = None;
                if Token::Else.eq(self.current_token()) {
                    self.advance_position();
                    alternative = Some(self.parse_expression()?);
                }
                Ok(Expression::If(
                    Box::new(condition),
                    Box::new(consequence),
                    alternative.map(|expression| Box::new(expression)),
                ))
            }
            Token::LeftBrace => {
                let mut statements = vec![];
                self.expect_token(Token::LeftBrace)?;
                while Token::RightBrace.ne(self.current_token())
                    && Token::Eof.ne(self.current_token())
                {
                    statements.push(self.parse_statement()?);
                }
                self.expect_token(Token::RightBrace)?;
                Ok(Expression::Block(statements))
            }
            Token::Hash => {
                let mut pairs = vec![];
                self.advance_position();
                self.expect_token(Token::LeftBracket)?;
                if Token::ne(self.current_token(), &Token::RightBracket) {
                    loop {
                        let key = self.parse_expression()?;
                        self.expect_token(Token::Colon)?;
                        let value = self.parse_expression()?;
                        pairs.push((key, value));
                        if Token::eq(self.current_token(), &Token::Comma) {
                            self.advance_position();
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                self.expect_token(Token::RightBracket)?;
                Ok(Expression::Hash(pairs))
            }
            Token::Array => {
                let mut expressions = vec![];
                self.advance_position();
                self.expect_token(Token::LeftBracket)?;
                if Token::ne(self.current_token(), &Token::RightBracket) {
                    loop {
                        expressions.push(self.parse_expression()?);
                        if Token::eq(self.current_token(), &Token::Comma) {
                            self.advance_position();
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                self.expect_token(Token::RightBracket)?;
                Ok(Expression::Array(expressions))
            }
            Token::LeftParen => {
                self.advance_position();
                let expression = self.parse_expression()?;
                self.expect_token(Token::RightParen)?;
                Ok(expression)
            }
            Token::True => {
                self.advance_position();
                Ok(Expression::Boolean(true))
            }
            Token::False => {
                self.advance_position();
                Ok(Expression::Boolean(false))
            }
            Token::Integer(integer) => {
                self.advance_position();
                Ok(Expression::Integer(integer.parse().unwrap()))
            }
            Token::Float(float) => {
                self.advance_position();
                Ok(Expression::Float(float.parse().unwrap()))
            }
            Token::String(string) => {
                self.advance_position();
                Ok(Expression::String(string))
            }
            token => {
                if let Token::Identifier(identifier) = token {
                    self.advance_position();
                    Ok(Expression::Identifier(identifier))
                } else {
                    Err(Error::Parser(format!(
                        "Unexpected token '{}', expected IDENTIFIER",
                        self.current_token(),
                    )))
                }
            }
        }
    }

    fn peek(&self, offset: usize) -> &Token {
        let index = self.position + offset;
        if index < self.tokens.len() {
            &self.tokens[index]
        } else {
            &Token::Eof
        }
    }

    fn current_token(&self) -> &Token {
        self.peek(0)
    }

    fn end_of_tokens(&self) -> bool {
        Token::Eof.eq(self.current_token())
    }

    fn advance_position(&mut self) {
        self.position += 1;
    }

    fn next_token(&mut self) -> Token {
        let token = self.current_token().to_owned();
        self.advance_position();
        token
    }

    fn current_token_in(&self, tokens: &[Token]) -> bool {
        tokens.contains(self.current_token())
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), Error> {
        if expected.eq(self.current_token()) {
            self.advance_position();
            Ok(())
        } else {
            Err(Error::Parser(format!(
                "Unexpected token '{}', expected '{}'",
                self.current_token(),
                expected
            )))
        }
    }
}
