use crate::lexer::Token;

pub enum Statement {
    Let {
        identifier: String,
        expression: Expression,
    },
    Return,
}

pub enum Expression {}

#[derive(Default)]
pub struct AST {
    statements: Vec<Statement>,
}
