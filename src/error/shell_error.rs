use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("unterminated {quote_char} quote starting at position {pos}")]
    UnterminatedQuote {
        pos: usize,
        quote_char: char,
    },

    #[error("unexpected token '{token}' at position {pos}")]
    UnexpectedToken {
        token: String,
        pos: usize,
    },

    #[error("empty command")]
    EmptyCommand,
}