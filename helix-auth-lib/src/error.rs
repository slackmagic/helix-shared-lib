use std::result::Result;
use thiserror::Error;

//Define the possible errors
#[derive(Error, Debug)]
pub enum HelixAuthError {
    #[error("Token invalid")]
    InvalidToken,
    #[error("Not found error")]
    NotFoundError,
}

//Define a generic error type to simplify return.
pub type HelixAuthResult<T> = Result<T, HelixAuthError>;
