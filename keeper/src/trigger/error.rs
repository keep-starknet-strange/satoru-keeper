use thiserror::Error;

#[derive(Error, Debug)]
pub enum TriggerError {
    #[error("Failed to call is_triggerable")]
    IsTriggerableCallFailed(),
    #[error("Conversion Error")]
    ConversionError(String),
    #[error("Smart Contract Error")]
    SmartContractError(String),
    #[error("Price Error")]
    PriceError(String),
}
