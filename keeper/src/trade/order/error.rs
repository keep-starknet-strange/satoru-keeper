use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderError {
    #[error("env variable is not set")]
    EnvVarNotSet(String),
    #[error("Conversion Error")]
    ConversionError(String),
    #[error("Smart Contract Error")]
    SmartContractError(String),
    #[error("Price Error")]
    PriceError(String),
}
