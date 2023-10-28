use starknet::{
    accounts::{Account, Call, SingleOwnerAccount},
    core::{types::FieldElement, utils::get_selector_from_name},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
    signers::LocalWallet,
};

use crate::error::KeeperError;
use crate::keepers::config::CommonKeeperConfig;
use crate::types::set_prices_params::SetPricesParams;
use crate::utils::{new_account, new_provider};

use log::info;
/// The Keeper struct is the main entry point for the Keeper service.
#[derive(Debug)]
pub struct Keeper {
    /// The account that the keeper uses to submit transactions.
    /// This account must be whitelisted by the Satoru contracts and have enough funds to pay for the transactions.
    account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    //provider: JsonRpcClient<HttpTransport>,
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
    pub async fn new(config: CommonKeeperConfig) -> Result<Self, KeeperError> {
        // Create the provider to the Starknet node.
        let provider = new_provider(config.rpc_url.as_str())?;
        let block_number = provider.block_number().await.unwrap();
        info!("lastest block number: {}", block_number);

        // Create the account.
        let account = new_account(
            config.rpc_url.as_str(),
            config.signer_private_key.as_str(),
            config.account_address.as_str(),
        )?;

        // Parse the Satoru Exchange Router address.
        let satoru_exchange_router_address = FieldElement::from_hex_be(
            config.satoru_exchange_router_address.as_str(),
        )
        .map_err(|e| {
            KeeperError::ConfigError(format!("invalid satoru exchange router address: {}", e))
        })?;
        Ok(Self {
            account,
            //provider,
            satoru_exchange_router_address,
        })
    }

    /// Execute a deposit.
    /// # Arguments
    /// * `deposit_key` - The deposit key.
    pub async fn execute_deposit(&self, deposit_key: &str) -> Result<(), KeeperError> {
        info!("running execute_deposit with key: {}", deposit_key);

        let selector = get_selector_from_name("execute_deposit")
            .map_err(|e| KeeperError::ConfigError(e.to_string()))?;

        let result = self
            .account
            .execute(vec![Call {
                to: self.satoru_exchange_router_address,
                selector,
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

    /// Execute a withdrawal.
    /// # Arguments
    /// * `deposit_key` - The deposit key.
    /// * `set_prices_params`.
    pub async fn execute_withdrawal(
        &self,
        deposit_key: &str,
        set_prices_params: &SetPricesParams,
    ) -> Result<(), KeeperError> {
        info!("running execute_deposit with key: {}", deposit_key);

        let selector = get_selector_from_name("execute_withdrawal")
            .map_err(|e| KeeperError::ConfigError(e.to_string()))?;

        let deposit_key = FieldElement::from_hex_be(deposit_key).map_err(|e| {
            KeeperError::ConfigError(format!(
                "could not convert deposit_key '{}' into FieldElement: {}",
                deposit_key, e
            ))
        })?;

        // The deposit key
        let mut calldata: Vec<FieldElement> = vec![deposit_key];
        // The SetPricesParams arguments
        calldata.extend::<Vec<FieldElement>>(set_prices_params.into());

        let result = self
            .account
            .execute(vec![Call {
                to: self.satoru_exchange_router_address,
                selector,
                calldata,
            }])
            .send()
            .await
            // TODO: Handle the error properly.
            .map_err(|e| KeeperError::StarknetTransactionError(e.to_string()))?;

        dbg!(result);
        Ok(())
    }
}
