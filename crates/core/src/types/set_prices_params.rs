use serde::Serialize;
use starknet::core::types::{EthAddress, FieldElement};

use crate::types::field_element_utils::IntoFieldElementVec;

#[derive(Debug, Serialize)]
pub struct SetPricesParams {
    pub signer_info: u128,
    pub tokens: Vec<EthAddress>,
    pub compacted_min_oracle_block_numbers: Vec<u64>,
    pub compacted_max_oracle_block_numbers: Vec<u64>,
    pub compacted_oracle_timestamps: Vec<u64>,
    pub compacted_decimals: Vec<u128>,
    pub compacted_min_prices: Vec<u128>,
    pub compacted_min_prices_indexes: Vec<u128>,
    pub compacted_max_prices: Vec<u128>,
    pub compacted_max_prices_indexes: Vec<u128>,
    pub signatures: Vec<FieldElement>,
    pub price_feed_tokens: Vec<EthAddress>,
}

impl From<&SetPricesParams> for Vec<FieldElement> {
    fn from(item: &SetPricesParams) -> Self {
        let mut field_elements = Vec::new();

        field_elements.push(FieldElement::from(item.signer_info));
        field_elements.extend(item.tokens.as_field_element_vec());
        field_elements.extend(
            item.compacted_min_oracle_block_numbers
                .as_field_element_vec(),
        );
        field_elements.extend(
            item.compacted_max_oracle_block_numbers
                .as_field_element_vec(),
        );
        field_elements.extend(item.compacted_oracle_timestamps.as_field_element_vec());
        field_elements.extend(item.compacted_decimals.as_field_element_vec());
        field_elements.extend(item.compacted_min_prices.as_field_element_vec());
        field_elements.extend(item.compacted_min_prices_indexes.as_field_element_vec());
        field_elements.extend(item.compacted_max_prices.as_field_element_vec());
        field_elements.extend(item.compacted_max_prices_indexes.as_field_element_vec());
        field_elements.extend(item.signatures.clone());
        field_elements.extend(item.price_feed_tokens.as_field_element_vec());

        field_elements
    }
}

impl From<SetPricesParams> for Vec<FieldElement> {
    fn from(item: SetPricesParams) -> Self {
        (&item).into()
    }
}
