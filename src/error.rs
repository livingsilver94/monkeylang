use std::fmt;
use std::io;

use crate::lexer;
use crate::parser;

#[derive(thiserror::Error, fmt::Debug)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] io::Error),

    #[error("invalid token \"{0}\"")]
    Lexing(#[from] lexer::Error),

    #[error("I/O error")]
    Parsing(#[from] parser::Error),
}
