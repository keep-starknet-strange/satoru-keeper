use thiserror::Error;

#[derive(Error, Debug)]
pub enum LiquidationError {
    #[error("Failed to call is_liquidatable")]
    IsLiquidatableCallFailed(),
}
