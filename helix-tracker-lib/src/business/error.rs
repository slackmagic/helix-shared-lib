use crate::storage::error::StorageError;
use std::result::Result;
use thiserror::Error;

//Define the possible errors
#[derive(Error, Debug)]
pub enum TrackerDomainError {
    #[error("NotImplemented")]
    NotImplemented,
    #[error("Storage Error")]
    StorageError,
    #[error("Not found error")]
    NotFoundError,
    #[error("Storage error: {source}")]
    Storage {
        #[from]
        source: StorageError,
    },
}

//Define a generic error type to simplify return.
pub type TrackerDomainResult<T> = Result<T, TrackerDomainError>;
