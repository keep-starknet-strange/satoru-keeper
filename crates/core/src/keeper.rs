use crate::config::KeeperConfig;
use crate::error::KeeperError;
use starknet::{
    accounts::{Account, Call, ExecutionEncoding, SingleOwnerAccount},
    core::{chain_id, types::FieldElement, utils::get_selector_from_name},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
    signers::{LocalWallet, SigningKey},
};
use url::Url;

use log::info;
/// The Keeper struct is the main entry point for the Keeper service.
#[derive(Debug)]
pub struct Keeper {
    /// The account that the keeper uses to submit transactions.
    /// This account must be whitelisted by the Satoru contracts and have enough funds to pay for the transactions.
    account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    /// The address of the Satoru Exchange Router contract.
    satoru_exchange_router_address: FieldElement,
}

impl Keeper {
    /// Create a new keeper from configuration.
    /// # Arguments
    /// * `config` - The keeper configuration.
    /// # Returns
    /// The keeper.
    /// # Errors
    /// - If the configuration is invalid.
    /// - If the RPC endpoint cannot be reached.
    /// - If the account cannot be created given the signer private key.
    pub async fn new(config: KeeperConfig) -> Result<Self, KeeperError> {
        // Parse the RPC URL.
        let rpc_url = Url::parse(config.rpc_url.as_str())
            .map_err(|e| KeeperError::ConfigError(format!("invalid rpc url: {}", e.to_string())))?;

        // Create the provider to the Starknet node.
        let provider = JsonRpcClient::new(HttpTransport::new(rpc_url));
        let block_number = provider.block_number().await.unwrap();
        info!("lastest block number: {}", block_number);

        // Parse the signer account private key.
        let signer_private_key_field_element =
            FieldElement::from_hex_be(&config.signer_private_key).map_err(|e| {
                KeeperError::ConfigError(format!("invalid private key: {}", e.to_string()))
            })?;

        // Create the signer that has access to the account.
        let signer = LocalWallet::from(SigningKey::from_secret_scalar(
            signer_private_key_field_element,
        ));

        // Parse the account address.
        let account_address = FieldElement::from_hex_be(&config.account_address).map_err(|e| {
            KeeperError::ConfigError(format!("invalid account address: {}", e.to_string()))
        })?;

        // Create the account.
        let account = SingleOwnerAccount::new(
            provider,
            signer,
            account_address,
            chain_id::TESTNET,
            ExecutionEncoding::Legacy,
        );

        // Parse the Satoru Exchange Router address.
        let satoru_exchange_router_address = FieldElement::from_hex_be(
            config.satoru_exchange_router_address.as_str(),
        )
        .map_err(|e| {
            KeeperError::ConfigError(format!(
                "invalid satoru exchange router address: {}",
                e.to_string()
            ))
        })?;
        Ok(Self {
            account,
            satoru_exchange_router_address,
        })
    }

    /// Execute a deposit.
    /// # Arguments
    /// * `deposit_key` - The deposit key.
    pub async fn execute_deposit(&self, deposit_key: &str) -> Result<(), KeeperError> {
        info!("running execute_deposit with key: {}", deposit_key);

        let result = self
            .account
            .execute(vec![Call {
                to: self.satoru_exchange_router_address,
                selector: get_selector_from_name("execute_deposit").unwrap(),
                calldata: vec![
                    // The deposit key.
                    FieldElement::from_hex_be(deposit_key).unwrap(),
                ],
            }])
            .send()
            .await
            // TODO: Handle the error properly.
            .map_err(|e| KeeperError::StarknetTransactionError(e.to_string()))?;

        dbg!(result);
        Ok(())
    }
}
