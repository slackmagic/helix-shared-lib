use thiserror::Error;

//Define the possible errors
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("NotImplemented")]
    NotImplemented,
    #[error("Creation impossible")]
    CreationImpossible,
    #[error("Another error")]
    AnotherError,
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("Serde Json error: {source}")]
    SerdeJson {
        #[from]
        source: serde_json::Error,
    },
    #[error("Postgres error: {source}")]
    PostGres {
        #[from]
        source: postgres::Error,
    },
    #[error("Tokio Postgres error: {source}")]
    TokioPostGres {
        #[from]
        source: tokio_postgres::Error,
    },
    #[error("Hex error: {source}")]
    Hex {
        #[from]
        source: hex::FromHexError,
    },
}

//Define a generic error type to simplify return.
pub type StorageResult<T> = std::result::Result<T, StorageError>;
