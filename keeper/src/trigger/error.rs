use thiserror::Error;

#[derive(Error, Debug)]
pub enum TriggerError {
    #[error("Failed to call is_triggerable")]
    IsTriggerableCallFailed(),
}
