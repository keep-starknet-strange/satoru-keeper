#[macro_use]
extern crate log;
use color_eyre::eyre::Result;
use dotenv::dotenv;
use satoru_keeper_core::{
    keepers::common::Keeper, keepers::config::CommonKeeperConfig,
    types::set_prices_params::SetPricesParams,
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // Initialize the logger.
    env_logger::init();

    // Load the environment variables from the .env file.
    dotenv().ok();

    info!("starting keeper service...");

    // Load the keeper configuration from the environment variables.
    let config = CommonKeeperConfig::load_from_config()?;

    // Create the keeper.
    let keeper = Keeper::new(config).await?;

    // Set the prices.
    let oracle_params = SetPricesParams {
        signer_info: 0,
        tokens: vec![],
        compacted_min_oracle_block_numbers: vec![],
        compacted_max_oracle_block_numbers: vec![],
        compacted_oracle_timestamps: vec![],
        compacted_decimals: vec![],
        compacted_min_prices: vec![],
        compacted_min_prices_indexes: vec![],
        compacted_max_prices: vec![],
        compacted_max_prices_indexes: vec![],
        signatures: vec![],
        price_feed_tokens: vec![],
    };

    // Execute the deposit.
    keeper.execute_deposit("0x1234", &oracle_params).await?;
    Ok(())
}
