mod config;
mod events;
mod provider;

use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use std::collections::HashMap;

use crate::events::event::Event;
use crate::events::{
    deposit::Deposit, market_created::MarketCreated, order::Order,
    pool_amount_updated::PoolAmountUpdated, swap_fees_collected::SwapFeesCollected,
    swap_info::SwapInfo, withdrawal::Withdrawal,
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&crate::config::get_database_url())
        .await?;

    let provider = provider::get_provider().unwrap();
    let from_block = config::get_from_block();

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

    let indexer = events::handler::EventIndexer::new(&provider, &pool, event_processors);

    loop {
        if let Err(e) = indexer.fetch_and_process_events(from_block).await {
            eprintln!("Error: {:?}", e);
        }
        sleep(Duration::from_secs(10)).await;
    }
}
