use std::fmt::{Display, Formatter, Result};

use super::token::Token;

#[derive(Debug, Default)]
pub(crate) struct Program {
    pub(crate) statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for statement in &self.statements {
            writeln!(f, "{statement}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Statement {
    Let(String, Expression),
    Mut(String, Expression),
    Return(Expression),
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Statement::Let(identifier, value) => write!(f, "let {identifier} = {value}"),
            Statement::Mut(identifier, value) => write!(f, "mut {identifier} = {value}"),
            Statement::Return(value) => write!(f, "return {value}"),
            Statement::Expression(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Expression {
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
    Block(Vec<Statement>),
    If(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
    Function(Vec<String>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expression::Identifier(name) => write!(f, "({name})"),
            Expression::Integer(value) => write!(f, "({value})"),
            Expression::Float(value) => write!(f, "({value})"),
            Expression::String(value) => write!(f, "({value})"),
            Expression::Boolean(value) => write!(f, "({value})"),
            Expression::Array(values) => write!(f, "([{}])", comma_separated_values(values)),
            Expression::Hash(pairs) => write!(f, "({{{}}})", comma_separated_pairs(pairs)),
            Expression::Index(left, index) => write!(f, "({left}[{index}])"),
            Expression::Prefix(operator, right) => write!(f, "({operator} {right})"),
            Expression::Infix(left, operator, right) => write!(f, "({left} {operator} {right})"),
            Expression::Block(_) => write!(f, "{{}}"),
            Expression::If(condition, consequence, alternative) => {
                write!(f, "(if {condition} {consequence}")?;
                if let Some(alternative) = alternative {
                    write!(f, "else {alternative})")
                } else {
                    write!(f, ")")
                }
            }
            Expression::Function(parameters, body) => {
                write!(f, "(fun({}) {body})", parameters.join(", "))
            }
            Expression::Call(function, arguments) => {
                write!(f, "({function}({}))", comma_separated_values(arguments))
            }
        }
    }
}

fn comma_separated_values(values: &[Expression]) -> String {
    values
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

fn comma_separated_pairs(pairs: &[(Expression, Expression)]) -> String {
    pairs
        .iter()
        .map(|pair| format!("{}: {}", pair.0, pair.1))
        .collect::<Vec<String>>()
        .join(", ")
}
