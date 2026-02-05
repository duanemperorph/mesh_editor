use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshDocumentError {
    #[error("Failed to read mesh file: {path}")]
    ReadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write mesh file: {path}")]
    WriteError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse mesh file: {path}")]
    ParseError {
        path: PathBuf,
        #[source]
        source: ron::error::SpannedError,
    },

    #[error("Failed to serialize mesh")]
    SerializeError(#[from] ron::Error),

    #[error("Directory does not exist: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("Version {0} not found")]
    VersionNotFound(i32),
}
