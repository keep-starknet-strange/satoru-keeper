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
    WithdrawalHandler,
    "./resources/satoru_WithdrawalHandler.contract_class.json"
);

async fn handle_withdrawal(
    account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    withdrawal: SatoruAction,
) {
    let withdrawal_handler_address =
        env::var("WITHDRAWAL_HANDLER").expect("WITHDRAWAL_HANDLER env variable not set");
    let withdrawal_handler = WithdrawalHandler::new(
        FieldElement::from_hex_be(&withdrawal_handler_address)
            .expect("Conversion error: withdrawal_handler_address"),
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

    let tx = withdrawal_handler
        .execute_withdrawal(&FieldElement::from_hex_be(&withdrawal.key).expect("Cannot convert string to felt"), &set_prices_params)
        .send()
        .await
        .expect("Withdrawal Execution Failed");

    // TODO: poll transaction status
}
