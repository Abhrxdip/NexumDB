mod engine;
mod error;

pub use engine::StorageEngine;
pub use error::StorageError;

pub type Result<T> = std::result::Result<T, StorageError>;
