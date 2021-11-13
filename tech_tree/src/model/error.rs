use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AddError {
    #[error("Technologies {0:?} form a cycle")]
    Cycle(Vec<String>),
    #[error("Name `{0}` is invalid")]
    InvalidName(String),
    #[error("Name `{0}` already exists")]
    NameExists(String),
    #[error("Predecessor `{0}` is unknown")]
    UnknownPredecessor(String),
}
