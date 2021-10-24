use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AddError {
    #[error("Name `{0}` is invalid")]
    InvalidName(String),
}
