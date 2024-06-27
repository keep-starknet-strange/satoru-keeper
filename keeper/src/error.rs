use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeeperError {
    #[error("Invalid RPC Url")]
    ProviderUrlError(String),
    #[error("RPC_URL not set")]
    RpcUrlNotSet(),
    #[error("PRIVATE_KEY not set")]
    PrivateKeyNotSet(),
    #[error("PUBLIC_KEY not set")]
    PublicKeyNotSet(),
    #[error("Wrong launch params")]
    WrongParam(),
}
