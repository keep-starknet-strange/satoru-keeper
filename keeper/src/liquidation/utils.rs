use std::{env, sync::Arc, vec};

use cainome::{cairo_serde::ContractAddress, rs::abigen};
use starknet::{
    accounts::SingleOwnerAccount,
    core::types::FieldElement,
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
    signers::LocalWallet,
};

use super::error::LiquidationError;

abigen!(Reader, "./resources/satoru_Reader.contract_class.json",);

// pub fn is_liquidatable(
//     position: Position,
//     market: Market,
//     prices: MarketPrices,
//     should_validate_min_collateral_usd: bool
// ) -> (bool, String) {
//     let (position_pnl_usd, _, _) = get_position_pnl_usd(market, prices, position, position.size_in_usd);
//     let collateral_token_price = get_cached_token_price(position.collateral_token, market, prices);
//     let collateral_usd = position.collateral_amount * collateral_token_price;

//     let usd_delta_for_price_impact = - i128::try_from(position.size_in_usd).expect("Could not convert position.size_in_usd to integer");
//     let mut price_impact_usd = get_price_impact_usd(market, usd_delta_for_price_impact, position.is_long);

//     let has_positive_impact = price_impact_usd > 0;

//     if price_impact_usd >= 0 {
//         price_impact_usd = 0;
//     } else {
//         let max_price_impact_factor = get_max_position_impact_factor_for_liquidations(
//             market.market_token
//         );

//         let max_negative_price_impact = apply_factor_u256(position.size_in_usd, max_price_impact_factor);
//         if price_impact_usd < max_negative_price_impact {
//             price_impact_usd = max_negative_price_impact;
//         }
//     }
//     let mut position_fees_params = GetPositionFeesParams {
//         position,
//         collateral_token_price,
//         has_positive_impact,
//         long_token,
//         short_token,
//         size_in_usd,
//         0
//     };
//     let fees = get_position_fees(position_fees_params);

//     let collateral_cost_usd = fees.total_cost_amount * collateral_token_price;

//     let remaining_collateral_usd = collateral_usd + position_pnl_usd + price_impact_usd - collateral_cost_usd;

//     if should_validate_min_collateral_usd {
//         if remaining_collateral_usd < MIN_COLLATERAL_USD {
//             return (true, "min collateral".to_owned());
//         }
//     }
//     if remaining_collateral_usd <= 0 {
//         return (true, "0<".to_owned());
//     }

//     let min_collateral_factor = get_min_collateral_factor(market.market_token);

//     let min_collateral_usd_for_leverage = apply_factor_us256(position.size_in_usd, min_collateral_factor);

//     if remaining_collateral_usd <= min_collateral_usd_for_leverage {
//         return (true, "min collateral for leverage".to_owned());
//     }
//     return (false, "".to_owned());
// }

// pub fn get_position_pnl_usd(
//     market: Market_,
//     prices: MarketPrices_,
//     position: Position_,
//     size_delta_usd: u128
// ) -> (i128, i128, u128) {
//     let execution_price = prices.index_token_price.pick_price_for_pnl(position.is_lon, false);

//     let position_value = position.size_in_tokens * execution_price;

//     let total_position_pnl = if position.is_long {
//         position_value - position.size_in_usd
//     } else {
//         position.size_in_usd - position_value
//     };
//     let uncapped_total_position_pnl = total_position_pnl;

//     let mut pnl_token;
//     if total_position_pnl > 0 {
//         pnl_token = if position.is_long {
//                 market.long_token
//             } else {
//                 market.short_token
//             };
//         let pool_token_amount = get_pool_amount(market, pnl_token);
//         let pool_token_price = if position.is_long {
//                 prices.long_token_price
//             } else {
//                 prices.short_token_price
//             };
//         let pool_token_usd = pool_token_amount * pool_token_price;

//         let pool_pnl = get_pnl(market, prices.index_token_price, position.is_long, true);

//         let capped_pool_pnl = get_capped_pnl(market.market_token, position.is_long, pool_pnl, pool_token_usd, MAX_PNL_FACTOR_FOR_TRADERS);
//         if (capped_pool_pnl != pool_pnl
//             && capped_pool_pnl > 0
//             && pool_pnl > 0) {
//             cache
//                 .total_position_pnl =
//                     precision::mul_div_inum(
//                         calc::to_unsigned(total_position_pnl),
//                         capped_pool_pnl,
//                         calc::to_unsigned(pool_pnl)
//                     );
//         }
//     }
//     let mut size_delta_in_tokens;
//     if position.size_in_usd == size_delta_usd {
//         size_delta_in_tokens = position.size_in_tokens;
//     } else {
//         if position.is_long {
//             size_delta_in_tokens =
//                     calc::roundup_division(
//                         position.size_in_tokens * size_delta_usd, position.size_in_usd
//                     );
//         } else {
//             error_utils::check_division_by_zero(position.size_in_usd, "position.size_in_usd");
//             size_delta_in_tokens = position.size_in_tokens
//                 * size_delta_usd
//                 / position.size_in_usd;
//         }
//     }
//     let position_pnl_usd =
//             precision::mul_div_ival(
//                 total_position_pnl, size_delta_in_tokens, position.size_in_tokens
//             );
//     let uncapped_position_pnl_usd =
//             precision::mul_div_ival(
//                 uncapped_total_position_pnl,
//                 size_delta_in_tokens,
//                 position.size_in_tokens
//             );

//     (position_pnl_usd, uncapped_position_pnl_usd, size_delta_in_tokens)
// }

pub async fn is_liquidatable_call(
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
    position: Position,
    market: Market,
    prices: MarketPrices,
    should_validate_min_collateral_usd: bool,
) -> Result<(bool, FieldElement), LiquidationError> {
    let reader_address = env::var("READER").expect("READER env variable not set");
    let reader = Reader::new(
        FieldElement::from_hex_be(&reader_address).expect("Conversion error: reader_address"),
        account.clone(),
    );

    let data_store_address = ContractAddress::from(
        FieldElement::from_hex_be(
            env::var("DATA_STORE")
                .expect("DATA_STORE env var not set")
                .as_str(),
        )
        .expect("Cannot convert string to felt"),
    );

    let referral_storage_address = ContractAddress::from(
        FieldElement::from_hex_be(
            env::var("REFERRAL_STORAGE")
                .expect("DATA_STORE env var not set")
                .as_str(),
        )
        .expect("Cannot convert string to felt"),
    );
    let is_position_liquidatable_result = reader
        .is_position_liquidable(
            &IDataStoreDispatcher {
                contract_address: data_store_address,
            },
            &IReferralStorageDispatcher {
                contract_address: referral_storage_address,
            },
            &position,
            &market,
            &prices,
            &should_validate_min_collateral_usd,
        )
        .call()
        .await;

    match is_position_liquidatable_result {
        Ok(is_position_liquidatable) => return Ok(is_position_liquidatable),
        Err(e) => return Err(LiquidationError::IsLiquidatableCallFailed()),
    };
}
