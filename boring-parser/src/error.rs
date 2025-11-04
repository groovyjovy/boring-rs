use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Failed to parse: {0}")]
    ParseError(String),
}
