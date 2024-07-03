use crate::types::{Position, Market, MarketPrices};

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

pub fn is_liquidatable_call() {
    
}