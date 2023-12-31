use std::fmt::Debug;

use crate::lexer::Token;

#[derive(PartialEq, Eq)]
pub enum Statement {
    /// An expression statement. Although it may sound confusing,
    /// a statement like `5 + 10;` is legal in Monkey.
    Expression(Expression),

    Let {
        identifier: String,
        expression: Expression,
    },
    Return(Expression),
}

impl Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Expression(exp) => write!(f, "{:?}", exp),
            Statement::Let { identifier, expression } => write!(f, "{} {} = {:?}", Token::Let, identifier, expression),
            Statement::Return(exp) => write!(f, "{} {:?}", Token::Return, exp),
        }
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum Priority {
    Lowest,
    Equals,
    LessOrGreaterThan,
    Sum,
    Product,
    Unary,
    Call,
}

impl Priority {
    /// Returns the Token's priority in an expression.
    pub fn from_token(tok: &Token) -> Self {
        match tok {
            Token::Asterisk | Token::Slash => Self::Product,
            Token::LessThan | Token::GreaterThan => Self::LessOrGreaterThan,
            Token::Plus | Token::Minus => Self::Sum,
            Token::Equal | Token::NotEqual => Self::Equals,
            _ => Self::Lowest,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Expression {
    None,
    Boolean(bool),
    Identifier(String),
    Integer(i64),
    Unary {
        operator: Token,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    If {
        cond: Box<Expression>,
        conseq: Vec<Statement>,
        altern: Option<Vec<Statement>>,
    },
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        match self {
            Expression::None => Ok(()),
            Expression::Boolean(b) => write!(f, "{}", b),
            Expression::Identifier(name) => write!(f, "{}", name),
            Expression::Integer(int) => write!(f, "{}", int),
            Expression::Unary { operator, expression } => write!(f, "{}{:?}", operator, expression),
            Expression::Binary { left, operator, right } => write!(f, "{:?}{}{:?}", left, operator, right),
            Expression::If { cond, conseq, altern } => {
                write!(f, "{} {:?} ", Token::If, cond)?;
                for st in conseq {
                    write!(f, "{:?}", st)?;
                }
                if let Some(alt) = altern {
                    write!(f, " {} ", Token::Else)?;
                    for st in alt {
                        write!(f, "{:?}", st)?;
                    }
                }
                Ok(())
            }
        }?;
        write!(f, ")")
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct AST {
    statements: Vec<Statement>,
}

impl AST {
    pub fn new(st: Vec<Statement>) -> Self {
        Self { statements: st }
    }

    pub fn push(&mut self, st: Statement) {
        self.statements.push(st);
    }
}
