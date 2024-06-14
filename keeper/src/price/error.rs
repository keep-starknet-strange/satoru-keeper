use thiserror::Error;

#[derive(Error, Debug)]
pub enum PragmaAPIError {
    #[error("Pragma price response could not get parsed to PriceInfo")]
    JsonParsing(#[from] reqwest::Error),
    #[error("Unauthorized Access: Pragma API")]
    UnauthorizedAccess(),
    #[error("Unknown Pragma API error")]
    Unknown(String),
    #[error("Could not fetch Pragma price")]
    FetchError(Box<PragmaAPIError>),
    #[error("API Key not set")]
    APIKeyNotSet(),
}
