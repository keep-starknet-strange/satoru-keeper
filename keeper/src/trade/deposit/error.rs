use thiserror::Error;

#[derive(Error, Debug)]
pub enum DepositError {
    #[error("env variable is not set")]
    EnvVarNotSet(String),
    #[error("Conversion Error")]
    ConversionError(String),
    #[error("Smart Contract Error")]
    SmartContractError(String),
    #[error("Price Error")]
    PriceError(String),
}
