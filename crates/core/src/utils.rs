//! Utility functions for the core library.
use crate::{error::KeeperError, keepers::KeeperAccount};
use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::{chain_id, types::FieldElement},
    providers::{jsonrpc::HttpTransport, JsonRpcClient},
    signers::{LocalWallet, SigningKey},
};
use url::Url;

/// Create a new account from configuration.
/// # Arguments
/// * `rpc_url` - The RPC URL of the Starknet node.
/// * `signer_private_key` - The private key of the signer that has access to the account.
/// * `account_address` - The address of the account.
/// # Returns
/// The keeper account.
pub fn new_account(
    rpc_url: &str,
    signer_private_key: &str,
    account_address: &str,
) -> Result<KeeperAccount, KeeperError> {
    let rpc_url = Url::parse(rpc_url)
        .map_err(|e| KeeperError::ConfigError(format!("invalid rpc url: {}", e)))?;

    // Create the provider to the Starknet node.
    let provider = new_provider(rpc_url.as_str())?;

    // Parse the signer account private key.
    let signer_private_key_field_element = FieldElement::from_hex_be(signer_private_key)
        .map_err(|e| KeeperError::ConfigError(format!("invalid private key: {}", e)))?;

    // Create the signer that has access to the account.
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        signer_private_key_field_element,
    ));

    // Parse the account address.
    let account_address = FieldElement::from_hex_be(account_address)
        .map_err(|e| KeeperError::ConfigError(format!("invalid account address: {}", e)))?;

    Ok(SingleOwnerAccount::new(
        provider,
        signer,
        account_address,
        chain_id::TESTNET,
        ExecutionEncoding::Legacy,
    ))
}

pub fn new_provider(rpc_url: &str) -> Result<JsonRpcClient<HttpTransport>, KeeperError> {
    // Create the provider to the Starknet node.
    Ok(JsonRpcClient::new(HttpTransport::new(
        Url::parse(rpc_url)
            .map_err(|e| KeeperError::ConfigError(format!("invalid rpc url: {}", e)))?,
    )))
}
