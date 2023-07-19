use std::fmt;

use crate::lexer::Token;

#[derive(thiserror::Error, fmt::Debug)]
pub enum Error {
    #[error("expected {expected}, got {got}")]
    ExpectedToken { expected: Token, got: Token },

    #[error("reached the end of the source")]
    EOF,
}
