use std::fmt;
use std::io;

#[derive(thiserror::Error, fmt::Debug)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] io::Error),

    #[error("invalid token \"{0}\"")]
    Token(String),
}
