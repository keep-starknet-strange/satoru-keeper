//! Configuration for the keeper.
use crate::error::KeeperError;
use config::Config;
use serde_derive::Deserialize;

#[derive(Default, Builder, Debug, Deserialize)]
#[builder(setter(into))]
pub struct KeeperConfig {
    pub rpc_url: String,
    pub signer_private_key: String,
    pub account_address: String,
    pub satoru_exchange_router_address: String,
}

impl KeeperConfig {
    /// Load the keeper configuration from the environment variables.
    /// # Returns
    /// The keeper configuration.
    /// # Errors
    /// - If the configuration is invalid.
    pub fn load_from_config() -> Result<KeeperConfig, KeeperError> {
        CONFIG.clone().try_deserialize().map_err(|e| e.into())
    }
}

lazy_static::lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = Config::builder()
        .add_source(config::Environment::with_prefix("keeper"))
        .build()
        .unwrap();
}
