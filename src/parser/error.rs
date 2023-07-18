use std::fmt;

#[derive(thiserror::Error, fmt::Debug)]
pub enum Error {
    #[error("parsing error")]
    ParserError(bool),
}
