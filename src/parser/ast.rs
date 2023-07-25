use crate::lexer::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    /// An expression statement. Although it may sound confusing,
    /// a statement like `5 + 10;` is legal in Monkey. Furthermore,
    /// we need a statement wrapper around an expression for the parsing logic.
    Expression(Expression),
    Let {
        identifier: String,
        expression: Expression,
    },
    Return(Expression),
}

pub enum Priority {
    Lowest,
    Equals,
    LessOrGreaterThan,
    Sum,
    Product,
    Unary,
    Call,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    None,
    Identifier(String),
    Integer(i64),
    Unary {
        operator: Token,
        expression: Box<Expression>,
    },
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
