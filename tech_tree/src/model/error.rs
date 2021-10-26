use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AddError {
    #[error("Name `{0}` is invalid")]
    InvalidName(String),
    #[error("Name `{0}` already exists")]
    NameExists(String),
    #[error("Predecessor `{0}` is unknown")]
    UnknownPredecessor(String),
}
