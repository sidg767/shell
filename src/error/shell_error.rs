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

    #[error("unexpected end of input: {context}")]
    UnexpectedEof {
        context: &'static str,
    },

    #[error("syntax error near '{token}'")]
    SyntaxError {
        token: String,
    },

    #[error("invalid redirect: {reason}")]
    InvalidRedirect {
        reason: &'static str,
    },

    #[error("invalid file descriptor: '{fd}'")]
    InvalidFileDescriptor {
        fd: String,
    },

     #[error("undefined variable '{name}'")]
    UndefinedVariable {
        name: String,
    },

    #[error("bad substitution: '{expr}'")]
    BadSubstitution {
        expr: String,
    },

    #[error("word expansion failed: {reason}")]
    ExpansionFailed {
        reason: String,
    },

    #[error("command not found: '{name}'")]
    CommandNotFound {
        name: String,
    },

    #[error("permission denied: '{path}'")]
    PermissionDenied {
        path: String,
    },

    #[error("is a directory: '{path}'")]
    IsADirectory {
        path: String,
    },

    #[error("command failed with exit code {code}")]
    ExitCode {
        code: i32,
    },

    #[error("process killed by signal {signal}")]
    Signal {
        signal: i32,
    },

     #[error("cannot open '{path}': {source}")]
    OpenFile {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("pipe failed: {source}")]
    PipeFailed {
        #[source]
        source: std::io::Error,
    },

    #[error("fork failed: {source}")]
    ForkFailed {
        #[source]
        source: std::io::Error,
    },

     #[error("{builtin}: {message}")]
    Builtin {
        builtin: &'static str,
        message: String,
    },

    #[error("{builtin}: too many arguments")]
    TooManyArgs {
        builtin: &'static str,
    },

    #[error("{builtin}: missing argument")]
    MissingArgument {
        builtin: &'static str,
    },

    #[error("no such job: {job_id}")]
    NoSuchJob {
        job_id: usize,
    },

    #[error("job control not available in this context")]
    JobControlUnavailable

    #[error("invalid identifier: '{name}'")]
    InvalidIdentifier {
        name: String,
    },

    #[error("readonly variable: '{name}'")]
    ReadonlyVariable {
        name: String,
    },
    
     #[error("io error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("internal error: {message}")]
    Internal {
        message: String,
    },
}
