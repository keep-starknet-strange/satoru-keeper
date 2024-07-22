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
    deposit::Deposit, market_created::MarketCreated, order::Order, order_executed::OrderExecuted,
    pool_amount_updated::PoolAmountUpdated, swap_fees_collected::SwapFeesCollected,
    swap_info::SwapInfo, withdrawal::Withdrawal, position::Position,
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
    event_processors.insert(
        OrderExecuted::event_key(),
        Box::new(events::handler::GenericEventProcessor::<OrderExecuted> {
            _marker: std::marker::PhantomData,
        }),
    );
    event_processors.insert(
        Position::event_key(),
        Box::new(events::handler::GenericEventProcessor::<Position> {
            _marker: std::marker::PhantomData,
        }),
    );

    let indexer =
        events::handler::EventIndexer::new(&provider, &pool, event_processors, head_chain);

    let mut current_block = start_block;

    loop {
        let latest_block_on_chain = match provider.block_number().await {
            Ok(block) => block as i64,
            Err(e) => {
                eprintln!("Error fetching latest block number: {:?}", e);
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        };

        if current_block < latest_block_on_chain {
            if let Err(e) = indexer.fetch_and_process_events(current_block as u64).await {
                eprintln!("Error processing pending events: {:?}", e);
            } else {
                current_block += 1;
            }
        }

        sleep(Duration::from_secs(10)).await;
    }
}
