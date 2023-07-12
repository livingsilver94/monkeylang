use std::iter::Peekable;

use self::ast::AST;
use crate::lexer::Token;

pub mod ast;

pub struct Parser<T: Iterator<Item = Token>> {
    tokens: Peekable<T>,
}

impl<T: Iterator<Item = Token>> Parser<T> {
    pub fn new(it: T) -> Self {
        Self {
            tokens: it.peekable(),
        }
    }

    pub fn parse(&mut self) -> AST {
        AST::default()
    }
}
