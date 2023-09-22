//! Definition of custom error types for the crate.
use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeeperError {
    #[error("invalid keeper configuration: {0}")]
    ConfigError(String),
    #[error("an error occurred while handling an event")]
    EventHandlingError,
    #[error("an error occurred while submitting a transaction: {0}")]
    StarknetTransactionError(String),
}

/// Convert a ConfigError into a KeeperError.
impl From<ConfigError> for KeeperError {
    fn from(e: ConfigError) -> Self {
        Self::ConfigError(e.to_string())
    }
}
