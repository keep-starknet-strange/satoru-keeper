use std::{env, vec};

use cainome::{
    cairo_serde::{ContractAddress, U256},
    rs::abigen,
};
use starknet::{
    accounts::SingleOwnerAccount,
    core::types::FieldElement,
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    signers::LocalWallet,
};

use crate::types::SatoruAction;

abigen!(
    DepositHandler,
    "./resources/satoru_DepositHandler.contract_class.json"
);

async fn handle_deposit(
    account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    deposit: SatoruAction,
) {
    let deposit_handler_address =
        env::var("DEPOSIT_HANDLER").expect("DEPOSIT_HANDLER env variable not set");
    let deposit_handler = DepositHandler::new(
        FieldElement::from_hex_be(&deposit_handler_address)
            .expect("Conversion error: deposit_handler_address"),
        account,
    );

    let set_prices_params: SetPricesParams = SetPricesParams {
        signer_info: U256 { low: 1, high: 0 },
        tokens: vec![
            ContractAddress::from(
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
            ),
            ContractAddress::from(
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
            ),
        ],
        compacted_min_oracle_block_numbers: vec![63970, 63970],
        compacted_max_oracle_block_numbers: vec![64901, 64901],
        compacted_oracle_timestamps: vec![171119803, 10],
        compacted_decimals: vec![U256 { low: 1, high: 0 }, U256 { low: 1, high: 0 }],
        compacted_min_prices: vec![U256 {
            low: 2147483648010000,
            high: 0,
        }],
        compacted_min_prices_indexes: vec![U256 { low: 0, high: 0 }],
        compacted_max_prices: vec![U256 {
            low: 2147483648010000,
            high: 0,
        }],
        compacted_max_prices_indexes: vec![U256 { low: 0, high: 0 }],
        signatures: vec![
            vec![
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
            ],
            vec![
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
                FieldElement::from_hex_be("0x").expect("Cannot convert string to felt"),
            ],
        ],
        price_feed_tokens: vec![],
    };

    let tx = deposit_handler
        .execute_deposit(&FieldElement::from_hex_be(&deposit.key).expect("Cannot convert string to felt"), &set_prices_params)
        .send()
        .await
        .expect("Deposit Execution Failed");

    // TODO: poll transaction status
}
