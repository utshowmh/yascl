use std::fmt::{Display, Formatter, Result};

use crate::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for statement in &self.statements {
            writeln!(f, "{statement}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.statements.is_empty() {
            write!(f, "{{ }}")
        } else {
            writeln!(f, "{{")?;
            for statement in &self.statements {
                writeln!(f, "{statement}")?;
            }
            writeln!(f, "}}")
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Let(String, Expression),
    Return(Option<Expression>),
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Statement::Let(identifier, value) => write!(f, "let {identifier} = {value};"),
            Statement::Return(None) => write!(f, "return;"),
            Statement::Return(Some(value)) => write!(f, "return {value};"),
            Statement::Expression(value) => write!(f, "{value};"),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Expression>),
    Hash(Vec<(Expression, Expression)>),
    Index(Box<Expression>, Box<Expression>),
    Prefix(Token, Box<Expression>),
    Infix(Box<Expression>, Token, Box<Expression>),
    If(Box<Expression>, BlockStatement, Option<BlockStatement>),
    Function(Vec<String>, BlockStatement),
    Call(Box<Expression>, Vec<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expression::Identifier(identifier) => write!(f, "{identifier}"),
            Expression::Integer(value) => write!(f, "{value}"),
            Expression::Float(value) => write!(f, "{value}"),
            Expression::String(value) => write!(f, "{value}"),
            Expression::Boolean(value) => write!(f, "{value}"),
            Expression::Array(values) => write!(f, "{}", comma_separated_values(values)),
            Expression::Hash(pairs) => write!(f, "{}", comma_separated_pairs(pairs)),
            Expression::Index(left, index) => write!(f, "({left}[{index}])"),
            Expression::Prefix(operator, right) => write!(f, "{operator} {right}"),
            Expression::Infix(left, operator, right) => write!(f, "{left} {operator} {right}"),
            Expression::If(condition, consequence, alternative) => {
                write!(f, "if {condition} {consequence}")?;
                if let Some(alternative) = alternative {
                    write!(f, "else {alternative}")?;
                }
                Ok(())
            }
            Expression::Function(parameters, body) => {
                write!(f, "fun({}) {body}", parameters.join(","))
            }
            Expression::Call(function, arguments) => {
                write!(f, "{function}({})", comma_separated_values(arguments))
            }
        }
    }
}

fn comma_separated_values(values: &[Expression]) -> String {
    values
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn comma_separated_pairs(pairs: &[(Expression, Expression)]) -> String {
    pairs
        .iter()
        .map(|pair| format!("{}: {}", pair.0, pair.1))
        .collect::<Vec<String>>()
        .join(",")
}
