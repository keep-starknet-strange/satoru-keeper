use serde::Serialize;
use starknet::core::types::FieldElement;

use crate::types::field_element::{FieldElementVecExt, IntoFieldElementVec};

#[derive(Debug, Serialize)]
pub struct SetPricesParams {
    pub signer_info: u128,
    pub tokens: Vec<FieldElement>,
    pub compacted_min_oracle_block_numbers: Vec<u64>,
    pub compacted_max_oracle_block_numbers: Vec<u64>,
    pub compacted_oracle_timestamps: Vec<u64>,
    pub compacted_decimals: Vec<u128>,
    pub compacted_min_prices: Vec<u128>,
    pub compacted_min_prices_indexes: Vec<u128>,
    pub compacted_max_prices: Vec<u128>,
    pub compacted_max_prices_indexes: Vec<u128>,
    pub signatures: Vec<FieldElement>,
    pub price_feed_tokens: Vec<FieldElement>,
}

impl From<&SetPricesParams> for Vec<FieldElement> {
    fn from(item: &SetPricesParams) -> Self {
        let mut field_elements = Vec::new();

        field_elements.push(FieldElement::from(item.signer_info));

        field_elements.extend_with_len(&item.tokens.as_field_element_vec());
        field_elements.extend_with_len(
            &item
                .compacted_min_oracle_block_numbers
                .as_field_element_vec(),
        );
        field_elements.extend_with_len(
            &item
                .compacted_max_oracle_block_numbers
                .as_field_element_vec(),
        );
        field_elements.extend_with_len(&item.compacted_oracle_timestamps.as_field_element_vec());
        field_elements.extend_with_len(&item.compacted_decimals.as_field_element_vec());
        field_elements.extend_with_len(&item.compacted_min_prices.as_field_element_vec());
        field_elements.extend_with_len(&item.compacted_min_prices_indexes.as_field_element_vec());
        field_elements.extend_with_len(&item.compacted_max_prices.as_field_element_vec());
        field_elements.extend_with_len(&item.compacted_max_prices_indexes.as_field_element_vec());
        field_elements.extend_with_len(&item.signatures.as_field_element_vec());
        field_elements.extend_with_len(&item.price_feed_tokens.as_field_element_vec());

        field_elements
    }
}

impl From<SetPricesParams> for Vec<FieldElement> {
    fn from(item: SetPricesParams) -> Self {
        (&item).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use starknet::core::types::FieldElement;

    #[test]
    fn test_set_prices_params_to_vec_field_elements() {
        let tokens = vec![FieldElement::from(90_u8), FieldElement::from(91_u8)];
        let compacted_min_oracle_block_numbers = vec![1, 2, 3];
        let compacted_max_oracle_block_numbers = vec![4, 5, 6];
        let compacted_oracle_timestamps = vec![7, 8, 9];
        let compacted_decimals = vec![10, 11, 12];
        let compacted_min_prices = vec![13, 14, 15];
        let compacted_min_prices_indexes = vec![16, 17, 18];
        let compacted_max_prices = vec![19, 20, 21];
        let compacted_max_prices_indexes = vec![22, 23, 24];
        let signatures = vec![FieldElement::from(25_u8), FieldElement::from(26_u8)];
        let price_feed_tokens = vec![FieldElement::from(33_u8), FieldElement::from(34_u8)];

        let set_prices_params = SetPricesParams {
            signer_info: 1,
            tokens,
            compacted_min_oracle_block_numbers,
            compacted_max_oracle_block_numbers,
            compacted_oracle_timestamps,
            compacted_decimals,
            compacted_min_prices,
            compacted_min_prices_indexes,
            compacted_max_prices,
            compacted_max_prices_indexes,
            signatures,
            price_feed_tokens,
        };

        let expected = vec![
            // tokens
            FieldElement::from(1_u8),
            // compacted_min_oracle_block_numbers
            FieldElement::from(2_u8),
            FieldElement::from(90_u8),
            FieldElement::from(91_u8),
            // compacted_min_oracle_block_numbers
            FieldElement::from(3_u8),
            FieldElement::from(1_u8),
            FieldElement::from(2_u8),
            FieldElement::from(3_u8),
            // compacted_max_oracle_block_numbers
            FieldElement::from(3_u8),
            FieldElement::from(4_u8),
            FieldElement::from(5_u8),
            FieldElement::from(6_u8),
            // compacted_oracle_timestamps
            FieldElement::from(3_u8),
            FieldElement::from(7_u8),
            FieldElement::from(8_u8),
            FieldElement::from(9_u8),
            // compacted_decimals
            FieldElement::from(3_u8),
            FieldElement::from(10_u8),
            FieldElement::from(11_u8),
            FieldElement::from(12_u8),
            // compacted_min_prices
            FieldElement::from(3_u8),
            FieldElement::from(13_u8),
            FieldElement::from(14_u8),
            FieldElement::from(15_u8),
            // compacted_min_prices_indexes
            FieldElement::from(3_u8),
            FieldElement::from(16_u8),
            FieldElement::from(17_u8),
            FieldElement::from(18_u8),
            // compacted_max_prices
            FieldElement::from(3_u8),
            FieldElement::from(19_u8),
            FieldElement::from(20_u8),
            FieldElement::from(21_u8),
            // compacted_max_prices_indexes
            FieldElement::from(3_u8),
            FieldElement::from(22_u8),
            FieldElement::from(23_u8),
            FieldElement::from(24_u8),
            // signatures
            FieldElement::from(2_u8),
            FieldElement::from(25_u8),
            FieldElement::from(26_u8),
            // price_feed_tokens
            FieldElement::from(2_u8),
            FieldElement::from(33_u8),
            FieldElement::from(34_u8),
        ];
        let out: Vec<FieldElement> = set_prices_params.into();

        assert_eq!(out, expected);
    }
}
