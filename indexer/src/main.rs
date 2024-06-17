mod config;
mod database;
mod provider;
mod events;

use dotenv::dotenv;
use tokio_postgres::Error;
use events::handler::Indexer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let (client, connection) = database::connect().await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {:?}", e);
        }
    });

    let provider = provider::get_provider().unwrap();

    let indexer = Indexer::new(&provider, &client);

    if let Err(e) = indexer.fetch_and_process_events().await {
        eprintln!("Error: {:?}", e);
    }

    Ok(())
}
