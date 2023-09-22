#[macro_use]
extern crate log;
use color_eyre::eyre::Result;
use dotenv::dotenv;
use satoru_keeper_core::{keepers::common::Keeper, keepers::config::CommonKeeperConfig};

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

    // Execute the deposit.
    keeper.execute_deposit("0x1234").await?;
    Ok(())
}
