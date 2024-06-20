use cainome::cairo_serde::ContractAddress;
use starknet::core::types::FieldElement;

pub fn get_token_name_from_address(token_address: ContractAddress) -> String {
    match token_address {
        x if x
            == ContractAddress::from(
                FieldElement::from_hex_be(
                    "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                )
                .expect("Cannot convert string to felt"),
            ) =>
        {
            "eth".to_owned()
        }
        x if x
            == ContractAddress::from(
                FieldElement::from_hex_be(
                    "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
                )
                .expect("Cannot convert string to felt"),
            ) =>
        {
            "usdc".to_owned()
        }
        _ => "".to_owned(),
    }
}
