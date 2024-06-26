mod blockchain;
mod config;
mod events;
mod provider;

use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use starknet::providers::Provider;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

use crate::blockchain::head_chain::HeadChain;
use crate::events::event::Event;
use crate::events::{
    deposit::Deposit, market_created::MarketCreated, order::Order,
    pool_amount_updated::PoolAmountUpdated, swap_fees_collected::SwapFeesCollected,
    swap_info::SwapInfo, withdrawal::Withdrawal,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&crate::config::get_database_url())
        .await?;

    let provider = provider::get_provider().unwrap();

    let head_chain = HeadChain::new(pool.clone());
    let last_block_indexed = head_chain.get_last_block_indexed().await?;
    let latest_block_on_chain = provider
        .block_number()
        .await
        .map_err(|e| sqlx::Error::Protocol(format!("{:?}", e)))?;

    let start_block = if last_block_indexed > 0 {
        last_block_indexed + 1
    } else {
        config::get_from_block() as i64
    };

    let mut event_processors: HashMap<
        &'static str,
        Box<dyn events::handler::EventProcessor + Send + Sync>,
    > = HashMap::new();
    event_processors.insert(
        Order::event_key(),
        Box::new(events::handler::GenericEventProcessor::<Order> {
            _marker: std::marker::PhantomData,
        }),
    );
    event_processors.insert(
        Deposit::event_key(),
        Box::new(events::handler::GenericEventProcessor::<Deposit> {
            _marker: std::marker::PhantomData,
        }),
    );
    event_processors.insert(
        Withdrawal::event_key(),
        Box::new(events::handler::GenericEventProcessor::<Withdrawal> {
            _marker: std::marker::PhantomData,
        }),
    );
    event_processors.insert(
        MarketCreated::event_key(),
        Box::new(events::handler::GenericEventProcessor::<MarketCreated> {
            _marker: std::marker::PhantomData,
        }),
    );
    event_processors.insert(
        SwapFeesCollected::event_key(),
        Box::new(
            events::handler::GenericEventProcessor::<SwapFeesCollected> {
                _marker: std::marker::PhantomData,
            },
        ),
    );
    event_processors.insert(
        SwapInfo::event_key(),
        Box::new(events::handler::GenericEventProcessor::<SwapInfo> {
            _marker: std::marker::PhantomData,
        }),
    );
    event_processors.insert(
        PoolAmountUpdated::event_key(),
        Box::new(
            events::handler::GenericEventProcessor::<PoolAmountUpdated> {
                _marker: std::marker::PhantomData,
            },
        ),
    );

    let indexer =
        events::handler::EventIndexer::new(&provider, &pool, event_processors, head_chain);

    if start_block <= latest_block_on_chain as i64 {
        if let Err(e) = indexer.fetch_and_process_events(start_block as u64).await {
            eprintln!("Error fetching and processing events: {:?}", e);
        } else {
            println!("Initial fetch and process completed successfully.");
        }
    }

    loop {
        if let Err(e) = indexer.fetch_pending_events().await {
            eprintln!("Error processing pending events: {:?}", e);
        }
        sleep(Duration::from_secs(10)).await;
    }
}
